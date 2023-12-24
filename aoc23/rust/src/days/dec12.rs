use std::fs;


pub fn p1(filepath: &str) {
    match fs::read_to_string(filepath) {
        Ok(file_content) => {
            
        },
        Err(e) => {
            println!("BRUH, {}", e)
        }
    }
}