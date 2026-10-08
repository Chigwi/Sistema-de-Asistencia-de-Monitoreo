

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

    #[error("{0} ya se encuentra ingresado")]
    AlreadyClockedIn(String),

    #[error("{0} no se encuentra ingresado")]
    NotClockedIn(String),

    #[error("{0} ya se encuentra en descanso")]
    AlreadyOnBreak(String),

    #[error("{0} no se encuentra en descanso")]
    NotOnBreak(String),

    #[error("{0} not found")]
    NotFound(String),

    //session errors
    #[error("Credenciales invalidas")]
    InvalidCredentials,

    #[error("Permiso denegado")]
    Forbidden,

    #[error("No hay sesion activa")]
    Unauthenticated,

    //internal errors
    #[error("Internal error: {0}")]
    Internal(String),
}


impl serde::Serialize for AppError {

    fn serialize<S: serde::Serializer> (&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }

}