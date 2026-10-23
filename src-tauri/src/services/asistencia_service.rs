// services/asistencia_service.rs
use chrono::{Local, NaiveDate, NaiveTime};
use diesel::prelude::*;

use crate::error::AppError;
use crate::models::dtos::{EmpleadoActivo, EmpleadoInactivo};
use crate::models::historial_descanso::{HistorialDescanso, StartBreak};
use crate::models::historial_horario::{ClockIn, ClockOut, HistorialHorario};
use crate::repositories::{
    empleado_repo, historial_descanso_repo, historial_horario_repo as historial_repo, horario_repo,
};

// whole minutes between two times of day (never negative)
fn minutes_between(start: NaiveTime, end: NaiveTime) -> i32 {
    (end - start).num_minutes().max(0) as i32
}

pub fn clock_in(conn: &mut PgConnection, id_empleado: i32) -> Result<HistorialHorario, AppError> {
    conn.transaction::<_, AppError, _>(|conn| {
        let emp = empleado_repo::find_by_id(conn, id_empleado)?
            .ok_or(AppError::NotFound("Empleado".into()))?;

        if historial_repo::find_open_shift(conn, id_empleado)?.is_some() {
            return Err(AppError::AlreadyClockedIn(emp.nombre.clone()));
        }

        let horario_id = emp
            .horario_establecido
            .ok_or(AppError::Validation("El empleado no tiene un horario asignado".into()))?;
        let horario = horario_repo::find_by_id(conn, horario_id)?
            .ok_or(AppError::NotFound("Horario".into()))?;

        let now = Local::now();
        let shift = historial_repo::insert_clock_in(
            conn,
            &ClockIn {
                empleado: id_empleado,
                hora_inicio: now.time(),
                minutos_establecidas: horario.minutos_establecidas,
                fecha: now.date_naive(),
            },
        )?;
        Ok(shift)
    })
}

pub fn start_break(conn: &mut PgConnection, id_empleado: i32) -> Result<HistorialDescanso, AppError> {
    conn.transaction::<_, AppError, _>(|conn| {
        let emp = empleado_repo::find_by_id(conn, id_empleado)?
            .ok_or(AppError::NotFound("Empleado".into()))?;

        let shift = historial_repo::find_open_shift(conn, id_empleado)?
            .ok_or_else(|| AppError::NotClockedIn(emp.nombre.clone()))?;

        if historial_descanso_repo::find_open_break(conn, shift.id_historial_empleado)?.is_some() {
            return Err(AppError::AlreadyOnBreak(emp.nombre.clone()));
        }

        let brk = historial_descanso_repo::insert_start(
            conn,
            &StartBreak {
                historial_horario: shift.id_historial_empleado,
                hora_inicio: Local::now().time(),
            },
        )?;
        empleado_repo::set_on_break(conn, id_empleado, true)?;
        Ok(brk)
    })
}

pub fn end_break(conn: &mut PgConnection, id_empleado: i32) -> Result<HistorialDescanso, AppError> {
    conn.transaction::<_, AppError, _>(|conn| {
        let emp = empleado_repo::find_by_id(conn, id_empleado)?
            .ok_or(AppError::NotFound("Empleado".into()))?;

        let shift = historial_repo::find_open_shift(conn, id_empleado)?
            .ok_or_else(|| AppError::NotClockedIn(emp.nombre.clone()))?;

        let open = historial_descanso_repo::find_open_break(conn, shift.id_historial_empleado)?
            .ok_or_else(|| AppError::NotOnBreak(emp.nombre.clone()))?;

        let brk = historial_descanso_repo::end_break(conn, open.id_historial_descanso, Local::now().time())?;
        empleado_repo::set_on_break(conn, id_empleado, false)?;
        Ok(brk)
    })
}

pub fn clock_out(conn: &mut PgConnection, id_empleado: i32) -> Result<HistorialHorario, AppError> {
    conn.transaction::<_, AppError, _>(|conn| {
        let emp = empleado_repo::find_by_id(conn, id_empleado)?
            .ok_or(AppError::NotFound("Empleado".into()))?;

        let shift = historial_repo::find_open_shift(conn, id_empleado)?
            .ok_or_else(|| AppError::NotClockedIn(emp.nombre.clone()))?;

        if historial_descanso_repo::find_open_break(conn, shift.id_historial_empleado)?.is_some() {
            return Err(AppError::Validation("Debe terminar el descanso antes de salir".into()));
        }

        // total minutes spent on (finished) breaks during this shift
        let breaks = historial_descanso_repo::find_breaks_by_shift(conn, shift.id_historial_empleado)?;
        let break_total: i32 = breaks
            .iter()
            .filter_map(|b| b.hora_salida.map(|end| minutes_between(b.hora_inicio, end)))
            .sum();

        // free break allowance from the employee's schedule
        let allowance = match emp.horario_establecido {
            Some(id) => horario_repo::find_by_id(conn, id)?
                .map(|h| h.tiempo_descanso)
                .unwrap_or(0),
            None => 0,
        };

        let now = Local::now().time();
        let span = minutes_between(shift.hora_inicio, now);
        let excess = (break_total - allowance).max(0); // only the excess is deducted
        let worked = (span - excess).max(0);
        let extra = (worked - shift.minutos_establecidas).max(0);

        let closed = historial_repo::close_shift(
            conn,
            shift.id_historial_empleado,
            &ClockOut {
                hora_salida: now,
                minutos_trabajadas: worked,
                minutos_descanso: break_total,
                minutos_extras: extra,
            },
        )?;
        Ok(closed)
    })
}

pub fn history(
    conn: &mut PgConnection,
    id_empleado: i32,
    from: NaiveDate,
    to: NaiveDate,
) -> Result<Vec<HistorialHorario>, AppError> {
    if from > to {
        return Err(AppError::Validation("La fecha inicial no puede ser posterior a la final".into()));
    }
    Ok(historial_repo::find_by_employee_and_range(conn, id_empleado, from, to)?)
}

pub fn logged_in_list(conn: &mut PgConnection) -> Result<Vec<EmpleadoActivo>, AppError> {
    Ok(historial_repo::find_logged_in(conn)?)
}

pub fn logged_out_list(conn: &mut PgConnection) -> Result<Vec<EmpleadoInactivo>, AppError> {
    Ok(historial_repo::find_logged_out(conn)?)
}