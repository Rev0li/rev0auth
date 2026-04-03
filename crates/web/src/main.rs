mod pages;
mod styles;

use axum::{
    extract::{Multipart, Path, Query, State},
    routing::{get, post, put, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::{atomic::AtomicU64, atomic::Ordering, Arc};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tokio::time::{timeout, Duration};
use tracing::info;

#[derive(Clone)]
struct WebState {
    signup_requests: Arc<RwLock<Vec<SignupRequestRecord>>>,
    next_request_id: Arc<AtomicU64>,
    users: Arc<RwLock<Vec<User>>>,
    user_passwords: Arc<RwLock<std::collections::HashMap<String, String>>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum UserStatus {
    Actif,
    Occupe,
    Inactif,
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserStatus::Actif => write!(f, "actif"),
            UserStatus::Occupe => write!(f, "occupe"),
            UserStatus::Inactif => write!(f, "inactif"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct User {
    pseudo: String,
    role: &'static str,
    active: bool,
    status: UserStatus,
    bio: String,
    avatar_filename: Option<String>,
    avatar_size_bytes: Option<usize>,
    created_at_epoch: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum ManualStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Serialize)]
struct SignupRequestRecord {
    id: u64,
    pseudo: String,
    reason: String,
    referral: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact: Option<String>,
    status: ManualStatus,
    created_at_epoch: u64,
}

#[derive(Debug, Deserialize)]
struct SignupRequestInput {
    pseudo: String,
    reason: String,
    referral: String,
    #[serde(default)]
    contact: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LoginInput {
    pseudo: String,
}

#[derive(Debug, Deserialize)]
struct PasswordCheckInput {
    pseudo: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct AdminSetPasswordInput {
    password: String,
}

#[derive(Debug, Deserialize)]
struct CreateUserInput {
    pseudo: String,
}

#[derive(Debug, Deserialize)]
struct UpdateUserInput {
    status: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ProfileQuery {
    pseudo: String,
}

#[derive(Debug, Deserialize)]
struct UpdateProfileInput {
    pseudo: String,
    bio: String,
}

#[derive(Debug, Serialize)]
struct PasswordCheckResponse {
    ok: bool,
    message: &'static str,
}

#[derive(Debug, Serialize)]
struct SignupResponse {
    ok: bool,
    request_id: u64,
    status: &'static str,
    message: &'static str,
}

#[derive(Debug, Serialize)]
struct ActionResponse {
    ok: bool,
    message: &'static str,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    ok: bool,
    state: &'static str,
    message: &'static str,
}

#[derive(Debug, Serialize)]
struct MemberProfileResponse {
    ok: bool,
    pseudo: String,
    role: &'static str,
    status: UserStatus,
    bio: String,
    avatar_filename: Option<String>,
    avatar_size_bytes: Option<usize>,
}

#[derive(Debug, Serialize)]
struct MessageResponse {
    ok: bool,
    message: String,
}

#[derive(Serialize)]
struct StatusResponse {
    admin_ok: bool,
    user_ok: bool,
    api_ok: bool,
    checked_at_epoch: u64,
}

#[derive(Serialize)]
struct PingResponse {
    side: &'static str,
    status: &'static str,
}

#[derive(Serialize)]
struct StatusAllResponse {
    checked_at_epoch: u64,
    admin_ok: bool,
    user_ok: bool,
    api_ok: bool,
    web_ok: bool,
    sprint: &'static str,
    tests_api_total: u32,
    signup_requests_pending: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_target(false)
        .compact()
        .init();

    let state = WebState {
        signup_requests: Arc::new(RwLock::new(Vec::new())),
        next_request_id: Arc::new(AtomicU64::new(1)),
        users: Arc::new(RwLock::new(Vec::new())),
        user_passwords: Arc::new(RwLock::new(std::collections::HashMap::new())),
    };

    let app = Router::new()
        .route("/", get(home))
        .route("/portal", get(portal))
        .route("/portal/signup-request", post(portal_signup_request))
        .route("/portal/login", post(portal_login))
        .route("/dashboard", get(dashboard))
        .route("/status", get(status))
        .route("/status/all", get(status_all))
        .route("/admin/signup-requests", get(admin_signup_requests))
        .route(
            "/admin/signup-requests/:id/approve",
            post(admin_approve_signup_request),
        )
        .route(
            "/admin/signup-requests/:id/reject",
            post(admin_reject_signup_request),
        )
        .route("/admin/ping", get(admin_ping))
        .route("/user/ping", get(user_ping))
        .route("/admin/set-password/:pseudo", post(admin_set_password))
        .route("/admin/remove-password/:pseudo", post(admin_remove_password))
        .route("/users", get(list_users))
        .route("/admin/users", post(admin_create_user))
        .route("/admin/users/:pseudo", put(admin_update_user))
        .route("/admin/users/:pseudo", delete(admin_delete_user))
        .route("/home/friend", get(friend_home))
        .route("/members/dashboard", get(members_dashboard))
        .route("/members/profile", get(members_profile_page))
        .route("/members/profile/data", get(member_profile_data))
        .route("/members/profile/data", put(member_update_profile))
        .route("/members/avatar", post(member_upload_avatar))
        .route("/auth/password-check", post(password_check))
        .route("/status/set-busy/:pseudo", post(set_user_busy))
        .route("/status/set-active/:pseudo", post(set_user_active))
        .route("/status/set-inactive/:pseudo", post(set_user_inactive))
        .with_state(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    info!(%addr, "Web app listening");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

// ============================================================================
// PAGE HANDLERS - Delegated to modules
// ============================================================================

async fn home() -> impl axum::response::IntoResponse {
    pages::home().await
}

async fn portal() -> impl axum::response::IntoResponse {
    pages::portal().await
}

async fn dashboard() -> impl axum::response::IntoResponse {
    pages::dashboard().await
}

async fn friend_home() -> impl axum::response::IntoResponse {
    pages::friend().await
}

async fn members_dashboard() -> impl axum::response::IntoResponse {
    pages::friend().await
}

async fn members_profile_page() -> impl axum::response::IntoResponse {
    pages::profile().await
}

// ============================================================================
// API HANDLERS - Status & Ping
// ============================================================================

async fn status() -> Json<StatusResponse> {
    let api_ok = timeout(
        Duration::from_millis(500),
        tokio::net::TcpStream::connect("127.0.0.1:8080"),
    )
    .await
    .map(|r| r.is_ok())
    .unwrap_or(false);

    Json(StatusResponse {
        admin_ok: true,
        user_ok: true,
        api_ok,
        checked_at_epoch: now_epoch(),
    })
}

async fn admin_ping() -> Json<PingResponse> {
    Json(PingResponse {
        side: "admin",
        status: "ok",
    })
}

async fn user_ping() -> Json<PingResponse> {
    Json(PingResponse {
        side: "user",
        status: "ok",
    })
}

async fn status_all(State(state): State<WebState>) -> Json<StatusAllResponse> {
    let api_ok = timeout(
        Duration::from_millis(500),
        tokio::net::TcpStream::connect("127.0.0.1:8080"),
    )
    .await
    .map(|r| r.is_ok())
    .unwrap_or(false);

    let requests = state.signup_requests.read().await;
    let signup_requests_pending = requests
        .iter()
        .filter(|r| r.status == ManualStatus::Pending)
        .count();

    Json(StatusAllResponse {
        checked_at_epoch: now_epoch(),
        admin_ok: true,
        user_ok: true,
        api_ok,
        web_ok: true,
        sprint: "AUTH-006",
        tests_api_total: 18,
        signup_requests_pending,
    })
}

// ============================================================================
// PORTAL HANDLERS - Signup & Login
// ============================================================================

async fn portal_signup_request(
    State(state): State<WebState>,
    Json(payload): Json<SignupRequestInput>,
) -> Json<SignupResponse> {
    if payload.pseudo.trim().is_empty()
        || payload.reason.trim().is_empty()
        || payload.referral.trim().is_empty()
    {
        return Json(SignupResponse {
            ok: false,
            request_id: 0,
            status: "rejected",
            message: "Champs invalides: remplis pseudo, raison et referral.",
        });
    }

    let id = state.next_request_id.fetch_add(1, Ordering::Relaxed);
    let request = SignupRequestRecord {
        id,
        pseudo: payload.pseudo.trim().to_string(),
        reason: payload.reason.trim().to_string(),
        referral: payload.referral.trim().to_string(),
        contact: payload.contact.map(|c| c.trim().to_string()).filter(|c| !c.is_empty()),
        status: ManualStatus::Pending,
        created_at_epoch: now_epoch(),
    };

    let mut requests = state.signup_requests.write().await;
    requests.push(request);

    Json(SignupResponse {
        ok: true,
        request_id: id,
        status: "pending",
        message: "Demande envoyee. En attente de validation manuelle admin.",
    })
}

async fn portal_login(
    State(state): State<WebState>,
    Json(payload): Json<LoginInput>,
) -> Json<LoginResponse> {
    let pseudo = payload.pseudo.trim();
    if pseudo.is_empty() {
        return Json(LoginResponse {
            ok: false,
            state: "invalid",
            message: "Pseudo requis.",
        });
    }

    let requests = state.signup_requests.read().await;
    let maybe = requests
        .iter()
        .rev()
        .find(|r| r.pseudo.eq_ignore_ascii_case(pseudo));

    match maybe {
        Some(r) if r.status == ManualStatus::Approved => Json(LoginResponse {
            ok: true,
            state: "approved",
            message: "Connexion autorisee: ton compte est valide.",
        }),
        Some(r) if r.status == ManualStatus::Pending => Json(LoginResponse {
            ok: false,
            state: "pending",
            message: "Compte en attente de validation admin.",
        }),
        Some(_) => Json(LoginResponse {
            ok: false,
            state: "rejected",
            message: "Demande refusee. Contacte l'administrateur.",
        }),
        None => Json(LoginResponse {
            ok: false,
            state: "missing",
            message: "Aucune demande trouvee pour ce pseudo.",
        }),
    }
}

// ============================================================================
// ADMIN HANDLERS - Signup Request Queue
// ============================================================================

async fn admin_signup_requests(State(state): State<WebState>) -> Json<Vec<SignupRequestRecord>> {
    let requests = state.signup_requests.read().await;
    Json(requests.clone())
}

async fn admin_approve_signup_request(
    Path(id): Path<u64>,
    State(state): State<WebState>,
) -> Json<ActionResponse> {
    let mut requests = state.signup_requests.write().await;
    let maybe = requests.iter_mut().find(|r| r.id == id);

    if let Some(req) = maybe {
        req.status = ManualStatus::Approved;
        
        // Add user to the users list
        info!(target: "rev0auth", "User approved: {}", req.pseudo);
        let user = User {
            pseudo: req.pseudo.clone(),
            role: "member",
            active: true,
            status: UserStatus::Actif,
            bio: String::new(),
            avatar_filename: None,
            avatar_size_bytes: None,
            created_at_epoch: now_epoch(),
        };
        let mut users = state.users.write().await;
        users.push(user);
        
        return Json(ActionResponse {
            ok: true,
            message: "Demande approuvee.",
        });
    }

    Json(ActionResponse {
        ok: false,
        message: "Demande introuvable.",
    })
}

async fn admin_reject_signup_request(
    Path(id): Path<u64>,
    State(state): State<WebState>,
) -> Json<ActionResponse> {
    let mut requests = state.signup_requests.write().await;
    let maybe = requests.iter_mut().find(|r| r.id == id);

    if let Some(req) = maybe {
        req.status = ManualStatus::Rejected;
        return Json(ActionResponse {
            ok: true,
            message: "Demande rejetee.",
        });
    }

    Json(ActionResponse {
        ok: false,
        message: "Demande introuvable.",
    })
}

// ============================================================================
// USER HANDLERS - List users
// ============================================================================

async fn list_users(State(state): State<WebState>) -> Json<Vec<User>> {
    let users = state.users.read().await;
    Json(users.clone())
}

async fn password_check(
    State(state): State<WebState>,
    Json(payload): Json<PasswordCheckInput>,
) -> Json<PasswordCheckResponse> {
    let passwords = state.user_passwords.read().await;
    let stored_password = passwords.get(&payload.pseudo);

    match stored_password {
        Some(pwd) if pwd == &payload.password => Json(PasswordCheckResponse {
            ok: true,
            message: "Mot de passe correct. Connexion autorisee.",
        }),
        Some(_) => Json(PasswordCheckResponse {
            ok: false,
            message: "Mot de passe incorrect.",
        }),
        None => {
            drop(passwords);
            
            // Auto-create password on first login (for testing)
            let mut passwords = state.user_passwords.write().await;
            passwords.insert(payload.pseudo.clone(), payload.password.clone());
            
            Json(PasswordCheckResponse {
                ok: true,
                message: "Mot de passe enregistre. Bienvenue !",
            })
        }
    }
}

async fn set_user_busy(
    Path(pseudo): Path<String>,
    State(state): State<WebState>,
) -> Json<ActionResponse> {
    let mut users = state.users.write().await;
    
    if let Some(user) = users.iter_mut().find(|u| u.pseudo == pseudo) {
        user.status = UserStatus::Occupe;
        info!(target: "rev0auth", "User {} set to busy", pseudo);
        return Json(ActionResponse {
            ok: true,
            message: "Statut change en occupe.",
        });
    }

    Json(ActionResponse {
        ok: false,
        message: "Utilisateur introuvable.",
    })
}

async fn set_user_active(
    Path(pseudo): Path<String>,
    State(state): State<WebState>,
) -> Json<ActionResponse> {
    let mut users = state.users.write().await;
    
    if let Some(user) = users.iter_mut().find(|u| u.pseudo == pseudo) {
        user.status = UserStatus::Actif;
        info!(target: "rev0auth", "User {} set to active", pseudo);
        return Json(ActionResponse {
            ok: true,
            message: "Statut change en actif.",
        });
    }

    Json(ActionResponse {
        ok: false,
        message: "Utilisateur introuvable.",
    })
}

async fn set_user_inactive(
    Path(pseudo): Path<String>,
    State(state): State<WebState>,
) -> Json<ActionResponse> {
    let mut users = state.users.write().await;
    
    if let Some(user) = users.iter_mut().find(|u| u.pseudo == pseudo) {
        user.status = UserStatus::Inactif;
        info!(target: "rev0auth", "User {} logged out", pseudo);
        return Json(ActionResponse {
            ok: true,
            message: "Statut change en inactif.",
        });
    }

    Json(ActionResponse {
        ok: false,
        message: "Utilisateur introuvable.",
    })
}

async fn admin_set_password(
    Path(pseudo): Path<String>,
    State(state): State<WebState>,
    Json(payload): Json<AdminSetPasswordInput>,
) -> Json<ActionResponse> {
    let users = state.users.read().await;
    
    // Verify user exists
    if !users.iter().any(|u| u.pseudo == pseudo) {
        return Json(ActionResponse {
            ok: false,
            message: "Utilisateur introuvable.",
        });
    }

    drop(users);
    
    // Set password
    let mut passwords = state.user_passwords.write().await;
    passwords.insert(pseudo.clone(), payload.password);
    
    info!(target: "rev0auth", "Admin set password for user {}", pseudo);
    Json(ActionResponse {
        ok: true,
        message: "Mot de passe defini.",
    })
}

async fn admin_remove_password(
    Path(pseudo): Path<String>,
    State(state): State<WebState>,
) -> Json<ActionResponse> {
    let users = state.users.read().await;
    
    // Verify user exists
    if !users.iter().any(|u| u.pseudo == pseudo) {
        return Json(ActionResponse {
            ok: false,
            message: "Utilisateur introuvable.",
        });
    }

    drop(users);
    
    // Remove password
    let mut passwords = state.user_passwords.write().await;
    if passwords.remove(&pseudo).is_some() {
        info!(target: "rev0auth", "Admin removed password for user {}", pseudo);
        Json(ActionResponse {
            ok: true,
            message: "Mot de passe supprime.",
        })
    } else {
        Json(ActionResponse {
            ok: false,
            message: "Pas de mot de passe pour cet utilisateur.",
        })
    }
}

async fn admin_create_user(
    State(state): State<WebState>,
    Json(payload): Json<CreateUserInput>,
) -> Json<ActionResponse> {
    let mut users = state.users.write().await;
    
    // Check if user already exists
    if users.iter().any(|u| u.pseudo == payload.pseudo) {
        return Json(ActionResponse {
            ok: false,
            message: "Utilisateur existe deja.",
        });
    }
    
    let user = User {
        pseudo: payload.pseudo.clone(),
        role: "member",
        active: true,
        status: UserStatus::Actif,
        bio: String::new(),
        avatar_filename: None,
        avatar_size_bytes: None,
        created_at_epoch: now_epoch(),
    };
    
    users.push(user);
    info!(target: "rev0auth", "Admin created user {}", payload.pseudo);
    
    Json(ActionResponse {
        ok: true,
        message: "Utilisateur cree.",
    })
}

async fn admin_update_user(
    Path(pseudo): Path<String>,
    State(state): State<WebState>,
    Json(payload): Json<UpdateUserInput>,
) -> Json<ActionResponse> {
    let mut users = state.users.write().await;
    
    if let Some(user) = users.iter_mut().find(|u| u.pseudo == pseudo) {
        if let Some(status_str) = payload.status {
            let new_status = match status_str.to_lowercase().as_str() {
                "actif" => UserStatus::Actif,
                "occupe" => UserStatus::Occupe,
                "inactif" => UserStatus::Inactif,
                _ => return Json(ActionResponse {
                    ok: false,
                    message: "Statut invalide.",
                }),
            };
            user.status = new_status;
        }
        
        info!(target: "rev0auth", "Admin updated user {}", pseudo);
        return Json(ActionResponse {
            ok: true,
            message: "Utilisateur modifie.",
        });
    }

    Json(ActionResponse {
        ok: false,
        message: "Utilisateur introuvable.",
    })
}

async fn admin_delete_user(
    Path(pseudo): Path<String>,
    State(state): State<WebState>,
) -> Json<ActionResponse> {
    let mut users = state.users.write().await;
    let initial_len = users.len();
    users.retain(|u| u.pseudo != pseudo);
    
    if users.len() < initial_len {
        // Also remove password if exists
        let mut passwords = state.user_passwords.write().await;
        passwords.remove(&pseudo);
        
        info!(target: "rev0auth", "Admin deleted user {}", pseudo);
        Json(ActionResponse {
            ok: true,
            message: "Utilisateur supprime.",
        })
    } else {
        Json(ActionResponse {
            ok: false,
            message: "Utilisateur introuvable.",
        })
    }
}

async fn member_profile_data(
    Query(query): Query<ProfileQuery>,
    State(state): State<WebState>,
) -> Json<MemberProfileResponse> {
    let users = state.users.read().await;
    if let Some(user) = users.iter().find(|u| u.pseudo.eq_ignore_ascii_case(&query.pseudo)) {
        return Json(MemberProfileResponse {
            ok: true,
            pseudo: user.pseudo.clone(),
            role: user.role,
            status: user.status,
            bio: user.bio.clone(),
            avatar_filename: user.avatar_filename.clone(),
            avatar_size_bytes: user.avatar_size_bytes,
        });
    }

    Json(MemberProfileResponse {
        ok: false,
        pseudo: query.pseudo,
        role: "member",
        status: UserStatus::Inactif,
        bio: String::new(),
        avatar_filename: None,
        avatar_size_bytes: None,
    })
}

async fn member_update_profile(
    State(state): State<WebState>,
    Json(payload): Json<UpdateProfileInput>,
) -> Json<MessageResponse> {
    let mut users = state.users.write().await;
    if let Some(user) = users
        .iter_mut()
        .find(|u| u.pseudo.eq_ignore_ascii_case(&payload.pseudo))
    {
        user.bio = payload.bio.trim().to_string();
        return Json(MessageResponse {
            ok: true,
            message: "Profil mis a jour.".to_string(),
        });
    }

    Json(MessageResponse {
        ok: false,
        message: "Utilisateur introuvable.".to_string(),
    })
}

async fn member_upload_avatar(
    State(state): State<WebState>,
    mut multipart: Multipart,
) -> Json<MessageResponse> {
    let mut pseudo: Option<String> = None;
    let mut avatar_filename: Option<String> = None;
    let mut avatar_size_bytes: Option<usize> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap_or_default().to_string();
        if field_name == "pseudo" {
            if let Ok(text) = field.text().await {
                pseudo = Some(text);
            }
            continue;
        }

        if field_name == "avatar" {
            avatar_filename = field.file_name().map(|s| s.to_string());
            if let Ok(bytes) = field.bytes().await {
                avatar_size_bytes = Some(bytes.len());
            }
        }
    }

    let Some(member_pseudo) = pseudo else {
        return Json(MessageResponse {
            ok: false,
            message: "Pseudo manquant dans le formulaire.".to_string(),
        });
    };

    if avatar_size_bytes.is_none() {
        return Json(MessageResponse {
            ok: false,
            message: "Fichier avatar manquant.".to_string(),
        });
    }

    let mut users = state.users.write().await;
    if let Some(user) = users
        .iter_mut()
        .find(|u| u.pseudo.eq_ignore_ascii_case(&member_pseudo))
    {
        user.avatar_filename = avatar_filename;
        user.avatar_size_bytes = avatar_size_bytes;
        return Json(MessageResponse {
            ok: true,
            message: "Avatar upload recu.".to_string(),
        });
    }

    Json(MessageResponse {
        ok: false,
        message: "Utilisateur introuvable.".to_string(),
    })
}

// ============================================================================
// UTILITIES
// ============================================================================

fn now_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
