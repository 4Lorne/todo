use std::fs::File;

/// Creates a file with the given name
pub fn create_file(file_name: &str) -> std::io::Result<()> {
    File::create(file_name)?;
    Ok(())
}
