// use std::io::BufReader;
use tauri::command;
use calamine::{open_workbook, Reader, Xlsx};
use calamine::DataType;
use serde_json::json;
// use std::fs;
use serde::{Deserialize, Serialize};
// use serde_json::Result;
use serde_json::to_string_pretty;
use std::collections::HashMap;

#[tauri::command]
pub fn import_excel(xl_path:String) -> Result<(), String> {

    let mut workbook: Xlsx<_> = match open_workbook(&xl_path){
        Ok(book)=>book,
        Err(_) => return Err("cannot open xl book".to_string()),
    };

    // シート名を取得
    let sheet_names = workbook.sheet_names().to_owned();
    for sheet_name in &sheet_names {
        
        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            // 行ごとに出力
            let mut map = HashMap::new();
            let mut class_map = HashMap::new();
                    
            for (i, row) in range.rows().skip(1).enumerate() {
                let index = i + 1;
                
                if *sheet_name == sheet_names[1] {
                    let name = match row[0].get_string(){
                        Some(n) => n.to_string(),
                        None => break,
                    };
                    // calamine::DataType::Bool に対応
                    let valid = match row[1].get_bool(){
                        Some(b) => b,      // b は bool でも、row[1] が &DataType のため mismatch
                        _ => false,
                    };
                    let id = format!{"c{:03}",index};
                    let class = Class {
                        name,
                        valid,
                    };
                    class_map.insert(id, class);
                }
            }
            map.insert("Classes".to_string(), class_map);
            let json_str = to_string_pretty(&map).unwrap();
            println!("{}", json_str);
        } else {
            println!("シートの読み込みに失敗しました。");
        }
    }

    Ok(())
}
// // 関数ポインタの配列
//     let funcs: Vec<fn()> = vec![func_a, func_b, func_c];

//     for i in 0..5 {
//         // インデックスに応じて関数を選択
//         let f = funcs[i % funcs.len()]; // ループでも回す
//         f(); // 関数を実行
//     }

#[derive(Serialize, Deserialize)]
struct Class {
    name: String,
    valid: bool,
}

#[derive(Serialize, Deserialize)]
struct Subject_Time {
    class_id: String,
    time: i64,
}

#[derive(Serialize, Deserialize)]
struct Subject {
    name: String,
    time: Vec<Subject_Time>,
}

#[derive(Serialize, Deserialize)]
struct Teacher {
    name: String,
    div: String,
    charge: String,
}

#[derive(Serialize, Deserialize)]
struct Support {
    class_id: String,
    member_class: Vec<String>,
    exchange: Vec<String>,
}
    

