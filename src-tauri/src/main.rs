#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod engine;
mod settings;

#[tauri::command]
fn get_settings() -> settings::BrowserSettings { settings::default_settings() }

#[tauri::command]
fn normalize_navigation(input: String, search_template: String) -> Result<String, String> {
    engine::normalize_navigation(&input, &search_template)
}

fn main() {
    tauri_runtime_verso::builder()
        .invoke_handler(tauri::generate_handler![get_settings, normalize_navigation])
        .run(tauri::generate_context!())
        .expect("error while running Voidium");
}
