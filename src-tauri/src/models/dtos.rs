use diesel::prelude::*;
use serde::{Deserialize,Serialize};
use chrono::{NaiveTime,NaiveDate};


#[derive (Queryable, Serialize)]
pub struct EmpleadoActivo{
    pub nombre: String,
    pub hora_inicio: NaiveTime,
}

#[derive(Queryable, Serialize)]
pub struct EmpleadoInactivo {
    pub nombre: String,
    pub fecha: NaiveDate,
    pub hora_inicio: NaiveTime,
    pub hora_salida: Option<NaiveTime>,
}