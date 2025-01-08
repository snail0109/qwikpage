use std::fs;
use std::path::Path;

pub fn init_config() {
    let files = vec![
        "project.json",
        "page.json", 
        "menu.json",
        "config.json"
    ];
    
    for file in files {
        init_json_file(file);
    }
}

fn init_json_file(filename: &str) {
    if !Path::new(filename).exists() {
        // 创建默认的空JSON对象
        let default_json = "{}";
        
        // 写入文件
        match fs::write(filename, default_json) {
            Ok(_) => println!("成功初始化文件: {}", filename),
            Err(e) => eprintln!("初始化文件失败 {}: {}", filename, e)
        }
    }
}
