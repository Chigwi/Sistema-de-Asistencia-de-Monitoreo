use diesel::prelude::*;
use serde::{Deserialize,Serialize};
use crate::schema::rol;


//readable entity
#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = rol)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Rol {
    pub id_rol: i32,
    pub nombre_rol: String,
}

//insertable entity
#[derive(Insertable , Deserialize)]
#[diesel(table_name = rol)]
pub struct NewRol {
    pub nombre_rol: String,
}