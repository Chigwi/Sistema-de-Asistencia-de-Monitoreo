use diesel::prelude::*;
use crate::models::rol::{Rol, NewRol};
use crate::schema::rol;


//create
pub fn insert (conn: &mut PgConnection, new: &NewRol) -> QueryResult<Rol>{
    diesel::insert_into(rol::table)
        .values(new)
        .returning(Rol::as_returning())
        .get_result(conn)
}

//read all
pub fn find_all(conn: &mut PgConnection) -> QueryResult<Vec<Rol>> {
    rol::table.select(Rol::as_select()).load(conn)
}

//read by id
pub fn find_by_id(conn: &mut PgConnection, id: i32) -> QueryResult<Option<Rol>> {
    rol::table
        .find(id)
        .select(Rol::as_select())
        .first(conn).optional()
}

//find by name
pub fn find_by_name(conn: &mut PgConnection, name: &str) -> QueryResult<Option<Rol>> {
    rol::table
        .filter(rol::nombre_rol.eq(name))
        .select(Rol::as_select())
        .first(conn).optional()
}

//delete by id
pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> QueryResult<usize> {
    diesel::delete(rol::table.find(id)).execute(conn)
}





