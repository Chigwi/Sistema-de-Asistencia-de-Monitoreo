use diesel::prelude::*;
use chrono::NaiveTime;
use crate::models::historial_descanso::{HistorialDescanso, StartBreak};
use crate::schema::historial_descanso;

//create
pub fn insert_start(conn: &mut PgConnection, new: &StartBreak) -> QueryResult<HistorialDescanso>{
    diesel::insert_into(historial_descanso::table)
        .values(new)
        .returning(HistorialDescanso::as_returning())
        .get_result(conn)
}

//find open break
pub fn find_open_break(conn: &mut PgConnection, id_historial: i32) -> QueryResult<Option<HistorialDescanso>> {
    historial_descanso::table
        .filter(historial_descanso::historial_horario.eq(id_historial))
        .filter(historial_descanso::hora_salida.is_null())
        .order(historial_descanso::hora_inicio.asc())
        .select(HistorialDescanso::as_select())
        .first(conn).optional()
}

//find the breaks on a specific shift
pub fn find_breaks_by_shift(conn: &mut PgConnection, id_historial: i32) -> QueryResult<Vec<HistorialDescanso>> {
    historial_descanso::table
        .filter(historial_descanso::historial_horario.eq(id_historial))
        .select(HistorialDescanso::as_select())
        .load(conn)
}

//end break
pub fn end_break(conn: &mut PgConnection, id: i32, hora_salida: NaiveTime) -> QueryResult<HistorialDescanso> {
    diesel::update(
        historial_descanso::table
            .filter(historial_descanso::id_historial_descanso.eq(id))
            .filter(historial_descanso::hora_salida.is_null()),
    )
    .set(historial_descanso::hora_salida.eq(hora_salida))
    .returning(HistorialDescanso::as_returning())
    .get_result(conn)
}
