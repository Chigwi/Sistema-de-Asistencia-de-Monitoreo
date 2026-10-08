// @generated automatically by Diesel CLI.

diesel::table! {
    empleado (id_empleado) {
        id_empleado -> Int4,
        rol_empleado -> Int4,
        #[max_length = 100]
        nombre -> Varchar,
        #[max_length = 50]
        cedula -> Varchar,
        #[max_length = 255]
        contrasenna -> Varchar,
        horario_establecido -> Nullable<Int4>,
        en_descanso -> Bool,
    }
}

diesel::table! {
    historial_descanso (id_historial_descanso) {
        id_historial_descanso -> Int4,
        historial_horario -> Int4,
        hora_inicio -> Time,
        hora_salida -> Nullable<Time>,
    }
}

diesel::table! {
    historial_horario (id_historial_empleado) {
        id_historial_empleado -> Int4,
        empleado -> Int4,
        hora_inicio -> Time,
        hora_salida -> Nullable<Time>,
        minutos_trabajadas -> Nullable<Int4>,
        minutos_establecidas -> Int4,
        minutos_extras -> Nullable<Int4>,
        notas -> Nullable<Text>,
        fecha -> Date,
    }
}

diesel::table! {
    horario (id_horario) {
        id_horario -> Int4,
        hora_inicio -> Time,
        hora_salida -> Time,
        tiempo_descanso -> Int4,
        minutos_establecidas -> Int4,
    }
}

diesel::table! {
    rol (id_rol) {
        id_rol -> Int4,
        #[max_length = 50]
        nombre_rol -> Varchar,
    }
}

diesel::joinable!(empleado -> horario (horario_establecido));
diesel::joinable!(empleado -> rol (rol_empleado));
diesel::joinable!(historial_descanso -> historial_horario (historial_horario));
diesel::joinable!(historial_horario -> empleado (empleado));

diesel::allow_tables_to_appear_in_same_query!(
    empleado,
    historial_descanso,
    historial_horario,
    horario,
    rol,
);
