use diesel::prelude::*;
use serde::{Deserialize,Serialize};
use chrono::{NaiveTime,NaiveDate};


#[derive(Queryable, Serialize)]
pub struct EmpleadoActivo {
    pub id_empleado: i32,
    pub nombre: String,
    pub hora_inicio: NaiveTime,
}

#[derive(Queryable, Serialize)]
pub struct EmpleadoInactivo {
    pub id_empleado: i32,
    pub nombre: String,
    pub fecha: NaiveDate,
    pub hora_inicio: NaiveTime,
    pub hora_salida: Option<NaiveTime>,
}

#[derive(Serialize)]
pub struct EstadoActual {
    pub turno_abierto: bool,
    pub en_descanso: bool,
    pub descanso_inicio: Option<NaiveTime>,  // when the running break started
    pub descanso_usado: i32,                 // minutes on finished breaks
    pub descanso_permitido: i32,             // the schedule's allowance
}