use std::sync::LazyLock;
use std::sync::Mutex;

use common_lib::tokio;
use lib_db::sqlite::get_sqlite_conn;
use lib_db::types::SqlitePool;
use serde::Serialize;
use serde::Deserialize;



const DB_POOL: LazyLock<Mutex<SqlitePool>>  = LazyLock::new(||
    {
        let rt = tokio::runtime::Runtime::new().unwrap();
        
        let pool = rt.block_on(async move {
        
            let binding = get_sqlite_conn(
                &"/opt/Connie/conf/.connieDB.sqlite".to_owned()
            ).await.unwrap();
            binding
        } );
        Mutex::new(pool)
    }
);



#[tauri::command]
async fn set_db_path(path: String) -> bool { 
    let is_pool = get_sqlite_conn(&path).await;
    if is_pool.is_err() {
        return false
    };
    *DB_POOL.lock().unwrap() = is_pool.unwrap();
    return true
}




// async fn list_users 



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
        .invoke_handler(tauri::generate_handler![set_db_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
