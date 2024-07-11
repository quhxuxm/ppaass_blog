use migration::sea_orm::TransactionError;
use sea_orm::error::DbErr;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum DaoError {
    #[error("User not found by username: {0}")]
    UserNotFoundByUsername(String),
    #[error("Blog not found by token: {0}")]
    BlogNotFoundByToken(String),
    #[error("Blog owner not found by blog token: {0}")]
    BlogOwnerNotFoundByBlogToken(String),
    #[error("Post not found by token: {0}")]
    PostNotFoundByToken(String),
    #[error(transparent)]
    DatabaseTransaction(#[from] TransactionError<DbErr>),
    #[error(transparent)]
    DatabaseGeneral(#[from] DbErr),
    #[error("Database configuration error happen: {0}")]
    DatabaseConfiguration(String),
}

impl From<DaoError> for DbErr {
    fn from(value: DaoError) -> Self {
        DbErr::Custom(format!("Database error happen: {value}"))
    }
}
