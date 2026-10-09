use diesel::prelude::*;
use chrono::NaiveDate;
use crate::models::historial_horario::{HistorialHorario, ClockIn, ClockOut};
use crate::models::dtos::{EmpleadoActivo,EmpleadoInactivo};
use crate::schema::historial_horario;
use crate::schema::empleado;


//create
pub fn insert_clock_in(conn: &mut PgConnection, new: &ClockIn) -> QueryResult<HistorialHorario>{
    diesel::insert_into(historial_horario::table)
        .values(new)
        .returning(HistorialHorario::as_returning())
        .get_result(conn)
}

//find by id
pub fn find_by_id(conn: &mut PgConnection, id: i32) -> QueryResult<Option<HistorialHorario>> {
    historial_horario::table
        .filter(historial_horario::id_historial_empleado.eq(id))
        .select(HistorialHorario::as_select())
        .first(conn).optional()
}

//find by fecha

//find open shift
pub fn find_open_shift(conn: &mut PgConnection, id_usuario: i32) -> QueryResult<Option<HistorialHorario>> {
    historial_horario::table
        .filter(historial_horario::empleado.eq(id_usuario))
        .filter(historial_horario::hora_salida.is_null())
        .select(HistorialHorario::as_select())
        .first(conn).optional()
}

//find logged in employees
pub fn find_logged_in (conn: &mut PgConnection) -> QueryResult<Vec<EmpleadoActivo>>{
    historial_horario::table.inner_join(empleado::table)
        .filter(historial_horario::hora_salida.is_null())
        .order(
            historial_horario::hora_inicio.desc(),
        )
        .select((
            empleado::nombre,
            historial_horario::hora_inicio
        ))
        .load(conn)
}



//find logged out employees last shift
pub fn find_logged_out(conn: &mut PgConnection) -> QueryResult<Vec<EmpleadoInactivo>>{
    let latest: Vec<EmpleadoInactivo> = historial_horario::table
    .inner_join(empleado::table)
    .distinct_on(historial_horario::empleado)
    .order((
    historial_horario::empleado,
    historial_horario::fecha.desc(),
    historial_horario::hora_inicio.desc(),
     ))
    .select((
        empleado::nombre,
        historial_horario::fecha,
        historial_horario::hora_inicio,
        historial_horario::hora_salida
    ))
    .load(conn)?;

Ok(latest.into_iter().filter(|s| s.hora_salida.is_some()).collect())
}


//close shift
pub fn close_shift(conn: &mut PgConnection, id_historial: i32, clock_out: &ClockOut) -> QueryResult<HistorialHorario> {
    diesel::update(
        historial_horario::table
            .filter(historial_horario::id_historial_empleado.eq(id_historial))
            .filter(historial_horario::hora_salida.is_null()),
    )
    .set(clock_out)
    .returning(HistorialHorario::as_returning())
    .get_result(conn)
}

//find user and range
pub fn find_by_employee_and_range( conn: &mut PgConnection, id_empleado: i32, from: NaiveDate, to: NaiveDate,
) -> QueryResult<Vec<HistorialHorario>> {

    historial_horario::table
        .filter(historial_horario::empleado.eq(id_empleado))
        .filter(historial_horario::fecha.ge(from))
        .filter(historial_horario::fecha.le(to))
        .order((historial_horario::fecha.desc(), historial_horario::hora_inicio.desc()))
        .select(HistorialHorario::as_select())
        .load(conn)
}

