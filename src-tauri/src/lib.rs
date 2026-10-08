use chrono::Local;
use diesel::prelude::*;
use std::sync::Mutex;
use tauri::State;

use crate::db::DbPool;
use crate::session::SessionState;

//module declarations
mod schema;

mod db;

mod models;

mod repositories;

mod services;

mod commands;

mod error;

mod session;



#[tauri::command]
fn greet(name: &str, pool: State<'_, DbPool>) -> String {
    let local = Local::now().format("%H:%M:%S").to_string();

    let db_status = match pool.get() {
        Ok(mut conn) => match diesel::sql_query("SELECT 1").execute(&mut conn) {
            Ok(_) => "DB connected".to_string(),
            Err(e) => format!("DB query failed: {e}"),
        },
        Err(e) => format!("DB connection failed: {e}"),
    };

    println!("{db_status}"); // also shows in the terminal running `cargo tauri dev`

    format!("Hello, {}! Greeted from Rust at {}! [{}]", name, local, db_status)
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenvy::dotenv().ok();

    tauri::Builder::default()
        .manage(db::init_pool())
        .manage(SessionState(Mutex::new(None)))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}