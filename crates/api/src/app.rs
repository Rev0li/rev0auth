use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    service: &'static str,
    status: &'static str,
}

pub async fn build_router() -> anyhow::Result<Router> {
    Ok(Router::new().route("/health", get(health)))
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        service: "rev0auth-api",
        status: "ok",
    })
}

#[cfg(test)]
mod tests {
    use super::build_router;
    use axum::{body::Body, http::Request};
    use serde_json::Value;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_endpoint() {
        let app = build_router().await.expect("build router");

        let response = app
            .oneshot(
                Request::get("/health")
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");

        assert_eq!(response.status().as_u16(), 200);

        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("bytes");
        let body = serde_json::from_slice::<Value>(&bytes).expect("json");

        assert_eq!(body["service"], "rev0auth-api");
        assert_eq!(body["status"], "ok");
    }
}
