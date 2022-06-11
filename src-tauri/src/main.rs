#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


// Our Tauri Command
#[tauri::command]
fn return_string(word: String) -> String{
    return word
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![return_string])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
