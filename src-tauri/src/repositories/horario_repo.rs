use diesel::prelude::*;
use crate::models::horario::{Horario, NewHorario};
use crate::schema::horario;


//create
pub fn insert (conn: &mut PgConnection, new: &NewHorario) -> QueryResult<Horario>{
    diesel::insert_into(horario::table)
        .values(new)
        .returning(Horario::as_returning())
        .get_result(conn)
}

//read all
pub fn find_all(conn: &mut PgConnection) -> QueryResult<Vec<Horario>> {
    horario::table.select(Horario::as_select()).load(conn)
}

//read by id
pub fn find_by_id(conn: &mut PgConnection, id: i32) -> QueryResult<Option<Horario>> {
    horario::table
        .find(id)
        .select(horario::as_select())
        .first(conn).optional()
}

//delete by id
pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> QueryResult<usize> {
    diesel::delete(Horario::table.find(id)).execute(conn)
}


