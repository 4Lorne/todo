use std::fs::File;
use std::io::{BufWriter, Write};

pub fn create_file(file_name: &str) -> std::io::Result<()> {
    File::create(file_name)?;
    Ok(())
}
