use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};

use crate::services::audit_log_service;

use super::errors::AppResult;

pub fn get_audit_log_routes() -> Router {
    Router::new().route("/", get(get_logs))
}

async fn get_logs() -> AppResult<impl IntoResponse> {
    let logs = audit_log_service::get_logs(1).await?;

    return Ok((StatusCode::OK, Json(logs)).into_response());
}

#[derive(serde::Serialize)]
struct AuditLogs {
    logs: Vec<audit_log_service::AuditLogEntry>,
}

