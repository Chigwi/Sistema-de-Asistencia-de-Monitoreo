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
use crate::error::AppError;
use crate::repositories::{empleado_repo, rol_repo};
use crate::session::{Role, Session};


pub fn hash_password(plain: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(plain.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(hash.to_string())
}

pub fn verify_password(plain: &str, stored: &str) -> bool {
    let Ok(parsed) = PasswordHash::new(stored) else {
        return false;
    };
    Argon2::default()
        .verify_password(plain.as_bytes(), &parsed)
        .is_ok()
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