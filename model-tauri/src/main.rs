// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod generate1 {
    use super::*;
    use specta::collect_types;
    use tauri_specta::ts;

    #[test]
    fn export_bindings() {
        ts::export(collect_types![greet], "../view/src/bindings.ts").unwrap();
    }
}
