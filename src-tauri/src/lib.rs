use std::sync::LazyLock;
use std::sync::Mutex;

use common_lib::tokio;
use lib_db::sqlite::get_sqlite_conn;
use lib_db::types::SqlitePool;
use lib_db::inner::{query, Row};
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


use lib_db::escape_user_input; 

#[tauri::command]
async fn list_media(u: User) -> Vec<Media> {
    let pool = DB_POOL.lock().unwrap().clone();
    let cpid = escape_user_input(&u.cpid);
    let sql = format!(" SELECT * from media WHERE cpid = '{cpid}' ;" );
    let res = query(&sql).fetch_all(&pool).await.unwrap();
    let mut media_vec: Vec<Media> = vec![];
    for r in res {
        let name: String = r.get("name");
        let cpid: String = r.get("cpid");
        let path: String = r.get("path");
        let type_: String = r.get("type");
        let in_host: String = r.get("host");
        let checksum: String = r.get("checksum");

        let date: i64 = r.get("date");
        let size: i64 = r.get("size");

        let media = Media {
            name,
            cpid,
            size,
            date,
            path,
            type_,
            in_host,
            checksum,
        };
        media_vec.push(media);

    }
    media_vec
}




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
    cpid: String,
    size: i64,
    date: i64,
    path: String,
    type_: String,
    in_host: String,
    checksum: String,

}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![set_db_path, list_media])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
