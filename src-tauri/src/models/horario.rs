use diesel::prelude::*;
use serde::{Deserialize,Serialize};
use chrono::NaiveTime;
use crate::schema::horario;



//readable entity
#[derive (Queryable, Selectable, Serialize)]
#[diesel (table_name = horario)]
#[diesel (check_for_backend(diesel::pg::Pg))]
pub struct Horario{
    pub id_horario: i32,
    pub hora_inicio: NaiveTime,
    pub hora_salida: NaiveTime,
    pub tiempo_descanso: i32,
    pub minutos_establecidas: i32,
}

//insertable entity

#[derive(Insertable, Deserialize)]
#[diesel(table_name = horario)]
pub struct NewHorario{
    pub hora_inicio: NaiveTime,
    pub hora_salida: NaiveTime,
    pub tiempo_descanso: i32,
    pub minutos_establecidas: i32,
}