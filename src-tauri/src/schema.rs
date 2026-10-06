// @generated automatically by Diesel CLI.

diesel::table! {
    empleado (id_empleado) {
        id_empleado -> Int4,
        rol_empleado -> Int4,
        #[max_length = 100]
        nombre -> Varchar,
        #[max_length = 50]
        cedula -> Varchar,
        #[sql_name = "contraseña"]
        #[max_length = 50]
        contrase_a -> Varchar,
        logged_in -> Nullable<Bool>,
        horario_establecido -> Nullable<Int4>,
    }
}

diesel::table! {
    historial_horario (id_historial_empleado) {
        id_historial_empleado -> Int4,
        empleado -> Int4,
        hora_inicio -> Time,
        hora_salida -> Time,
        horas_trabajadas -> Nullable<Int4>,
        horas_establecidas -> Int4,
        horas_extras -> Nullable<Bool>,
        notas -> Nullable<Text>,
        fecha -> Date,
    }
}

diesel::table! {
    horario (id_horario) {
        id_horario -> Int4,
        hora_inicio -> Time,
        hora_salida -> Time,
        tiempo_descasnso -> Int4,
        horas_establecidas -> Int4,
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
diesel::joinable!(historial_horario -> empleado (empleado));

diesel::allow_tables_to_appear_in_same_query!(empleado, historial_horario, horario, rol,);
