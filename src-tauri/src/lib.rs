mod connie;

pub use connie::db::media;



#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You're a brat", name)
}











#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
