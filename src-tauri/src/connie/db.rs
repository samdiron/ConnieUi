use lib_db::sqlite::get_sqlite_conn;
// use lib_db::types::SqlitePool;
use lib_db::inner::query;
use lib_db::inner::Row;

use std::io::Result;



#[tauri::command]
pub fn media(function_name: &str) -> Vec<String> {
    let media_vec = vec![
        "exampleMedia1, type, checksum, size".to_owned(),
        "exampleMedia2, type, checksum, size".to_owned(),
        "exampleMedia3, type, checksum, size".to_owned(),
        "exampleMedia4, type, checksum, size".to_owned(),
        "exampleMedia5, type, checksum, size".to_owned(),
        "exampleMedia6, type, checksum, size".to_owned(),
    ];
    media_vec

    
    // let path = "/opt/Connie/conf/.connieDB.sqlite".to_owned();
    // print!("media function got called from {}",function_name);
    // let pool = get_sqlite_conn(&path).await.unwrap();
    // let sql = format!("SELECT name, type, size, checksum FROM media ; ");
    // let res = query(&sql)
    //     .fetch_all(&pool)
    //     .await.unwrap();
    // let mut media_vec: Vec<String> = vec![];
    // for r in res {
    //     let size: i64 = r.get("size");
    //     let name: String = r.get("name");
    //     let type_: String = r.get("type");
    //     let checksum: String = r.get("checksum");
    //     let full_string = format!("{size}\n{name}\n{type_}\n{checksum}");
    //     media_vec.push(full_string);
    // }
    // Ok(media_vec)
}


