use thiserror::Error;

#[derive(Error, Debug)]
pub enum RBACError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Role not found")]
    NotFound,
}
