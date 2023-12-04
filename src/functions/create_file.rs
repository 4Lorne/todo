use std::fs::File;

use crate::functions::open_file::open_file;

/// Creates a text file with the given name
pub fn create_file(file_name: &str) {
    let new_file_name = format!("{}.txt", file_name);
    File::create(new_file_name.clone()).expect("Unable to create file");

    open_file(new_file_name.as_str()).expect("TODO: panic message");
}
