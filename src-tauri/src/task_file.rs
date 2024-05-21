use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

// 定义结构体
#[derive(Serialize, Deserialize)]
pub struct Task {
    id: String,
    from_wxid_list: Vec<String>,
    to_wxid_list: Vec<String>,
}

// 将结构体写入 JSON 文件的函数
pub fn write_to_json_file(wxid: &str, task: &Task) -> std::io::Result<()> {
    // 定义文件路径
    let file_path = "".to_string() + wxid + "/task/"+ &task.id +".json";
    
    // 创建嵌套目录
    if let Some(parent) = Path::new(&file_path.clone()).parent() {
        fs::create_dir_all(parent)?;
    }

    // 将 Task 实例序列化为 JSON 字符串
    let json_string = serde_json::to_string(task).unwrap();

    // 打开文件以进行写入
    let mut file = File::create(file_path.clone())?;
    log::info!("file_path -- {:?}",file_path.clone());
    // 获取文件的绝对路径并返回
    let absolute_path = fs::canonicalize(file_path.clone())?;
   
    log::info!("absolute_path -- {:?}",absolute_path);
    // 将 JSON 字符串写入文件
    file.write_all(json_string.as_bytes())?;
    
    Ok(())
}

// 从 JSON 文件读取数据并反序列化为结构体的函数
pub fn read_from_json_file(wxid: &str) -> Vec<Task> {
    let directory_path = ".\\".to_string() + wxid + "\\task"; // 当前目录

    let mut result = Vec::new();

    // 获取目录中的所有文件
    if let Ok(entries) = fs::read_dir(directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "json" {
                        if let Some(file_name) = path.file_name().and_then(|os_str| os_str.to_str()).map(String::from) {
                            if let Ok(file_content) = fs::read_to_string(&path) {
                                // 将文件内容解析为 Person 结构体
                                if let Ok(task) = serde_json::from_str::<Task>(&file_content) {
                                    result.push(task);
                                } else {
                                    eprintln!("Failed to parse JSON in file: {:?}", path);
                                }
                            } else {
                                eprintln!("Failed to read file: {:?}", path);
                            }
                        } else {
                            eprintln!("Failed to get file name: {:?}", path);
                        }
                    }
                }
            }
        }
    }

    result
}