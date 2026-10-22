use std::time::Duration;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::error::AppError;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> DbPool {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(url);

    Pool::builder()
        .max_size(5)
        .connection_timeout(Duration::from_secs(5))
        .build(manager)
        .expect("Failed to create the database pool")
}


pub async fn run_db<T, F>(pool: &DbPool, f: F) -> Result<T, AppError>
where
    T: Send + 'static,
    F: FnOnce(&mut PgConnection) -> Result<T, AppError> + Send + 'static,
{
    let pool = pool.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let mut conn = pool.get()?;
        f(&mut conn)
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
}