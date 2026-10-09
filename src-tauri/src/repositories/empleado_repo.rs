use diesel::prelude::*;
use crate::models::empleado::{Empleado, NewEmpleado, UpdateEmpleado};
use crate::schema::empleado;


//create
pub fn insert (conn: &mut PgConnection, new: &NewEmpleado) -> QueryResult<Empleado>{
    diesel::insert_into(empleado::table)
        .values(new)
        .returning(Empleado::as_returning())
        .get_result(conn)
}

//read all
pub fn find_all(conn: &mut PgConnection) -> QueryResult<Vec<Empleado>> {
    empleado::table.select(Empleado::as_select()).load(conn)
}

//read by id
pub fn find_by_id(conn: &mut PgConnection, id: i32) -> QueryResult<Option<Empleado>> {
    empleado::table
        .find(id)
        .select(Empleado::as_select())
        .first(conn).optional()
}

//find by cedula
pub fn find_by_cedula(conn: &mut PgConnection, cedula: &str) -> QueryResult<Option<Empleado>>{
    empleado::table
        .filter(empleado::cedula.eq(cedula))
        .select(Empleado::as_select())
        .first(conn).optional()

}

//update
pub fn update(conn: &mut PgConnection, id: i32, changes: &UpdateEmpleado) -> QueryResult<Empleado> {
    diesel::update(empleado::table.find(id))
        .set(changes)
        .returning(Empleado::as_returning())
        .get_result(conn)
}


//delete by id
pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> QueryResult<usize> {
    diesel::delete(empleado::table.find(id)).execute(conn)
}

//set on break
pub fn set_on_break(conn: &mut PgConnection, id: i32, on_break: bool) -> QueryResult<Empleado> {
    diesel::update(empleado::table.find(id))
        .set(empleado::en_descanso.eq(on_break))
        .returning(Empleado::as_returning())
        .get_result(conn)
}

//find employees on break
pub fn find_on_break(conn: &mut PgConnection) -> QueryResult<Vec<Empleado>> {
    empleado::table
        .filter(empleado::en_descanso.eq(true))
        .select(Empleado::as_select())
        .load(conn) 
}



//count employees
pub fn count_all(conn: &mut PgConnection) -> QueryResult<i64> {
    empleado::table.count().get_result(conn)
}

//change password
pub fn set_password(conn: &mut PgConnection, id: i32, hash: &str) -> QueryResult<usize>{
    diesel::update(empleado::table.find(id))
        .set(empleado::contrasenna.eq(hash))
        .execute(conn)
}

