use tauri::State;
use crate::db::DbPool;
use crate::error::AppError;
use crate::services::auth_service;
use crate::session::{Session,SessionState};



//<'_> tell the program to keep this info/function alive guided by context

#[tauri::command]
pub async fn login(
    pool: State<'_,DbPool>,
    state: State<'_,SessionState>,
    cedula: String,
    password:String,
)-> Result <Session, AppError> {
    let pool = pool.inner().clone();
    let session = tauri::async_runtime::spawn_blocking(move ||{
        let mut conn = pool.get()?;
        auth_service::login(&mut conn, &cedula, &password)
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))??;

    *state.0.lock().map_err(|_| AppError::Internal("session lock".into()))? = Some(session.clone());
    Ok(session)
}

#[tauri::command]
pub fn logout(state: State<'_, SessionState>) -> Result<(), AppError> {
    *state.0.lock().map_err(|_| AppError::Internal("session lock".into()))? = None;
    Ok(())
}
