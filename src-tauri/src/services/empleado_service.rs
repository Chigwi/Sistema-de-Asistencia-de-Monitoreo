use diesel::prelude::*;

use crate::error::AppError;
use crate::models::empleado::{Empleado, NewEmpleado, RegisterEmpleado};
use crate::repositories::{empleado_repo, horario_repo, rol_repo};
use crate::services::auth_service;

pub fn register(conn: &mut PgConnection, data: RegisterEmpleado) -> Result<Empleado, AppError> {
    let nombre = data.nombre.trim().to_string();
    let cedula = data.cedula.trim().to_string();

    if nombre.is_empty() || cedula.is_empty() {
        return Err(AppError::Validation("Nombre y cédula son obligatorios".into()));
    }
    if data.contrasena.len() < 8 {
        return Err(AppError::Validation("Contraseña muy corta".into()));
    }
    if empleado_repo::find_by_cedula(conn, &cedula)?.is_some() {
        return Err(AppError::Validation("Ya existe un empleado con esa cédula".into()));
    }
    if rol_repo::find_by_id(conn, data.rol_empleado)?.is_none() {
        return Err(AppError::Validation("El rol no existe".into()));
    }
    if let Some(id) = data.horario_establecido {
        if horario_repo::find_by_id(conn, id)?.is_none() {
            return Err(AppError::Validation("El horario no existe".into()));
        }
    }

    let new = NewEmpleado {
        rol_empleado: data.rol_empleado,
        nombre,
        cedula,
        contrasenna: auth_service::hash_password(&data.contrasena)?,
        horario_establecido: data.horario_establecido,
    };
    Ok(empleado_repo::insert(conn, &new)?)
}

pub fn list_employees(conn: &mut PgConnection) -> Result<Vec<Empleado>, AppError> {
    Ok(empleado_repo::find_all(conn)?)
}