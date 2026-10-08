use std::sync::Mutex;

#[derive(Clone, Copy, PartialEq)]
pub enum Role {
    Admin,
    Empleado,
}

#[derive(Clone)]
pub struct Session {
    pub id_empleado: i32,
    pub rol: Role,
}

pub struct SessionState(pub Mutex<Option<Session>>);

