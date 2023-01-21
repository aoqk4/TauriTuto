#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, add, sub, div, mul])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello {}!", name)
}
#[tauri::command]
fn add(op1: i64, op2: i64) -> i64 {
    op1 + op2
}
#[tauri::command]
fn sub(op1: i64, op2: i64) -> i64 {
    op1 - op2
}
#[tauri::command]
fn div(op1: i64, op2: i64) -> i64 {
    op1 / op2
}
#[tauri::command]
fn mul(op1: i64, op2: i64) -> i64 {
    op1 * op2
}
