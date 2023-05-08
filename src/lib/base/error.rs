#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Todo {0}")]
    Todo(String), // Placeholder for new errors

    // Specific errors added here - BadRequest is an example
    #[error("BadRequest {0} [{1}]")]
    BadRequest(i32, String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}
