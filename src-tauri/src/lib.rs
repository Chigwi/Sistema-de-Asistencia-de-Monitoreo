//use chrono::Local;
//use diesel::prelude::*;
use std::sync::Mutex;
//use tauri::State;

//use crate::db::DbPool;
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


/*
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
}*/



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenvy::dotenv().ok();

    let pool = db::init_pool();

    match pool.get() {
        Ok(mut conn) => {
            if let Err(e) = services::auth_service::ensure_first_admin(&mut conn) {
                eprintln!("Could not create the first admin: {e}");
            }
        }
        Err(e) => eprintln!("No connection for admin setup: {e}"),
    }

    tauri::Builder::default()
        .manage(pool)
        .manage(SessionState(Mutex::new(None)))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::auth_commands::login,
            commands::auth_commands::logout,
            commands::asistencia_commands::clock_in,
            commands::asistencia_commands::clock_out,
            commands::asistencia_commands::start_break,
            commands::asistencia_commands::end_break,
            commands::asistencia_commands::my_history,
            commands::asistencia_commands::employee_history,
            commands::asistencia_commands::logged_in_list,
            commands::asistencia_commands::logged_out_list,
            commands::asistencia_commands::on_break_list,
            commands::asistencia_commands::current_status,
            commands::asistencia_commands::employee_status,
            commands::empleado_commands::register_employee,
            commands::empleado_commands::list_employees,
            commands::empleado_commands::reset_employee_password,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}