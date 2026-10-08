use diesel::prelude::*;
use crate::models::empleado::{Empleado, NewEmpleado, RegisterEmpleado, UpdateEmpleado};
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

//delete by id
pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> QueryResult<usize> {
    diesel::delete(empleado::table.find(id)).execute(conn)
}
