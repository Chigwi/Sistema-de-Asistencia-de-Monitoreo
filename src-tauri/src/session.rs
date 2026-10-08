use std::sync::Mutex;
use crate::error::AppError;

#[derive(Clone, Copy, PartialEq)]
pub enum Role {
    Admin,
    Empleado,
}

#[derive(Clone)]
pub struct Session {
    pub id_empleado: i32,
    pub rol: Role,
}

pub struct SessionState(pub Mutex<Option<Session>>);

pub fn require_login(state: &SessionState) -> Result<Session, AppError> {
   let guard= state
            .0
            .lock()
            .map_err(|_| AppError::Internal("Failed to acquire session lock".into()))?;
    guard.clone().ok_or(AppError::Unauthenticated)
}

pub fn require_admin(state: &SessionState)-> Result<Session, AppError> {
    let session = require_login(state)?;

    if session.rol != Role::Admin {
        return Err(AppError::Forbidden);
    }
    Ok(session)
}

