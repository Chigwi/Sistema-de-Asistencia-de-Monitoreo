//argon hashing
use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};
//diesel
use diesel::prelude::*;
//local
use crate::error::AppError::{self, InvalidCredentials};
use crate::repositories::{empleado_repo, rol_repo};
use crate::session::{Role, Session};

//hashing of passwords using argon2
pub fn hash_password(plain: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(plain.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(hash.to_string())
}

//password verification using argon2
pub fn verify_password(plain: &str, stored: &str) -> bool {
    let Ok(parsed) = PasswordHash::new(stored) else {
        return false;
    };
    Argon2::default()
        .verify_password(plain.as_bytes(), &parsed)
        .is_ok()
}

//role name string to enum conversion
fn role_from_name(name: &str) -> Result<Role, AppError>{
    match name{
        "Admin" => Ok(Role::Admin),
        "Empleado" => Ok(Role::Empleado),
        other => Err(AppError::Internal(format!("Rol desconocido en base de datos: {other}")))
    }
}

//login
pub fn login(conn: &mut PgConnection, cedula: &str, password: &str) -> Result<Session, AppError> {
    let Some(emp) = empleado_repo::find_by_cedula(conn, cedula)? else {
        return Err(AppError::InvalidCredentials);
    };

    if !verify_password(password, &emp.contrasenna) {
        return Err(AppError::InvalidCredentials);
    }

    let emp_rol = rol_repo::find_by_id(conn, emp.rol_empleado)?
        .ok_or(AppError::Internal("Employee role not found".into()))?;
    let rol = role_from_name(&emp_rol.nombre_rol)?;

    Ok(Session { id_empleado: emp.id_empleado, rol })
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_then_verify() {
        let hash = hash_password("secret123").unwrap();
        assert!(verify_password("secret123", &hash));
        assert!(!verify_password("wrong", &hash));
        assert!(!verify_password("secret123", "not-a-real-hash"));
    }
}