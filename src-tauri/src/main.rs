#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod auth;
pub mod iteration;
pub mod models;
pub mod schema;

use auth::authenticate;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use iteration::{create_iteration, find_all};
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            authenticate,
            find_all,
            create_iteration
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
