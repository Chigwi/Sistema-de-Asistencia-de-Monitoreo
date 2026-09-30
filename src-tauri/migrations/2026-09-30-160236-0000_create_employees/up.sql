-- Your SQL goes here
CREATE TABLE rol(
                    id_rol SERIAL PRIMARY KEY,
                    nombre_rol varchar(50) NOT NULL
);

CREATE TABLE horario(
                        id_horario SERIAL PRIMARY KEY,
                        hora_inicio TIME(0) NOT NULL,
                        hora_salida TIME(0) NOT NULL,
                        tiempo_descasnso integer NOT NULL,
                        horas_establecidas integer NOT NULL
);


CREATE TABLE empleado (
                          id_empleado SERIAL PRIMARY KEY,
                          rol_empleado integer REFERENCES rol (id_rol) NOT NULL,
                          nombre varchar(100) NOT NULL,
                          cedula varchar(50) NOT NULL,
                          contraseña varchar(50) NOT NULL,
                          logged_in boolean,
                          horario_establecido integer REFERENCES horario (id_horario)
);

CREATE TABLE historial_horario(
                                  id_historial_empleado SERIAL PRIMARY KEY,
                                  empleado integer REFERENCES empleado (id_empleado) NOT NULL,
                                  hora_inicio TIME(0) NOT NULL,
                                  hora_salida TIME(0) NOT NULL,
                                  horas_trabajadas integer,
                                  horas_establecidas integer NOT NULL,
                                  horas_extras boolean,
                                  notas text,
                                  fecha DATE NOT NULL
);
