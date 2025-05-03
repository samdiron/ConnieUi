use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    username: String,
    passwd: String,
    email: String,
    cpid: String
}

#[derive(Serialize, Deserialize)]
struct Server {
    name: String,
    host: String,
    cpid: String,
    pub_ip: String,
    pri_ip: String,
}

#[derive(Serialize, Deserialize)]
struct Media {
    name: String,
    ext: String,
    checksum: String,
    in_host: String,

}











#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
