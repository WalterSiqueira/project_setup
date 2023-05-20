use std::fs::{self, File};
use std::io;
fn main() {
    println!("Please insert the path for the target folder");
    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("Failed to read path input");

    path = path.trim().to_string();

    match fs::create_dir_all(&path) {
        Ok(_) => {
            println!("directory created: {}", path);

            create_subdirectory(&path, "css"); 
            create_subdirectory(&path, "js");
            create_subdirectory(&path, "img");
            let file_path = format!("{}/index.html", path);
            let _file = match File::create(&file_path) {
                Ok(_file) => _file,
                Err(err) => {
                    println!("Failed to create the index.html file: {}",err);
                    return;
                }
            };
        }
        Err(err) => println!("Failed to create the directory in the specified path: {}", err),
    }
}

fn create_subdirectory(parent_path: &str, subdirectory_name: &str) {
    let subdirectory_path = format!("{}/{}", parent_path, subdirectory_name);
    match fs::create_dir(&subdirectory_path) {
        Ok(_) => println!("Subdirectory created: {}", subdirectory_path),
        Err(err) => println!("Failed to create subdirectory: {}", err),
    };
}
