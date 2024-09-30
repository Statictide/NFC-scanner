use super::errors::ServiceResult;


pub async fn get_logs(user_id: u32) -> ServiceResult<Vec<AuditLogEntry>> {
    let logs = vec![
        AuditLogEntry {
            action: "Server1".to_string(),
        },
        AuditLogEntry {
            action: "Server2".to_string(),
        },
    ];

    return Ok(logs);
}


#[derive(serde::Serialize)]
pub struct AuditLogEntry {
    pub action: String,
}