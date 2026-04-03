mod pages;
mod styles;

use axum::{
    body::Body,
    extract::{Multipart, Path, Query, State},
    http::{header, HeaderMap, HeaderValue, Request, StatusCode},
    middleware::{from_fn_with_state, Next},
    response::{IntoResponse, Redirect, Response},
    routing::{get, post, put, delete},
    Json, Router,
};
use rand::{distributions::Alphanumeric, Rng};
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
    admin_sessions: Arc<RwLock<std::collections::HashMap<String, u64>>>,
}

const ADMIN_SESSION_COOKIE: &str = "rev0auth_admin_session";
const ADMIN_SESSION_TTL_SECS: u64 = 8 * 60 * 60;

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
    commentary: String,
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
    commentary: Option<String>,
    status: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MemberStatusInput {
    pseudo: String,
    status: String,
    commentary: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DeleteAccountInput {
    pseudo: String,
}

#[derive(Debug, Deserialize)]
struct AdminLoginInput {
    password: String,
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
    commentary: String,
    avatar_filename: Option<String>,
    avatar_size_bytes: Option<usize>,
    created_at_epoch: u64,
}

#[derive(Debug, Serialize)]
struct MessageResponse {
    ok: bool,
    message: String,
}

#[derive(Debug, Serialize)]
struct AdminLoginResponse {
    ok: bool,
    message: &'static str,
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
        admin_sessions: Arc::new(RwLock::new(std::collections::HashMap::new())),
    };

    let protected_state = state.clone();

    let protected_routes = Router::new()
        .route("/dashboard", get(dashboard))
        .route("/japprends/tdd", get(tdd_dashboard))
        .route("/status", get(status))
        .route("/status/all", get(status_all))
        .route("/japprends/signup-requests", get(admin_signup_requests))
        .route(
            "/japprends/signup-requests/:id/approve",
            post(admin_approve_signup_request),
        )
        .route(
            "/japprends/signup-requests/:id/reject",
            post(admin_reject_signup_request),
        )
        .route("/japprends/ping", get(admin_ping))
        .route("/user/ping", get(user_ping))
        .route("/japprends/set-password/:pseudo", post(admin_set_password))
        .route("/japprends/remove-password/:pseudo", post(admin_remove_password))
        .route("/users", get(list_users))
        .route("/japprends/users", post(admin_create_user))
        .route("/japprends/users/:pseudo", put(admin_update_user))
        .route("/japprends/users/:pseudo", delete(admin_delete_user))
        .route("/status/set-busy/:pseudo", post(set_user_busy))
        .route("/status/set-active/:pseudo", post(set_user_active))
        .route("/status/set-inactive/:pseudo", post(set_user_inactive))
        .route_layer(from_fn_with_state(
            protected_state,
            require_admin_session,
        ));

    let app = Router::new()
        .route("/", get(home))
        .route("/portal", get(portal))
        .route("/portal/signup-request", post(portal_signup_request))
        .route("/portal/login", post(portal_login))
        .route("/japprends/login", get(admin_login_page))
        .route("/japprends/login", post(admin_login))
        .route("/japprends/logout", post(admin_logout))
        .route("/home/friend", get(friend_home))
        .route("/members/dashboard", get(members_dashboard))
        .route("/members/profile", get(members_profile_page))
        .route("/members/profile/data", get(member_profile_data))
        .route("/members/profile/data", put(member_update_profile))
        .route("/members/status", put(member_set_status))
        .route("/members/account", delete(member_delete_account))
        .route("/members/avatar", post(member_upload_avatar))
        .route("/auth/password-check", post(password_check))
        .merge(protected_routes)
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

async fn admin_login_page() -> impl axum::response::IntoResponse {
    pages::admin_login().await
}

async fn dashboard() -> impl axum::response::IntoResponse {
    pages::dashboard().await
}

async fn tdd_dashboard() -> impl axum::response::IntoResponse {
    pages::tdd_dashboard().await
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

async fn admin_login(
    State(state): State<WebState>,
    Json(payload): Json<AdminLoginInput>,
) -> impl IntoResponse {
    let Some(expected_password) = admin_password_from_env() else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(AdminLoginResponse {
                ok: false,
                message: "admin password not configured",
            }),
        )
            .into_response();
    };

    if payload.password != expected_password {
        return (
            StatusCode::UNAUTHORIZED,
            Json(AdminLoginResponse {
                ok: false,
                message: "mot de passe admin invalide",
            }),
        )
            .into_response();
    }

    let token = generate_admin_session_token();
    let expires_at = now_epoch() + ADMIN_SESSION_TTL_SECS;
    {
        let mut sessions = state.admin_sessions.write().await;
        sessions.insert(token.clone(), expires_at);
    }

    let mut headers = HeaderMap::new();
    headers.append(
        header::SET_COOKIE,
        HeaderValue::from_str(&build_admin_cookie(&token)).expect("valid admin cookie"),
    );

    (
        StatusCode::OK,
        headers,
        Json(AdminLoginResponse {
            ok: true,
            message: "connexion admin validee",
        }),
    )
        .into_response()
}

async fn admin_logout(State(state): State<WebState>, headers: HeaderMap) -> impl IntoResponse {
    if let Some(token) = extract_cookie_from_headers(&headers, ADMIN_SESSION_COOKIE) {
        let mut sessions = state.admin_sessions.write().await;
        sessions.remove(&token);
    }

    let mut out_headers = HeaderMap::new();
    out_headers.append(
        header::SET_COOKIE,
        HeaderValue::from_str(&build_admin_logout_cookie()).expect("valid logout cookie"),
    );

    (
        StatusCode::OK,
        out_headers,
        Json(AdminLoginResponse {
            ok: true,
            message: "deconnexion admin ok",
        }),
    )
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
            commentary: String::new(),
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
        commentary: String::new(),
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
            commentary: user.commentary.clone(),
            avatar_filename: user.avatar_filename.clone(),
            avatar_size_bytes: user.avatar_size_bytes,
            created_at_epoch: user.created_at_epoch,
        });
    }

    Json(MemberProfileResponse {
        ok: false,
        pseudo: query.pseudo,
        role: "member",
        status: UserStatus::Inactif,
        bio: String::new(),
        commentary: String::new(),
        avatar_filename: None,
        avatar_size_bytes: None,
        created_at_epoch: 0,
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
        if let Some(commentary) = payload.commentary {
            user.commentary = commentary.trim().to_string();
        }
        if let Some(status) = payload.status {
            user.status = parse_member_status(&status).unwrap_or(user.status);
        }
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

async fn member_set_status(
    State(state): State<WebState>,
    Json(payload): Json<MemberStatusInput>,
) -> Json<ActionResponse> {
    let Some(next_status) = parse_member_status(&payload.status) else {
        return Json(ActionResponse {
            ok: false,
            message: "Statut invalide (content|bof|question).",
        });
    };

    let mut users = state.users.write().await;
    if let Some(user) = users
        .iter_mut()
        .find(|u| u.pseudo.eq_ignore_ascii_case(payload.pseudo.trim()))
    {
        user.status = next_status;
        if let Some(commentary) = payload.commentary {
            user.commentary = commentary.trim().to_string();
        }

        return Json(ActionResponse {
            ok: true,
            message: "Statut mis a jour.",
        });
    }

    Json(ActionResponse {
        ok: false,
        message: "Utilisateur introuvable.",
    })
}

async fn member_delete_account(
    State(state): State<WebState>,
    Json(payload): Json<DeleteAccountInput>,
) -> Json<ActionResponse> {
    let pseudo = payload.pseudo.trim().to_string();
    if pseudo.is_empty() {
        return Json(ActionResponse {
            ok: false,
            message: "Pseudo manquant.",
        });
    }

    let mut users = state.users.write().await;
    let initial_len = users.len();
    users.retain(|u| !u.pseudo.eq_ignore_ascii_case(&pseudo));
    let deleted = users.len() < initial_len;
    drop(users);

    if !deleted {
        return Json(ActionResponse {
            ok: false,
            message: "Utilisateur introuvable.",
        });
    }

    let mut passwords = state.user_passwords.write().await;
    passwords.remove(&pseudo);

    Json(ActionResponse {
        ok: true,
        message: "Compte supprime.",
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

fn parse_member_status(raw: &str) -> Option<UserStatus> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "content" | "happy" => Some(UserStatus::Actif),
        "bof" | "meh" => Some(UserStatus::Occupe),
        "question" | "help" | "improvement" => Some(UserStatus::Inactif),
        _ => None,
    }
}

fn admin_password_from_env() -> Option<String> {
    std::env::var("ADMIN_DASH_PASSWORD")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

fn generate_admin_session_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(48)
        .map(char::from)
        .collect()
}

fn build_admin_cookie(token: &str) -> String {
    format!(
        "{}={}; HttpOnly; SameSite=Lax; Path=/; Max-Age={}",
        ADMIN_SESSION_COOKIE, token, ADMIN_SESSION_TTL_SECS
    )
}

fn build_admin_logout_cookie() -> String {
    format!(
        "{}=deleted; HttpOnly; SameSite=Lax; Path=/; Max-Age=0",
        ADMIN_SESSION_COOKIE
    )
}

fn extract_cookie_from_headers(headers: &HeaderMap, cookie_name: &str) -> Option<String> {
    let raw = headers.get(header::COOKIE)?.to_str().ok()?;
    raw.split(';')
        .map(|part| part.trim())
        .find_map(|pair| {
            let (name, value) = pair.split_once('=')?;
            if name == cookie_name {
                Some(value.to_string())
            } else {
                None
            }
        })
}

async fn is_admin_authenticated(headers: &HeaderMap, state: &WebState) -> bool {
    let Some(token) = extract_cookie_from_headers(headers, ADMIN_SESSION_COOKIE) else {
        return false;
    };

    let now = now_epoch();
    let mut sessions = state.admin_sessions.write().await;
    sessions.retain(|_, expires_at| *expires_at > now);
    sessions.get(&token).is_some_and(|expires_at| *expires_at > now)
}

async fn require_admin_session(
    State(state): State<WebState>,
    req: Request<Body>,
    next: Next,
) -> Response {
    if is_admin_authenticated(req.headers(), &state).await {
        return next.run(req).await;
    }

    if req.method() == axum::http::Method::GET {
        return Redirect::to("/japprends/login").into_response();
    }

    (
        StatusCode::UNAUTHORIZED,
        Json(AdminLoginResponse {
            ok: false,
            message: "admin auth required",
        }),
    )
        .into_response()
}
