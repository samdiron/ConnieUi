use std::sync::LazyLock;
use std::sync::Mutex;

use common_lib::tokio;
// use serde::Serialize;
// use serde::Deserialize;

use lib_db::inner::{
    sqlite::{
        get_sqlite_conn,
        sqlite_host::SqliteHost,
        sqlite_user::SqliteUser,
        sqlite_media::SqliteMedia,
    },
    types::SqlitePool,
    escape_user_input,
    sqlx::{Result, query, Row},

};





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


#[tauri::command]
async fn login(
    username: String,
    server_name: String,
    host_name: String,
) -> (SqliteUser, SqliteHost) {
    let pool = DB_POOL.lock().unwrap().clone();
    let host = get_server(&server_name, &host_name, &pool).await;
    let username = escape_user_input(&username);
    let host = if host.is_err() {
        SqliteHost {
            name: "NULL".to_owned(),
            cpid: "NULL".to_owned(),
            host: "NULL".to_owned(),
            port: 0,
            pub_ip: "NULL".to_owned(),
            pri_ip: "NULL".to_owned(),
        }
    } else {
        host.unwrap()
    };
    let sql = format!(" SELECT * FROM user WHERE username = '{username}' AND host = '{}' ", &host.cpid);
    let res = query(&sql).fetch_one(&pool).await;
    let user = if res.is_ok() {
        let res = res.unwrap();
        let name: String = res.get("name");
        let cpid: String = res.get("cpid");
        let in_host: String = res.get("host");
        let email: String = res.get("email");
        let usrname: String = res.get("usrname");
        let user = SqliteUser {
            name,
            host: in_host,
            cpid,
            email,
            usrname
        };
        user
    } else {
        SqliteUser {
            name: "NULL".to_owned(),
            host: "NULL".to_owned(),
            cpid: "NULL".to_owned(),
            email: "NULL".to_owned(),
            usrname: "NULL".to_owned(),
        }
    };
    (user, host)
}


async fn get_server(
    server_name: &String,
    host_name: &String,
    pool: &SqlitePool
) -> Result<SqliteHost> {
    let name = escape_user_input(server_name);
    let host = escape_user_input(host_name);
    let sql = format!(" SELECT * FROM host WHERE name = '{name}' AND host = '{host}' ;");
    let res = query(&sql).fetch_one(pool).await?;
    
    let name: String = res.get("name");
    let host: String = res.get("host");
    let cpid: String = res.get("cpid");
    let pub_ip: String = res.get("pub_ip");
    let pri_ip: String = res.get("pri_ip");
    let port: u16 = res.get("port");

    let host = SqliteHost {
        name,
        cpid,
        host,
        port,
        pub_ip,
        pri_ip,
    };
    Ok(host)
}


#[tauri::command]
async fn list_media(user_cpid: String) -> Vec<SqliteMedia> {
    let pool = DB_POOL.lock().unwrap().clone();
    let cpid = escape_user_input(&user_cpid);
    let sql = format!(" SELECT * FROM media WHERE cpid = '{cpid}' ;" );
    let res = query(&sql).fetch_all(&pool).await.unwrap();
    let mut media_vec: Vec<SqliteMedia> = vec![];
    for r in res {
        let name: String = r.get("name");
        let cpid: String = r.get("cpid");
        let path: String = r.get("path");
        let type_: String = r.get("type");
        let host: String = r.get("host");
        let checksum: String = r.get("checksum");

        let date: i64 = r.get("date");
        let size: i64 = r.get("size");

        let media = SqliteMedia {
            name,
            cpid,
            size,
            date,
            path,
            type_,
            host,
            checksum,
        };
        media_vec.push(media);

    }
    media_vec
}







#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            set_db_path,
            list_media,
            login,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
