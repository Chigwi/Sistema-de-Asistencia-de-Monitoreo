use chrono::NaiveDate;
use tauri::State;

use crate::db::{run_db, DbPool};
use crate::error::AppError;
use crate::models::dtos::{EmpleadoActivo, EmpleadoInactivo, EstadoActual};
use crate::models::historial_descanso::HistorialDescanso;
use crate::models::historial_horario::HistorialHorario;
use crate::services::asistencia_service;
use crate::models::empleado::Empleado;
use crate::session::{require_admin, require_login, SessionState};

#[tauri::command]
pub async fn clock_in(
    pool: State<'_, DbPool>,
    state: State<'_, SessionState>,
) -> Result<HistorialHorario, AppError> {
    let id = require_login(&state)?.id_empleado;
    run_db(pool.inner(), move |conn| asistencia_service::clock_in(conn, id)).await
}

#[tauri::command]
pub async fn clock_out(
    pool: State<'_, DbPool>,
    state: State<'_, SessionState>,
) -> Result<HistorialHorario, AppError> {
    let id = require_login(&state)?.id_empleado;
    run_db(pool.inner(), move |conn| asistencia_service::clock_out(conn, id)).await
}

#[tauri::command]
pub async fn start_break(
    pool: State<'_, DbPool>,
    state: State<'_, SessionState>,
) -> Result<HistorialDescanso, AppError> {
    let id = require_login(&state)?.id_empleado;
    run_db(pool.inner(), move |conn| asistencia_service::start_break(conn, id)).await
}

#[tauri::command]
pub async fn end_break(
    pool: State<'_, DbPool>,
    state: State<'_, SessionState>,
) -> Result<HistorialDescanso, AppError> {
    let id = require_login(&state)?.id_empleado;
    run_db(pool.inner(), move |conn| asistencia_service::end_break(conn, id)).await
}

#[tauri::command]
pub async fn my_history(
    pool: State<'_, DbPool>,
    state: State<'_, SessionState>,
    from: NaiveDate,
    to: NaiveDate,
) -> Result<Vec<HistorialHorario>, AppError> {
    let id = require_login(&state)?.id_empleado;
    run_db(pool.inner(), move |conn| asistencia_service::history(conn, id, from, to)).await
}

#[tauri::command]
pub async fn employee_history(
    pool: State<'_, DbPool>,
    state: State<'_, SessionState>,
    id_empleado: i32,
    from: NaiveDate,
    to: NaiveDate,
) -> Result<Vec<HistorialHorario>, AppError> {
    require_admin(&state)?;
    run_db(pool.inner(), move |conn| asistencia_service::history(conn, id_empleado, from, to)).await
}

#[tauri::command]
pub async fn logged_in_list(
    pool: State<'_, DbPool>,
    state: State<'_, SessionState>,
) -> Result<Vec<EmpleadoActivo>, AppError> {
    require_admin(&state)?;
    run_db(pool.inner(), asistencia_service::logged_in_list).await
}

#[tauri::command]
pub async fn logged_out_list(
    pool: State<'_, DbPool>,
    state: State<'_, SessionState>,
) -> Result<Vec<EmpleadoInactivo>, AppError> {
    require_admin(&state)?;
    run_db(pool.inner(), asistencia_service::logged_out_list).await
}

#[tauri::command]
pub async fn current_status(
    pool: State<'_, DbPool>,
    state: State<'_, SessionState>,
) -> Result<EstadoActual, AppError> {
    let id = require_login(&state)?.id_empleado;
    run_db(pool.inner(), move |conn| asistencia_service::current_status(conn, id)).await
}

#[tauri::command]
pub async fn on_break_list(
    pool: State<'_, DbPool>,
    state: State<'_, SessionState>,
) -> Result<Vec<Empleado>, AppError> {
    require_admin(&state)?;
    run_db(pool.inner(), asistencia_service::on_break_list).await
}