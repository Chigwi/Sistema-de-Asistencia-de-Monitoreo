use diesel::prelude::*;
use serde::{Deserialize,Serialize};
use chrono::NaiveTime;
use crate::schema::historial_horario;



//readable entity
#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name= historial_horario)]
#diesel(check_for_backend(diesel::pg::Pg))
pub struct HistorialHorario{
    pub id_historial_empleado: i32,
    pub empleado: i32,
    pub hora_inicio: NaiveTime,
    pub hora_salida: NaiveTime,
    pub minutos_trabajados: i32,
    pub minutos_establecidas: i32,
    pub minutos_extras: i32,
    pub notas: String,
    pub fecha: NaiveTime,
}

//insertable entity
#[derive(Insertable, Deserialize)]
#[diesel(table_name = historial_horario)]
pub struct NewHistorialHorario{
    pub empleado: i32,
    pub hora_inicio: NaiveTime,
    pub hora_salida: NaiveTime,
    pub minutos_trabajados: i32,
    pub minutos_establecidas: i32,
    pub minutos_extras: i32,
    pub notas: String,
    pub fecha: NaiveTime,
}