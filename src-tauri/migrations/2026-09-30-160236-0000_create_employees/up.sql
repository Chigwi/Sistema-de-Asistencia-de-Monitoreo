-- Your SQL goes here
CREATE TABLE rol(
                    id_rol SERIAL PRIMARY KEY,
                    nombre_rol varchar(50) NOT NULL UNIQUE
);

INSERT INTO rol (nombre_rol) VALUES ('Admin'), ('Empleado');

CREATE TABLE horario(
                        id_horario SERIAL PRIMARY KEY,
                        hora_inicio TIME(0) NOT NULL,
                        hora_salida TIME(0) NOT NULL,
                        tiempo_descanso integer NOT NULL DEFAULT 90,
                        minutos_establecidas integer NOT NULL
);


CREATE TABLE empleado (
                          id_empleado SERIAL PRIMARY KEY,
                          rol_empleado integer NOT NULL REFERENCES rol (id_rol),
                          nombre varchar(100) NOT NULL,
                          cedula varchar(50) NOT NULL,
                          contrasenna varchar(255) NOT NULL,
                          horario_establecido integer REFERENCES horario (id_horario),
                          en_descanso boolean NOT NULL DEFAULT false
);

CREATE TABLE historial_horario(
                                  id_historial_empleado SERIAL PRIMARY KEY,
                                  empleado integer NOT NULL REFERENCES empleado (id_empleado),
                                  hora_inicio TIME(0) NOT NULL,
                                  hora_salida TIME(0),
                                  minutos_trabajadas integer,
                                  minutos_establecidas integer NOT NULL,
                                  minutos_extras integer,
                                  notas text,
                                  fecha DATE NOT NULL
);

CREATE TABLE historial_descanso(
                                 id_historial_descanso SERIAL PRIMARY KEY,
                                 historial_horario integer NOT NULL REFERENCES historial_horario (id_historial_empleado),
                                 hora_inicio TIME NOT NULL,
                                 hora_salida TIME
);
