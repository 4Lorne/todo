use std::fs;

/// Deletes the file with the given name
pub fn delete_file(file_name: &str) -> std::io::Result<()> {
    if file_exists(file_name) {
        fs::remove_file(file_name).expect("TODO: panic message");
    } else {
        println!("File not found.")
    }
    Ok(())
}

/// Checks if a file with the given name exists
pub fn file_exists(file_name: &str) -> bool {
    if let Ok(metadata) = fs::metadata(file_name) {
        metadata.is_file()
    } else {
        false
    }
}
