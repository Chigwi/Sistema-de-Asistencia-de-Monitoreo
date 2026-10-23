UPDATE historial_descanso
SET hora_salida = LOCALTIME(0)
WHERE hora_salida IS NULL
  AND historial_horario IN (
      SELECT h.id_historial_empleado
      FROM historial_horario h
      JOIN empleado e ON e.id_empleado = h.empleado
      WHERE e.cedula LIKE '90__');

UPDATE empleado SET en_descanso = false WHERE cedula LIKE '90__';

UPDATE historial_horario h
SET hora_salida        = LOCALTIME(0),
    minutos_trabajadas = GREATEST(0, (EXTRACT(EPOCH FROM (LOCALTIME(0) - h.hora_inicio)) / 60)::int),
    minutos_descanso   = 0,
    minutos_extras     = 0
WHERE h.hora_salida IS NULL
  AND h.empleado IN (SELECT id_empleado FROM empleado WHERE cedula LIKE '90__');