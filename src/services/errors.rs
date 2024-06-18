use crate::database::errors::DatabaseError;

pub type ServiceResult<T> = Result<T, ServiceError>;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("Not found")]
    NotFound,
    #[error("Internal server error: {0}")]
    InternalServerError(#[from] anyhow::Error),
}

impl From<DatabaseError> for ServiceError {
    fn from(error: DatabaseError) -> Self {
        match error {
            DatabaseError::NotFound => ServiceError::NotFound,
            DatabaseError::DatabaseError(error) => ServiceError::InternalServerError(error.into()),
            DatabaseError::InternalServerError(error) => ServiceError::InternalServerError(error),
        }
    }
}
