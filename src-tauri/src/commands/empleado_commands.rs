use tauri::State;

use crate::db::{run_db, DbPool};
use crate::error::AppError;
use crate::models::empleado::{Empleado, RegisterEmpleado};
use crate::services::{auth_service, empleado_service};
use crate::session::{require_admin, SessionState};

#[tauri::command]
pub async fn register_employee(
    pool: State<'_, DbPool>,
    state: State<'_, SessionState>,
    data: RegisterEmpleado,
) -> Result<Empleado, AppError> {
    require_admin(&state)?;
    run_db(pool.inner(), move |conn| empleado_service::register(conn, data)).await
}

#[tauri::command]
pub async fn list_employees(
    pool: State<'_, DbPool>,
    state: State<'_, SessionState>,
) -> Result<Vec<Empleado>, AppError> {
    require_admin(&state)?;
    run_db(pool.inner(), empleado_service::list_employees).await
}

#[tauri::command]
pub async fn reset_employee_password(
    pool: State<'_, DbPool>,
    state: State<'_, SessionState>,
    id_empleado: i32,
    new_password: String,
) -> Result<(), AppError> {
    require_admin(&state)?;
    run_db(pool.inner(), move |conn| {
        auth_service::reset_password(conn, id_empleado, &new_password)
    })
    .await
}