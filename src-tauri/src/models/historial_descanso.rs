use diesel::prelude::*;
use serde::{Serialize};
use chrono::NaiveTime;
use crate::schema::historial_descanso;




//readable entity
#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = historial_descanso)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct HistorialDescanso{
    pub id_historial_descanso: i32,
    pub historial_horario: i32,
    pub hora_inicio: NaiveTime,
    pub hora_salida: Option<NaiveTime>
}

//startbreak entity
#[derive(Insertable)]
#[diesel(table_name = historial_descanso)]
pub struct StartBreak{
    pub historial_horario: i32,
    pub hora_inicio: NaiveTime
}






