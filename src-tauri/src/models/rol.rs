use diesel::prelude::*;
use serde::{Deserialize,Serialize};
use crate::schema::rol;


#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = rol)]
#[diesel(check_for_backend(diesel::pg::pg))]
pub struct Rol {
    pub id_rol: i32,
    pub nombre_rol: String,
}

