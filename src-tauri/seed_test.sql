-- 20 test employees: cedulas 9001..9020, names "Empleado 01".."Empleado 20"
-- They get the ADMIN's password hash, so you can log in as any of them with the admin password.
-- Odd numbers get schedule 1 (diurna), even numbers schedule 2 (nocturna).
INSERT INTO empleado (rol_empleado, nombre, cedula, contrasenna, horario_establecido)
SELECT 2,
       'Empleado ' || LPAD(n::text, 2, '0'),
       '90' || LPAD(n::text, 2, '0'),
       (SELECT contrasenna FROM empleado WHERE rol_empleado = 1 LIMIT 1),
       CASE WHEN n % 2 = 1 THEN 1 ELSE 2 END
FROM generate_series(1, 20) AS n;

-- Open shifts for 9001..9008 (hora_salida stays NULL = currently logged in)
INSERT INTO historial_horario (empleado, fecha, hora_inicio, minutos_establecidas)
SELECT id_empleado,
       CURRENT_DATE,
       CASE WHEN horario_establecido = 1 THEN TIME '06:00' ELSE TIME '12:00' END,
       480
FROM empleado
WHERE cedula BETWEEN '9001' AND '9008';

-- 9001 and 9003 are on a break right now (open break row + flag, kept consistent)
INSERT INTO historial_descanso (historial_horario, hora_inicio)
SELECT h.id_historial_empleado, TIME '09:00'
FROM historial_horario h
JOIN empleado e ON e.id_empleado = h.empleado
WHERE e.cedula IN ('9001', '9003') AND h.hora_salida IS NULL;

UPDATE empleado SET en_descanso = true WHERE cedula IN ('9001', '9003');

-- Closed shifts from yesterday for 9009..9012, so "Ultima coneccion" has data
INSERT INTO historial_horario
    (empleado, fecha, hora_inicio, hora_salida,
     minutos_trabajadas, minutos_descanso, minutos_extras, minutos_establecidas)
SELECT id_empleado,
       CURRENT_DATE - 1,
       CASE WHEN horario_establecido = 1 THEN TIME '06:00' ELSE TIME '12:00' END,
       CASE WHEN horario_establecido = 1 THEN TIME '14:00' ELSE TIME '20:00' END,
       480, 60, 0, 480
FROM empleado
WHERE cedula BETWEEN '9009' AND '9012';

