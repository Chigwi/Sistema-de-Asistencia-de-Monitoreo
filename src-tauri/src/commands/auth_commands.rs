use tauri::State;
use crate::db::DbPool;
use crate::error::AppError;
use crate::services::auth_service;
use crate::session::{Session,SessionState};
use crate::db::run_db;
use crate::services::asistencia_service;
use crate::session::{require_login, Role};



//<'_> tell the program to keep this info/function alive guided by context

#[tauri::command]
pub async fn login(
    pool: State<'_, DbPool>,
    state: State<'_, SessionState>,
    cedula: String,
    password: String,
) -> Result<Session, AppError> {
    let session = run_db(pool.inner(), move |conn| {
        auth_service::login(conn, &cedula, &password)
    })
    .await?;

    // employees start their shift on login; admins don't have shifts
    if session.rol == Role::Empleado {
        let id = session.id_empleado;
        match run_db(pool.inner(), move |conn| asistencia_service::clock_in(conn, id)).await {
            Ok(_) | Err(AppError::AlreadyClockedIn(_)) => {}
            Err(e) => return Err(e),
        }
    }

    *state.0.lock().map_err(|_| AppError::Internal("session lock".into()))? = Some(session.clone());
    Ok(session)
}

#[tauri::command]
pub async fn logout(
    pool: State<'_, DbPool>,
    state: State<'_, SessionState>,
) -> Result<(), AppError> {
    if let Ok(session) = require_login(&state) {
        if session.rol == Role::Empleado {
            let id = session.id_empleado;
            match run_db(pool.inner(), move |conn| asistencia_service::clock_out(conn, id)).await {
                Ok(_) | Err(AppError::NotClockedIn(_)) => {}
                Err(e) => return Err(e),
            }
        }
    }

    *state.0.lock().map_err(|_| AppError::Internal("session lock".into()))? = None;
    Ok(())
}