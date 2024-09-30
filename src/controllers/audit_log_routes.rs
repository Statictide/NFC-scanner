use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};

use crate::services::audit_log_service;

use super::errors::AppResult;

pub fn get_history_routes() -> Router {
    Router::new().route("/", get(get_history))
}

async fn get_history() -> AppResult<impl IntoResponse> {
    let logs = audit_log_service::get_parent_history(1).await?;
    let parent_history = logs
        .into_iter()
        .map(ParentHistoryEntry::from_history)
        .collect::<Vec<_>>();

    let res: AuditLogs = AuditLogs { parent_history };
    return Ok((StatusCode::OK, Json(res)));
}

#[derive(serde::Serialize)]
struct AuditLogs {
    parent_history: Vec<ParentHistoryEntry>,
}

#[derive(serde::Serialize)]
pub struct ParentHistoryEntry {
    //parent_history_id: u32,
    entity_name: String,
    old_parent_name: Option<String>,
    new_parent_name: Option<String>,
    created_at: chrono::NaiveDateTime,
}

impl ParentHistoryEntry {
    fn from_history(entry: audit_log_service::HistoryEntry) -> Self {
        Self {
            //parent_history_id: entry.parent_history_id,
            entity_name: entry.entity_name,
            old_parent_name: entry.old_parent_name,
            new_parent_name: entry.new_parent_name,
            created_at: entry.created_at,
        }
    }
}
