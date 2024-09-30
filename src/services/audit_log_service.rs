use crate::database::dao::audit_log_dao;

use super::errors::ServiceResult;

pub async fn add_parent_history_entry(
    entity_name: String,
    old_parent_name: Option<String>,
    new_parent_name: Option<String>,
) -> ServiceResult<()> {
    audit_log_dao::create_parent_history_entry(1, entity_name, old_parent_name, new_parent_name).await?;

    return Ok(());
}

pub async fn get_parent_history(user_id: u32) -> ServiceResult<Vec<HistoryEntry>> {
    let history = audit_log_dao::get_parent_history(user_id, 1000).await?;

    return Ok(history);
}

pub type HistoryEntry = audit_log_dao::ParentHistoryTable;
