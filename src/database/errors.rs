pub type DatabaseResult<T> = Result<T, DatabaseError>;

#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("Not found")]
    NotFound,
    #[error("Bad request: {0}")]
    DatabaseError(sqlx::Error),
    #[error("Internal server error: {0}")]
    InternalServerError(#[from] anyhow::Error),
}

impl From<sqlx::Error> for DatabaseError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => DatabaseError::NotFound,
            _ => DatabaseError::DatabaseError(error),
        }
    }
}
