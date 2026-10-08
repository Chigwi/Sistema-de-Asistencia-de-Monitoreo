

//error enum

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    //diesel library errors
    #[error("Database error: {0}")]
    Db(#[from] diesel::result::Error),

    #[error("Could not get a database connection: {0}")]
    Pool(#[from] diesel::r2d2::PoolError),

    //custom errors
    #[error("{0}")]
    Validation(String),

    #[error("{0} is already clocked in")]
    AlreadyClockedIn(String),

    #[error("{0} is not clocked in")]
    NotClockedIn(String),

    #[error("{0} is already on break")]
    AlreadyOnBreak(String),

    #[error("{0} is not on break")]
    NotOnBreak(String),

    #[error("{0} not found")]
    NotFound(String),

    //internal errors
    #[error("Internal error: {0}")]
    Internal(String),
}


impl serde::Serialize for AppError {

    fn serialize<S: serde::Serializer> (&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }

}