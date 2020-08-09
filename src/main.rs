use std::fs;

fn main() {
    let mut files_list: Vec<String> = vec![];
    let dir: &str = "C:\\Users\\sahil\\AppData\\Roaming\\npm-cache\\_logs";
    if let Ok(files) = fs::read_dir(dir) {
        for file in files {
            if let Ok(file) = file {
                files_list.push(file.path().display().to_string());
            }
        }
    }
    for i in 0..files_list.len() {
        fs::remove_file(files_list[i].to_string()).unwrap();
    }
}
