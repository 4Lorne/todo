use std::fs::File;
use std::io::{BufRead, BufReader};
use delete_file::file_exists;
use crate::functions::delete_file;

pub fn open_file(file_name: &str) -> std::io::Result<()> {
    if file_exists(file_name) {
        let file = File::open(file_name)?;

        let reader = BufReader::new(file);

        for (index, line) in reader.lines().enumerate() {
            match line {
                Ok(content) => {
                    println!("#{}: {}", index + 1, content);
                }
                Err(err) => {
                    eprintln!("Error reading line {}: {}", index + 1, err);
                }
            }
        }
    } else {
        println!("File not found.")
    }

    Ok(())
}
