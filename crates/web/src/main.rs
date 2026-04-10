mod pages;

use axum::{
    body::Body,
    extract::{Multipart, Path, Query, State},
    http::{header, HeaderMap, HeaderValue, Request, StatusCode},
    middleware::{from_fn_with_state, Next},
    response::{IntoResponse, Redirect, Response},
    routing::{get, post, put, delete},
    Json, Router,
};
use data_encoding::BASE32_NOPAD;
use hmac::{Hmac, Mac};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use sha1::Sha1;
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
    next_test_run_id: Arc<AtomicU64>,
    users: Arc<RwLock<Vec<User>>>,
    member_messages: Arc<RwLock<Vec<MemberMessage>>>,
    next_message_id: Arc<AtomicU64>,
    donation_proofs: Arc<RwLock<Vec<DonationProof>>>,
    next_donation_id: Arc<AtomicU64>,
    user_passwords: Arc<RwLock<std::collections::HashMap<String, String>>>,
    admin_sessions: Arc<RwLock<std::collections::HashMap<String, u64>>>,
    dashboard_test_runs: Arc<RwLock<Vec<DashboardTestRun>>>,
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
    access_github: bool,
    access_jellyfin: bool,
    access_songsurf: bool,
    request_github: bool,
    request_jellyfin: bool,
    request_songsurf: bool,
    github_star_claimed: bool,
    github_username: Option<String>,
    avatar_filename: Option<String>,
    avatar_size_bytes: Option<usize>,
    avatar_mime_type: Option<String>,
    avatar_bytes: Option<Vec<u8>>,
    must_change_password: bool,
    created_at_epoch: u64,
}

#[derive(Debug, Clone)]
struct MemberMessage {
    id: u64,
    from_pseudo: String,
    to_pseudo: String,
    subject: String,
    body: String,
    is_read: bool,
    created_at_epoch: u64,
}

#[derive(Debug, Clone)]
struct DonationProof {
    id: u64,
    pseudo: String,
    method: String,
    code: String,
    photo_filename: Option<String>,
    photo_mime_type: String,
    photo_bytes: Vec<u8>,
    reviewed: bool,
    approved: bool,
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
    referral: String,
    #[serde(skip)]
    temp_password: String,
    status: ManualStatus,
    created_at_epoch: u64,
}

#[derive(Debug, Deserialize)]
struct SignupRequestInput {
    pseudo: String,
    referral: String,
    #[serde(default)]
    temp_password: Option<String>,
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

#[derive(Debug, Serialize)]
struct AccountPasswordResponse {
    ok: bool,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    temp_password: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UpdateUserInput {
    status: Option<String>,
    access_github: Option<bool>,
    access_jellyfin: Option<bool>,
    access_songsurf: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct AccessRequestInput {
    pseudo: String,
    service: String,
    github_username: Option<String>,
    starred: Option<bool>,
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
struct PasswordUpdateInput {
    pseudo: String,
    current_password: Option<String>,
    new_password: String,
}

#[derive(Debug, Deserialize)]
struct MessageSendInput {
    from_pseudo: String,
    subject: String,
    body: String,
}

#[derive(Debug, Deserialize)]
struct AdminMessageSendInput {
    to_pseudo: String,
    subject: String,
    body: String,
}

#[derive(Debug, Deserialize)]
struct MessageListQuery {
    pseudo: String,
}

#[derive(Debug, Deserialize)]
struct DonationListQuery {
    pseudo: String,
}

#[derive(Debug, Deserialize)]
struct DonationReviewInput {
    approved: bool,
}

#[derive(Debug, Deserialize)]
struct AdminLoginInput {
    pseudo: String,
    seed: String,
    password: String,
    #[serde(default)]
    otp: Option<String>,
    challenge_choice: String,
    #[serde(default)]
    trap_value: Option<String>,
}

#[derive(Debug, Serialize)]
struct PasswordCheckResponse {
    ok: bool,
    state: &'static str,
    message: &'static str,
}

#[derive(Debug, Serialize)]
struct SignupResponse {
    ok: bool,
    request_id: u64,
    status: &'static str,
    message: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    temp_password: Option<String>,
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
    access_github: bool,
    access_jellyfin: bool,
    access_songsurf: bool,
    request_github: bool,
    request_jellyfin: bool,
    request_songsurf: bool,
    github_star_claimed: bool,
    github_username: Option<String>,
    avatar_present: bool,
    avatar_filename: Option<String>,
    avatar_size_bytes: Option<usize>,
    created_at_epoch: u64,
}

#[derive(Debug, Serialize)]
struct MemberMessageView {
    id: u64,
    from_pseudo: String,
    to_pseudo: String,
    subject: String,
    body: String,
    is_read: bool,
    created_at_epoch: u64,
}

#[derive(Debug, Serialize)]
struct DonationProofView {
    id: u64,
    pseudo: String,
    method: String,
    code: String,
    photo_filename: Option<String>,
    reviewed: bool,
    approved: bool,
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

#[derive(Debug, Clone, Serialize)]
struct DashboardTestCase {
    name: &'static str,
    ok: bool,
    detail: String,
}
#[derive(Debug, Clone, Serialize)]
struct DashboardTestRun {
    run_id: u64,
    executed_at_epoch: u64,
    passed: usize,
    total: usize,
    cases: Vec<DashboardTestCase>,
}

#[derive(Debug, Clone, Serialize)]
struct EndpointInfo {
    method: &'static str,
    path: &'static str,
    scope: &'static str,
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
        next_test_run_id: Arc::new(AtomicU64::new(1)),
        users: Arc::new(RwLock::new(Vec::new())),
        member_messages: Arc::new(RwLock::new(Vec::new())),
        next_message_id: Arc::new(AtomicU64::new(1)),
        donation_proofs: Arc::new(RwLock::new(Vec::new())),
        next_donation_id: Arc::new(AtomicU64::new(1)),
        user_passwords: Arc::new(RwLock::new(std::collections::HashMap::new())),
        admin_sessions: Arc::new(RwLock::new(std::collections::HashMap::new())),
        dashboard_test_runs: Arc::new(RwLock::new(Vec::new())),
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
        .route("/japprends/auth-check", post(admin_auth_check))
        .route("/user/ping", get(user_ping))
        .route("/japprends/set-password/:pseudo", post(admin_set_password))
        .route("/japprends/remove-password/:pseudo", post(admin_remove_password))
        .route("/users", get(list_users))
        .route("/japprends/users", post(admin_create_user))
        .route("/japprends/users/:pseudo", put(admin_update_user))
        .route("/japprends/users/:pseudo", delete(admin_delete_user))
        .route("/japprends/tests/history", get(admin_tests_history))
        .route("/japprends/tests/launch", post(admin_launch_tests_now))
        .route("/japprends/endpoints", get(admin_all_endpoints))
        .route("/japprends/messages", get(admin_messages_all))
        .route("/japprends/messages/reply", post(admin_message_reply))
        .route("/japprends/donations", get(admin_donations_all))
        .route("/japprends/donations/:id/review", post(admin_donation_review))
        .route("/status/set-busy/:pseudo", post(set_user_busy))
        .route("/status/set-active/:pseudo", post(set_user_active))
        .route("/status/set-inactive/:pseudo", post(set_user_inactive))
        .route_layer(from_fn_with_state(protected_state, require_admin_session));

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
        .route("/members/password", put(member_update_password))
        .route("/members/status", put(member_set_status))
        .route("/members/access/request", post(member_request_access))
        .route("/members/messages/send", post(member_message_send))
        .route("/members/messages/inbox", get(member_messages_inbox))
        .route("/members/messages/sent", get(member_messages_sent))
        .route("/members/messages/:id/read", post(member_mark_message_read))
        .route("/members/donations/proof", post(member_upload_donation_proof))
        .route("/members/donations/proof/:id/photo", get(member_donation_photo))
        .route("/members/donations", get(member_donations))
        .route("/members/account", delete(member_delete_account))
        .route("/members/avatar", post(member_upload_avatar))
        .route("/members/avatar/:pseudo", get(member_avatar))
        .route("/members/avatar/:pseudo", delete(member_delete_avatar))
        .route("/auth/password-check", post(password_check))
        .merge(protected_routes)
        .with_state(state);
    let addr = web_bind_addr();

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
    let api_upstream = api_upstream_addr();
    let api_ok = timeout(
        Duration::from_millis(500),
        tokio::net::TcpStream::connect(api_upstream.as_str()),
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

async fn admin_auth_check() -> Json<ActionResponse> {
    Json(ActionResponse {
        ok: true,
        message: "admin auth ok",
    })
}

async fn status_all(State(state): State<WebState>) -> Json<StatusAllResponse> {
    let api_upstream = api_upstream_addr();
    let api_ok = timeout(
        Duration::from_millis(500),
        tokio::net::TcpStream::connect(api_upstream.as_str()),
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
    if payload.pseudo.trim().is_empty() || payload.referral.trim().is_empty() {
        return Json(SignupResponse {
            ok: false,
            request_id: 0,
            status: "rejected",
            message: "Champs invalides: remplis pseudo et referral.",
            temp_password: None,
        });
    }

    let pseudo = payload.pseudo.trim().to_string();
    let id = state.next_request_id.fetch_add(1, Ordering::Relaxed);
    let temp_password = payload
        .temp_password
        .map(|pwd| pwd.trim().to_string())
        .filter(|pwd| !pwd.is_empty())
        .unwrap_or_else(generate_temp_password);

    let mut users = state.users.write().await;
    if users.iter().any(|u| u.pseudo.eq_ignore_ascii_case(&pseudo)) {
        return Json(SignupResponse {
            ok: false,
            request_id: 0,
            status: "rejected",
            message: "Pseudo deja utilise.",
            temp_password: None,
        });
    }

    users.push(User {
        pseudo: pseudo.clone(),
        role: "member",
        active: true,
        status: UserStatus::Actif,
        bio: String::new(),
        commentary: String::new(),
        access_github: false,
        access_jellyfin: false,
        access_songsurf: false,
        request_github: false,
        request_jellyfin: false,
        request_songsurf: false,
        github_star_claimed: false,
        github_username: None,
        avatar_filename: None,
        avatar_size_bytes: None,
        avatar_mime_type: None,
        avatar_bytes: None,
        must_change_password: true,
        created_at_epoch: now_epoch(),
    });
    drop(users);

    let mut passwords = state.user_passwords.write().await;
    passwords.insert(pseudo_key(&pseudo), temp_password.clone());
    drop(passwords);

    let request = SignupRequestRecord {
        id,
        pseudo,
        referral: payload.referral.trim().to_string(),
        temp_password: temp_password.clone(),
        status: ManualStatus::Approved,
        created_at_epoch: now_epoch(),
    };

    let mut requests = state.signup_requests.write().await;
    requests.push(request);

    Json(SignupResponse {
        ok: true,
        request_id: id,
        status: "approved",
        message: "Compte cree immediatement.",
        temp_password: Some(temp_password),
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

    let users = state.users.read().await;
    let exists = users.iter().any(|u| u.pseudo.eq_ignore_ascii_case(pseudo));

    if exists {
        Json(LoginResponse {
            ok: true,
            state: "approved",
            message: "Connexion autorisee.",
        })
    } else {
        Json(LoginResponse {
            ok: false,
            state: "missing",
            message: "Compte introuvable.",
        })
    }
}

async fn admin_login(
    State(state): State<WebState>,
    Json(payload): Json<AdminLoginInput>,
) -> impl IntoResponse {
    if payload
        .trap_value
        .as_ref()
        .map(|v| !v.trim().is_empty())
        .unwrap_or(false)
    {
        return (
            StatusCode::UNAUTHORIZED,
            Json(AdminLoginResponse {
                ok: false,
                message: "tentative invalide",
            }),
        )
            .into_response();
    }

    if payload.challenge_choice.trim() != "secure-lock" {
        return (
            StatusCode::UNAUTHORIZED,
            Json(AdminLoginResponse {
                ok: false,
                message: "challenge invalide",
            }),
        )
            .into_response();
    }

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

    let expected_pseudo = admin_pseudo_from_env();
    let expected_seed = admin_seed_from_env();

    if payload.pseudo.trim() != expected_pseudo || payload.seed.trim() != expected_seed {
        return (
            StatusCode::UNAUTHORIZED,
            Json(AdminLoginResponse {
                ok: false,
                message: "identifiants admin invalides",
            }),
        )
            .into_response();
    }

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

    if let Some(totp_secret) = admin_totp_secret_from_env() {
        let otp = payload.otp.unwrap_or_default();
        if !verify_totp_code(&totp_secret, otp.trim(), now_epoch()) {
            return (
                StatusCode::UNAUTHORIZED,
                Json(AdminLoginResponse {
                    ok: false,
                    message: "code 2FA invalide",
                }),
            )
                .into_response();
        }
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
) -> Json<AccountPasswordResponse> {
    let mut requests = state.signup_requests.write().await;
    let maybe = requests.iter_mut().find(|r| r.id == id);

    if let Some(req) = maybe {
        let mut users = state.users.write().await;
        if users.iter().any(|u| u.pseudo.eq_ignore_ascii_case(&req.pseudo)) {
            return Json(AccountPasswordResponse {
                ok: false,
                message: "Utilisateur existe deja.".to_string(),
                temp_password: None,
            });
        }

        req.status = ManualStatus::Approved;

        info!(target: "rev0auth", "User approved: {}", req.pseudo);
        let user = User {
            pseudo: req.pseudo.clone(),
            role: "member",
            active: true,
            status: UserStatus::Actif,
            bio: String::new(),
            commentary: String::new(),
            access_github: false,
            access_jellyfin: false,
            access_songsurf: false,
            request_github: false,
            request_jellyfin: false,
            request_songsurf: false,
            github_star_claimed: false,
            github_username: None,
            avatar_filename: None,
            avatar_size_bytes: None,
            avatar_mime_type: None,
            avatar_bytes: None,
            must_change_password: true,
            created_at_epoch: now_epoch(),
        };
        users.push(user);
        drop(users);

        let temp_password = if req.temp_password.trim().is_empty() {
            generate_temp_password()
        } else {
            req.temp_password.clone()
        };
        let mut passwords = state.user_passwords.write().await;
        passwords.insert(pseudo_key(&req.pseudo), temp_password.clone());

        return Json(AccountPasswordResponse {
            ok: true,
            message: "Demande approuvee. Mot de passe temporaire genere.".to_string(),
            temp_password: Some(temp_password),
        });
    }

    Json(AccountPasswordResponse {
        ok: false,
        message: "Demande introuvable.".to_string(),
        temp_password: None,
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

async fn admin_tests_history(State(state): State<WebState>) -> Json<Vec<DashboardTestRun>> {
    let runs = state.dashboard_test_runs.read().await;
    Json(runs.iter().cloned().rev().collect())
}

async fn admin_launch_tests_now(State(state): State<WebState>) -> Json<DashboardTestRun> {
    let api_upstream = api_upstream_addr();
    let api_ok = timeout(
        Duration::from_millis(500),
        tokio::net::TcpStream::connect(api_upstream.as_str()),
    )
    .await
    .map(|r| r.is_ok())
    .unwrap_or(false);

    let admin_pwd_ok = admin_password_from_env().is_some();
    let users_total = state.users.read().await.len();

    let cases = vec![
        DashboardTestCase {
            name: "api_health_socket",
            ok: api_ok,
            detail: if api_ok {
                format!("{api_upstream} reachable")
            } else {
                format!("cannot reach {api_upstream}")
            },
        },
        DashboardTestCase {
            name: "admin_password_configured",
            ok: admin_pwd_ok,
            detail: if admin_pwd_ok {
                "ADMIN_DASH_PASSWORD configured".to_string()
            } else {
                "missing ADMIN_DASH_PASSWORD".to_string()
            },
        },
        DashboardTestCase {
            name: "user_store_access",
            ok: true,
            detail: format!("user records in memory: {users_total}"),
        },
    ];

    let passed = cases.iter().filter(|c| c.ok).count();
    let run = DashboardTestRun {
        run_id: state.next_test_run_id.fetch_add(1, Ordering::Relaxed),
        executed_at_epoch: now_epoch(),
        passed,
        total: cases.len(),
        cases,
    };

    {
        let mut runs = state.dashboard_test_runs.write().await;
        runs.push(run.clone());
        if runs.len() > 200 {
            let drain_to = runs.len().saturating_sub(200);
            if drain_to > 0 {
                runs.drain(0..drain_to);
            }
        }
    }

    Json(run)
}

async fn admin_all_endpoints() -> Json<Vec<EndpointInfo>> {
    Json(vec![
        EndpointInfo { method: "GET", path: "/", scope: "public" },
        EndpointInfo { method: "GET", path: "/portal", scope: "public" },
        EndpointInfo { method: "POST", path: "/portal/signup-request", scope: "public" },
        EndpointInfo { method: "POST", path: "/portal/login", scope: "public" },
        EndpointInfo { method: "GET", path: "/japprends/login", scope: "public" },
        EndpointInfo { method: "POST", path: "/japprends/login", scope: "public" },
        EndpointInfo { method: "POST", path: "/japprends/logout", scope: "admin" },
        EndpointInfo { method: "GET", path: "/dashboard", scope: "admin" },
        EndpointInfo { method: "GET", path: "/japprends/tdd", scope: "admin" },
        EndpointInfo { method: "GET", path: "/status", scope: "admin" },
        EndpointInfo { method: "GET", path: "/status/all", scope: "admin" },
        EndpointInfo { method: "GET", path: "/japprends/ping", scope: "admin" },
        EndpointInfo { method: "POST", path: "/japprends/auth-check", scope: "admin" },
        EndpointInfo { method: "GET", path: "/user/ping", scope: "admin" },
        EndpointInfo { method: "GET", path: "/japprends/signup-requests", scope: "admin" },
        EndpointInfo { method: "POST", path: "/japprends/signup-requests/:id/approve", scope: "admin" },
        EndpointInfo { method: "POST", path: "/japprends/signup-requests/:id/reject", scope: "admin" },
        EndpointInfo { method: "POST", path: "/japprends/users", scope: "admin" },
        EndpointInfo { method: "PUT", path: "/japprends/users/:pseudo", scope: "admin" },
        EndpointInfo { method: "DELETE", path: "/japprends/users/:pseudo", scope: "admin" },
        EndpointInfo { method: "POST", path: "/japprends/tests/launch", scope: "admin" },
        EndpointInfo { method: "GET", path: "/japprends/tests/history", scope: "admin" },
        EndpointInfo { method: "GET", path: "/japprends/endpoints", scope: "admin" },
        EndpointInfo { method: "GET", path: "/japprends/messages", scope: "admin" },
        EndpointInfo { method: "POST", path: "/japprends/messages/reply", scope: "admin" },
        EndpointInfo { method: "GET", path: "/japprends/donations", scope: "admin" },
        EndpointInfo { method: "POST", path: "/japprends/donations/:id/review", scope: "admin" },
        EndpointInfo { method: "GET", path: "/members/dashboard", scope: "member" },
        EndpointInfo { method: "GET", path: "/members/profile", scope: "member" },
        EndpointInfo { method: "GET", path: "/members/profile/data", scope: "member" },
        EndpointInfo { method: "PUT", path: "/members/profile/data", scope: "member" },
        EndpointInfo { method: "PUT", path: "/members/password", scope: "member" },
        EndpointInfo { method: "POST", path: "/members/access/request", scope: "member" },
        EndpointInfo { method: "POST", path: "/members/messages/send", scope: "member" },
        EndpointInfo { method: "GET", path: "/members/messages/inbox", scope: "member" },
        EndpointInfo { method: "GET", path: "/members/messages/sent", scope: "member" },
        EndpointInfo { method: "POST", path: "/members/messages/:id/read", scope: "member" },
        EndpointInfo { method: "POST", path: "/members/donations/proof", scope: "member" },
        EndpointInfo { method: "GET", path: "/members/donations", scope: "member" },
        EndpointInfo { method: "GET", path: "/members/donations/proof/:id/photo", scope: "member" },
        EndpointInfo { method: "PUT", path: "/members/status", scope: "member" },
        EndpointInfo { method: "DELETE", path: "/members/account", scope: "member" },
        EndpointInfo { method: "POST", path: "/members/avatar", scope: "member" },
        EndpointInfo { method: "POST", path: "/auth/password-check", scope: "member" },
    ])
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
    let pseudo = payload.pseudo.trim();
    let password = payload.password.trim();
    if pseudo.is_empty() || password.is_empty() {
        return Json(PasswordCheckResponse {
            ok: false,
            state: "invalid",
            message: "Pseudo et mot de passe requis.",
        });
    }

    let passwords = state.user_passwords.read().await;
    let stored_password = passwords.get(&pseudo_key(pseudo));

    match stored_password {
        Some(pwd) if pwd == password => {
            let users = state.users.read().await;
            let requires_change = users
                .iter()
                .find(|u| u.pseudo.eq_ignore_ascii_case(pseudo))
                .map(|u| u.must_change_password)
                .unwrap_or(false);

            Json(PasswordCheckResponse {
                ok: true,
                state: if requires_change { "onboarding" } else { "ok" },
                message: if requires_change {
                    "Mot de passe correct. Onboarding requis."
                } else {
                    "Mot de passe correct. Connexion autorisee."
                },
            })
        }
        Some(_) => Json(PasswordCheckResponse {
            ok: false,
            state: "invalid",
            message: "Mot de passe incorrect.",
        }),
        None => Json(PasswordCheckResponse {
            ok: false,
            state: "missing",
            message: "Mot de passe non configure. Contacte l'admin pour initialiser le compte.",
        }),
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
    if !users.iter().any(|u| u.pseudo.eq_ignore_ascii_case(&pseudo)) {
        return Json(ActionResponse {
            ok: false,
            message: "Utilisateur introuvable.",
        });
    }

    // Set password
    let mut passwords = state.user_passwords.write().await;
    passwords.insert(pseudo_key(&pseudo), payload.password);

    drop(passwords);

    let mut users = state.users.write().await;
    if let Some(user) = users.iter_mut().find(|u| u.pseudo.eq_ignore_ascii_case(&pseudo)) {
        user.must_change_password = true;
    }
    
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
    if !users.iter().any(|u| u.pseudo.eq_ignore_ascii_case(&pseudo)) {
        return Json(ActionResponse {
            ok: false,
            message: "Utilisateur introuvable.",
        });
    }

    drop(users);
    
    // Remove password
    let mut passwords = state.user_passwords.write().await;
    if passwords.remove(&pseudo_key(&pseudo)).is_some() {
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
    State(_state): State<WebState>,
    Json(_payload): Json<serde_json::Value>,
) -> Json<AccountPasswordResponse> {
    Json(AccountPasswordResponse {
        ok: false,
        message: "Creation manuelle desactivee pour eviter les collisions de compte.".to_string(),
        temp_password: None,
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

        if let Some(access) = payload.access_jellyfin {
            user.access_jellyfin = access;
            if access {
                user.request_jellyfin = false;
            }
        }

        if let Some(access) = payload.access_songsurf {
            user.access_songsurf = access;
            if access {
                user.request_songsurf = false;
            }
        }

        if let Some(access) = payload.access_github {
            if access && !user.github_star_claimed {
                return Json(ActionResponse {
                    ok: false,
                    message: "User doit confirmer sa star GitHub avant activation.",
                });
            }
            user.access_github = access;
            if access {
                user.request_github = false;
            }
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
        passwords.remove(&pseudo_key(&pseudo));
        drop(passwords);

        let mut messages = state.member_messages.write().await;
        messages.retain(|m| {
            !m.from_pseudo.eq_ignore_ascii_case(&pseudo)
                && !m.to_pseudo.eq_ignore_ascii_case(&pseudo)
        });
        drop(messages);

        let mut donations = state.donation_proofs.write().await;
        donations.retain(|d| !d.pseudo.eq_ignore_ascii_case(&pseudo));
        
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
            access_github: user.access_github,
            access_jellyfin: user.access_jellyfin,
            access_songsurf: user.access_songsurf,
            request_github: user.request_github,
            request_jellyfin: user.request_jellyfin,
            request_songsurf: user.request_songsurf,
            github_star_claimed: user.github_star_claimed,
            github_username: user.github_username.clone(),
            avatar_present: user.avatar_bytes.is_some(),
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
        access_github: false,
        access_jellyfin: false,
        access_songsurf: false,
        request_github: false,
        request_jellyfin: false,
        request_songsurf: false,
        github_star_claimed: false,
        github_username: None,
        avatar_present: false,
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

async fn member_request_access(
    State(state): State<WebState>,
    Json(payload): Json<AccessRequestInput>,
) -> Json<ActionResponse> {
    let pseudo = payload.pseudo.trim();
    if pseudo.is_empty() {
        return Json(ActionResponse {
            ok: false,
            message: "Pseudo manquant.",
        });
    }

    let service = payload.service.trim().to_ascii_lowercase();
    let mut users = state.users.write().await;
    let Some(user) = users
        .iter_mut()
        .find(|u| u.pseudo.eq_ignore_ascii_case(pseudo))
    else {
        return Json(ActionResponse {
            ok: false,
            message: "Utilisateur introuvable.",
        });
    };

    match service.as_str() {
        "jellyfin" => {
            user.request_jellyfin = true;
            Json(ActionResponse {
                ok: true,
                message: "Demande Jellyfin envoyee a l'admin.",
            })
        }
        "songsurf" => {
            user.request_songsurf = true;
            Json(ActionResponse {
                ok: true,
                message: "Demande Songsurf envoyee a l'admin.",
            })
        }
        "github" => {
            let Some(true) = payload.starred else {
                return Json(ActionResponse {
                    ok: false,
                    message: "Confirme la star GitHub avant de demander l'acces.",
                });
            };

            let username = payload
                .github_username
                .map(|u| u.trim().to_string())
                .filter(|u| !u.is_empty());
            if username.is_none() {
                return Json(ActionResponse {
                    ok: false,
                    message: "GitHub username requis.",
                });
            }

            user.request_github = true;
            user.github_star_claimed = true;
            user.github_username = username;
            Json(ActionResponse {
                ok: true,
                message: "Demande GitHub envoyee, en attente de validation admin.",
            })
        }
        _ => Json(ActionResponse {
            ok: false,
            message: "Service invalide.",
        }),
    }
}

async fn member_message_send(
    State(state): State<WebState>,
    Json(payload): Json<MessageSendInput>,
) -> Json<MessageResponse> {
    let from_pseudo = payload.from_pseudo.trim().to_string();
    let to_pseudo = admin_pseudo_from_env();
    let subject = payload.subject.trim().to_string();
    let body = payload.body.trim().to_string();

    if from_pseudo.is_empty() || subject.is_empty() || body.is_empty() {
        return Json(MessageResponse {
            ok: false,
            message: "Expediteur, sujet et message sont requis.".to_string(),
        });
    }

    if from_pseudo.eq_ignore_ascii_case(&to_pseudo) {
        return Json(MessageResponse {
            ok: false,
            message: "Tu ne peux pas t'envoyer un message a toi-meme.".to_string(),
        });
    }

    let users = state.users.read().await;
    let sender_exists = users.iter().any(|u| u.pseudo.eq_ignore_ascii_case(&from_pseudo));
    if !sender_exists {
        return Json(MessageResponse {
            ok: false,
            message: "Expediteur introuvable.".to_string(),
        });
    }
    drop(users);

    let mut messages = state.member_messages.write().await;
    let id = state.next_message_id.fetch_add(1, Ordering::Relaxed);
    messages.push(MemberMessage {
        id,
        from_pseudo,
        to_pseudo,
        subject,
        body,
        is_read: false,
        created_at_epoch: now_epoch(),
    });

    Json(MessageResponse {
        ok: true,
        message: "Message envoye.".to_string(),
    })
}

async fn member_messages_inbox(
    Query(query): Query<MessageListQuery>,
    State(state): State<WebState>,
) -> Json<Vec<MemberMessageView>> {
    let pseudo = query.pseudo.trim();
    let messages = state.member_messages.read().await;
    let list = messages
        .iter()
        .filter(|m| m.to_pseudo.eq_ignore_ascii_case(pseudo))
        .map(|m| MemberMessageView {
            id: m.id,
            from_pseudo: m.from_pseudo.clone(),
            to_pseudo: m.to_pseudo.clone(),
            subject: m.subject.clone(),
            body: m.body.clone(),
            is_read: m.is_read,
            created_at_epoch: m.created_at_epoch,
        })
        .collect();
    Json(list)
}

async fn member_messages_sent(
    Query(query): Query<MessageListQuery>,
    State(state): State<WebState>,
) -> Json<Vec<MemberMessageView>> {
    let pseudo = query.pseudo.trim();
    let messages = state.member_messages.read().await;
    let list = messages
        .iter()
        .filter(|m| m.from_pseudo.eq_ignore_ascii_case(pseudo))
        .map(|m| MemberMessageView {
            id: m.id,
            from_pseudo: m.from_pseudo.clone(),
            to_pseudo: m.to_pseudo.clone(),
            subject: m.subject.clone(),
            body: m.body.clone(),
            is_read: m.is_read,
            created_at_epoch: m.created_at_epoch,
        })
        .collect();
    Json(list)
}

async fn member_mark_message_read(
    Path(id): Path<u64>,
    State(state): State<WebState>,
    Json(query): Json<MessageListQuery>,
) -> Json<ActionResponse> {
    let pseudo = query.pseudo.trim();
    let mut messages = state.member_messages.write().await;
    if let Some(msg) = messages
        .iter_mut()
        .find(|m| m.id == id && m.to_pseudo.eq_ignore_ascii_case(pseudo))
    {
        msg.is_read = true;
        return Json(ActionResponse {
            ok: true,
            message: "Message marque comme lu.",
        });
    }

    Json(ActionResponse {
        ok: false,
        message: "Message introuvable.",
    })
}

async fn member_upload_donation_proof(
    State(state): State<WebState>,
    mut multipart: Multipart,
) -> Json<MessageResponse> {
    let mut pseudo: Option<String> = None;
    let mut method: Option<String> = None;
    let mut code: Option<String> = None;
    let mut photo_filename: Option<String> = None;
    let mut photo_bytes: Option<Vec<u8>> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap_or_default().to_string();
        if field_name == "pseudo" {
            if let Ok(text) = field.text().await {
                pseudo = Some(text);
            }
            continue;
        }
        if field_name == "method" {
            if let Ok(text) = field.text().await {
                method = Some(text);
            }
            continue;
        }
        if field_name == "code" {
            if let Ok(text) = field.text().await {
                code = Some(text);
            }
            continue;
        }
        if field_name == "photo" {
            photo_filename = field.file_name().map(|s| s.to_string());
            if let Ok(bytes) = field.bytes().await {
                photo_bytes = Some(bytes.to_vec());
            }
        }
    }

    let Some(pseudo) = pseudo.map(|v| v.trim().to_string()).filter(|v| !v.is_empty()) else {
        return Json(MessageResponse {
            ok: false,
            message: "Pseudo manquant.".to_string(),
        });
    };
    let Some(method) = method.map(|v| v.trim().to_ascii_lowercase()).filter(|v| !v.is_empty()) else {
        return Json(MessageResponse {
            ok: false,
            message: "Methode donation manquante (crypto|pcs).".to_string(),
        });
    };
    let Some(code) = code.map(|v| v.trim().to_string()).filter(|v| !v.is_empty()) else {
        return Json(MessageResponse {
            ok: false,
            message: "Code/reference donation manquant.".to_string(),
        });
    };
    let Some(photo_bytes) = photo_bytes else {
        return Json(MessageResponse {
            ok: false,
            message: "Photo justificative manquante.".to_string(),
        });
    };

    if method != "crypto" && method != "pcs" {
        return Json(MessageResponse {
            ok: false,
            message: "Methode invalide. Utilise crypto ou pcs.".to_string(),
        });
    }

    let users = state.users.read().await;
    if !users.iter().any(|u| u.pseudo.eq_ignore_ascii_case(&pseudo)) {
        return Json(MessageResponse {
            ok: false,
            message: "Utilisateur introuvable.".to_string(),
        });
    }
    drop(users);

    let id = state.next_donation_id.fetch_add(1, Ordering::Relaxed);
    let mut donations = state.donation_proofs.write().await;
    donations.push(DonationProof {
        id,
        pseudo,
        method,
        code,
        photo_filename: photo_filename.clone(),
        photo_mime_type: guess_avatar_mime(photo_filename.as_deref()).to_string(),
        photo_bytes,
        reviewed: false,
        approved: false,
        created_at_epoch: now_epoch(),
    });

    Json(MessageResponse {
        ok: true,
        message: format!("Preuve donation envoyee (ID #{id})."),
    })
}

async fn member_donation_photo(
    Path(id): Path<u64>,
    State(state): State<WebState>,
) -> Response {
    let donations = state.donation_proofs.read().await;
    let Some(item) = donations.iter().find(|d| d.id == id) else {
        return (
            StatusCode::NOT_FOUND,
            [
                (header::CONTENT_TYPE, HeaderValue::from_static("text/plain; charset=utf-8")),
                (header::CACHE_CONTROL, HeaderValue::from_static("no-store")),
            ],
            "preuve donation introuvable",
        )
            .into_response();
    };

    let content_type = HeaderValue::from_str(&item.photo_mime_type)
        .unwrap_or_else(|_| HeaderValue::from_static("image/png"));
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, content_type),
            (header::CACHE_CONTROL, HeaderValue::from_static("no-store")),
        ],
        item.photo_bytes.clone(),
    )
        .into_response()
}

async fn member_donations(
    Query(query): Query<DonationListQuery>,
    State(state): State<WebState>,
) -> Json<Vec<DonationProofView>> {
    let pseudo = query.pseudo.trim();
    let donations = state.donation_proofs.read().await;
    let list = donations
        .iter()
        .filter(|d| d.pseudo.eq_ignore_ascii_case(pseudo))
        .map(|d| DonationProofView {
            id: d.id,
            pseudo: d.pseudo.clone(),
            method: d.method.clone(),
            code: d.code.clone(),
            photo_filename: d.photo_filename.clone(),
            reviewed: d.reviewed,
            approved: d.approved,
            created_at_epoch: d.created_at_epoch,
        })
        .collect();
    Json(list)
}

async fn admin_messages_all(State(state): State<WebState>) -> Json<Vec<MemberMessageView>> {
    let messages = state.member_messages.read().await;
    Json(
        messages
            .iter()
            .map(|m| MemberMessageView {
                id: m.id,
                from_pseudo: m.from_pseudo.clone(),
                to_pseudo: m.to_pseudo.clone(),
                subject: m.subject.clone(),
                body: m.body.clone(),
                is_read: m.is_read,
                created_at_epoch: m.created_at_epoch,
            })
            .collect(),
    )
}

async fn admin_message_reply(
    State(state): State<WebState>,
    Json(payload): Json<AdminMessageSendInput>,
) -> Json<ActionResponse> {
    let to_pseudo = payload.to_pseudo.trim().to_string();
    let subject = payload.subject.trim().to_string();
    let body = payload.body.trim().to_string();
    let from_pseudo = admin_pseudo_from_env();

    if to_pseudo.is_empty() || subject.is_empty() || body.is_empty() {
        return Json(ActionResponse {
            ok: false,
            message: "Destinataire, sujet et message sont requis.",
        });
    }

    if from_pseudo.eq_ignore_ascii_case(&to_pseudo) {
        return Json(ActionResponse {
            ok: false,
            message: "Impossible d'envoyer un message a toi-meme.",
        });
    }

    let users = state.users.read().await;
    let recipient_exists = users.iter().any(|u| u.pseudo.eq_ignore_ascii_case(&to_pseudo));
    if !recipient_exists {
        return Json(ActionResponse {
            ok: false,
            message: "Destinataire introuvable.",
        });
    }
    drop(users);

    let mut messages = state.member_messages.write().await;
    let id = state.next_message_id.fetch_add(1, Ordering::Relaxed);
    messages.push(MemberMessage {
        id,
        from_pseudo,
        to_pseudo,
        subject,
        body,
        is_read: false,
        created_at_epoch: now_epoch(),
    });

    Json(ActionResponse {
        ok: true,
        message: "Reponse envoyee.",
    })
}

async fn admin_donations_all(State(state): State<WebState>) -> Json<Vec<DonationProofView>> {
    let donations = state.donation_proofs.read().await;
    Json(
        donations
            .iter()
            .map(|d| DonationProofView {
                id: d.id,
                pseudo: d.pseudo.clone(),
                method: d.method.clone(),
                code: d.code.clone(),
                photo_filename: d.photo_filename.clone(),
                reviewed: d.reviewed,
                approved: d.approved,
                created_at_epoch: d.created_at_epoch,
            })
            .collect(),
    )
}

async fn admin_donation_review(
    Path(id): Path<u64>,
    State(state): State<WebState>,
    Json(payload): Json<DonationReviewInput>,
) -> Json<ActionResponse> {
    let mut donations = state.donation_proofs.write().await;
    if let Some(item) = donations.iter_mut().find(|d| d.id == id) {
        item.reviewed = true;
        item.approved = payload.approved;
        return Json(ActionResponse {
            ok: true,
            message: "Preuve donation moderee.",
        });
    }
    Json(ActionResponse {
        ok: false,
        message: "Preuve donation introuvable.",
    })
}

async fn member_update_password(
    State(state): State<WebState>,
    Json(payload): Json<PasswordUpdateInput>,
) -> Json<ActionResponse> {
    let pseudo = payload.pseudo.trim().to_string();
    if pseudo.is_empty() {
        return Json(ActionResponse {
            ok: false,
            message: "Pseudo manquant.",
        });
    }

    let new_password = payload.new_password.trim();
    if new_password.len() < 4 {
        return Json(ActionResponse {
            ok: false,
            message: "Nouveau mot de passe trop court.",
        });
    }

    let users = state.users.read().await;
    let Some(user_ref) = users.iter().find(|u| u.pseudo.eq_ignore_ascii_case(&pseudo)) else {
        return Json(ActionResponse {
            ok: false,
            message: "Utilisateur introuvable.",
        });
    };
    let must_change_password = user_ref.must_change_password;
    drop(users);

    let mut passwords = state.user_passwords.write().await;
    let pseudo_key = pseudo_key(&pseudo);
    if let Some(existing) = passwords.get(&pseudo_key) {
        if must_change_password {
            // First-login path: allow setting a fresh password without asking current one again.
        } else {
            if let Some(current_password) = payload.current_password.as_ref() {
                if existing != current_password {
                    return Json(ActionResponse {
                        ok: false,
                        message: "Mot de passe actuel invalide.",
                    });
                }
            } else {
                return Json(ActionResponse {
                    ok: false,
                    message: "Mot de passe actuel requis.",
                });
            }
        }
    }

    passwords.insert(pseudo_key, new_password.to_string());

    drop(passwords);

    let mut users = state.users.write().await;
    if let Some(user) = users.iter_mut().find(|u| u.pseudo.eq_ignore_ascii_case(&pseudo)) {
        user.must_change_password = false;
    }
    Json(ActionResponse {
        ok: true,
        message: "Mot de passe mis a jour.",
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
    passwords.remove(&pseudo_key(&pseudo));
    drop(passwords);

    let mut messages = state.member_messages.write().await;
    messages.retain(|m| {
        !m.from_pseudo.eq_ignore_ascii_case(&pseudo)
            && !m.to_pseudo.eq_ignore_ascii_case(&pseudo)
    });
    drop(messages);

    let mut donations = state.donation_proofs.write().await;
    donations.retain(|d| !d.pseudo.eq_ignore_ascii_case(&pseudo));

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
    let mut avatar_bytes: Option<Vec<u8>> = None;

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
                avatar_bytes = Some(bytes.to_vec());
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
        user.avatar_mime_type = Some(guess_avatar_mime(user.avatar_filename.as_deref()).to_string());
        user.avatar_bytes = avatar_bytes;
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

async fn member_avatar(
    Path(pseudo): Path<String>,
    State(state): State<WebState>,
) -> Response {
    let users = state.users.read().await;
    let Some(user) = users.iter().find(|u| u.pseudo.eq_ignore_ascii_case(&pseudo)) else {
        return (
            StatusCode::NOT_FOUND,
            [
                (header::CONTENT_TYPE, HeaderValue::from_static("text/plain; charset=utf-8")),
                (header::CACHE_CONTROL, HeaderValue::from_static("no-store")),
            ],
            "avatar introuvable",
        )
            .into_response();
    };

    let Some(bytes) = user.avatar_bytes.as_ref() else {
        return (
            StatusCode::NOT_FOUND,
            [
                (header::CONTENT_TYPE, HeaderValue::from_static("text/plain; charset=utf-8")),
                (header::CACHE_CONTROL, HeaderValue::from_static("no-store")),
            ],
            "avatar introuvable",
        )
            .into_response();
    };

    let mime = user.avatar_mime_type.as_deref().unwrap_or("image/png");
    let content_type = HeaderValue::from_str(mime).unwrap_or_else(|_| HeaderValue::from_static("image/png"));

    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, content_type),
            (header::CACHE_CONTROL, HeaderValue::from_static("no-store")),
        ],
        bytes.clone(),
    )
        .into_response()
}

async fn member_delete_avatar(
    Path(pseudo): Path<String>,
    State(state): State<WebState>,
) -> Json<ActionResponse> {
    let mut users = state.users.write().await;
    if let Some(user) = users
        .iter_mut()
        .find(|u| u.pseudo.eq_ignore_ascii_case(&pseudo))
    {
        user.avatar_filename = None;
        user.avatar_size_bytes = None;
        user.avatar_mime_type = None;
        user.avatar_bytes = None;
        return Json(ActionResponse {
            ok: true,
            message: "Avatar supprime.",
        });
    }

    Json(ActionResponse {
        ok: false,
        message: "Utilisateur introuvable.",
    })
}

// ============================================================================
// UTILITIES
// ============================================================================

fn pseudo_key(pseudo: &str) -> String {
    pseudo.trim().to_ascii_lowercase()
}

fn generate_temp_password() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect()
}

fn now_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn guess_avatar_mime(filename: Option<&str>) -> &'static str {
    match filename
        .and_then(|name| name.rsplit('.').next())
        .map(|ext| ext.to_ascii_lowercase())
        .as_deref()
    {
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("webp") => "image/webp",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        _ => "image/png",
    }
}

fn web_bind_addr() -> SocketAddr {
    std::env::var("WEB_BIND_ADDR")
        .ok()
        .and_then(|raw| raw.parse::<SocketAddr>().ok())
        .unwrap_or_else(|| SocketAddr::from(([0, 0, 0, 0], 3000)))
}

fn api_upstream_addr() -> String {
    std::env::var("REV0AUTH_API_UPSTREAM")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "127.0.0.1:8080".to_string())
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

fn admin_pseudo_from_env() -> String {
    std::env::var("ADMIN_DASH_PSEUDO")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "admin".to_string())
}

fn admin_seed_from_env() -> String {
    std::env::var("ADMIN_DASH_SEED")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "rev0auth-seed".to_string())
}

fn admin_totp_secret_from_env() -> Option<String> {
    std::env::var("ADMIN_DASH_TOTP_SECRET")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

fn decode_base32_secret(secret: &str) -> Option<Vec<u8>> {
    let normalized: String = secret
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .collect::<String>()
        .to_ascii_uppercase();
    BASE32_NOPAD.decode(normalized.as_bytes()).ok()
}

fn totp_code(secret: &[u8], epoch: u64, step_secs: u64, digits: u32) -> Option<u32> {
    type HmacSha1 = Hmac<Sha1>;
    let counter = epoch / step_secs;
    let mut counter_bytes = [0u8; 8];
    counter_bytes.copy_from_slice(&counter.to_be_bytes());

    let mut mac = HmacSha1::new_from_slice(secret).ok()?;
    mac.update(&counter_bytes);
    let result = mac.finalize().into_bytes();

    let offset = (result[19] & 0x0f) as usize;
    let binary = ((u32::from(result[offset]) & 0x7f) << 24)
        | (u32::from(result[offset + 1]) << 16)
        | (u32::from(result[offset + 2]) << 8)
        | u32::from(result[offset + 3]);

    Some(binary % 10u32.pow(digits))
}

fn verify_totp_code(secret_b32: &str, otp_input: &str, now_epoch_secs: u64) -> bool {
    if otp_input.len() != 6 || !otp_input.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let Some(secret) = decode_base32_secret(secret_b32) else {
        return false;
    };

    for drift in [-1_i64, 0, 1] {
        let ts = if drift < 0 {
            now_epoch_secs.saturating_sub(30)
        } else if drift > 0 {
            now_epoch_secs.saturating_add(30)
        } else {
            now_epoch_secs
        };
        if let Some(code) = totp_code(&secret, ts, 30, 6) {
            if format!("{code:06}") == otp_input {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::{totp_code, verify_totp_code};

    #[test]
    fn test_totp_validation_accepts_current_code() {
        let secret_b32 = "JBSWY3DPEHPK3PXP";
        let secret = data_encoding::BASE32_NOPAD
            .decode(secret_b32.as_bytes())
            .expect("valid b32 secret");
        let now = 1_700_000_000_u64;
        let code = totp_code(&secret, now, 30, 6).expect("code");
        assert!(verify_totp_code(secret_b32, &format!("{code:06}"), now));
    }

    #[test]
    fn test_totp_validation_rejects_wrong_code() {
        let secret_b32 = "JBSWY3DPEHPK3PXP";
        let now = 1_700_000_000_u64;
        assert!(!verify_totp_code(secret_b32, "000000", now));
    }
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
