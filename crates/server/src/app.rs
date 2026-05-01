use std::time::Duration;

use axum::{
    extract::DefaultBodyLimit,
    http::{HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use cryputil_core::{core::trace::Trace, dispatch};
use serde::{Deserialize, Serialize};
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_BODY_BYTES: usize = 32 * 1024;

#[derive(Deserialize)]
pub struct RunRequest {
    pub command: String,
    #[serde(default)]
    pub params: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct RunResponse {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<Trace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct CommandsResponse {
    pub commands: Vec<CommandView>,
}

#[derive(Serialize)]
pub struct CommandView {
    pub name: String,
    pub params: Vec<String>,
    pub description: String,
}

// Input: optionaler Origin für CORS
// Calc:  Router mit Routen, Body-Limit, Timeout, CORS aufbauen
// Output: konfigurierter axum::Router
pub fn router(allowed_origin: Option<String>) -> Router {
    let cors = match allowed_origin {
        Some(origin) => match origin.parse::<HeaderValue>() {
            Ok(h) => CorsLayer::new()
                .allow_origin(h)
                .allow_methods([Method::GET, Method::POST])
                .allow_headers(Any),
            Err(_) => CorsLayer::permissive(),
        },
        None => CorsLayer::permissive(),
    };

    Router::new()
        .route("/api/health", get(health))
        .route("/api/commands", get(commands))
        .route("/api/run", post(run))
        .layer(DefaultBodyLimit::max(MAX_BODY_BYTES))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            REQUEST_TIMEOUT,
        ))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}

async fn health() -> &'static str {
    "ok"
}

async fn commands() -> Json<CommandsResponse> {
    let list = dispatch::commands()
        .iter()
        .map(|c| CommandView {
            name: c.name.to_string(),
            params: c.params.iter().map(|s| s.to_string()).collect(),
            description: c.description.to_string(),
        })
        .collect();
    Json(CommandsResponse { commands: list })
}

async fn run(Json(req): Json<RunRequest>) -> impl IntoResponse {
    match dispatch::run(&req.command, &req.params) {
        Ok(trace) => (
            StatusCode::OK,
            Json(RunResponse {
                ok: true,
                trace: Some(trace),
                error: None,
            }),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(RunResponse {
                ok: false,
                trace: None,
                error: Some(err.to_string()),
            }),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{to_bytes, Body};
    use axum::http::Request;
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_returns_ok() {
        let app = router(None);
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/api/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn run_dispatches_command() {
        let app = router(None);
        let body = serde_json::json!({
            "command": "mod.add",
            "params": {"a": 7, "b": 5, "n": 12}
        })
        .to_string();
        let res = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/run")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let bytes = to_bytes(res.into_body(), MAX_BODY_BYTES).await.unwrap();
        let parsed: RunResponse = serde_json::from_slice(&bytes).unwrap();
        assert!(parsed.ok);
        assert!(parsed.trace.is_some());
    }

    #[tokio::test]
    async fn unknown_command_is_400() {
        let app = router(None);
        let body = serde_json::json!({"command": "nope", "params": {}}).to_string();
        let res = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/run")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }
}
