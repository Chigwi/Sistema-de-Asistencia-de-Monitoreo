DELETE FROM historial_descanso
WHERE historial_horario IN (
    SELECT h.id_historial_empleado FROM historial_horario h
    JOIN empleado e ON e.id_empleado = h.empleado WHERE e.cedula LIKE '90__');
DELETE FROM historial_horario
WHERE empleado IN (SELECT id_empleado FROM empleado WHERE cedula LIKE '90__');
DELETE FROM empleado WHERE cedula LIKE '90__';