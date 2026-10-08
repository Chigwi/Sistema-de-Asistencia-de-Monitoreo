use diesel::prelude::*;
use serde::{Deserialize,Serialize};
use crate::schema::empleado;


//readable entity
#[derive (Queryable, Selectable, Serialize)]
#[diesel(table_name = empleado)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Empleado{
    pub id_empleado: i32,
    pub rol_empleado: i32,
    pub nombre: String,
    pub cedula: String,
    pub contrasenna: String,
    pub horario_establecido: Option<i32>,
}

//insertable entity
#[derive(Insertable , Deserialize)]
#[diesel(table_name = empleado)]
pub struct NewEmpleado{
    pub rol_empleado: i32,
    pub nombre: String,
    pub cedula: String,
    pub contrasenna: String,
    pub horario_establecido: Option<i32>,
}
