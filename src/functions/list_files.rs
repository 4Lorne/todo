use crate::functions::create_file::create_file;
use crate::functions::open_file::open_file;
use crate::functions::tasks::selection;

use std::{
    fs,
    io::{stdin, stdout, Write},
};

/// Lists all the txt files in the directory specified in the config file
pub fn list_files(directory: String) {
    let entries: Vec<_> = fs::read_dir(&directory).unwrap().collect();

    //Maps the entries to a vector of strings
    let mut file_paths: Vec<_> = entries
        .into_iter()
        .filter_map(|res| {
            let path = res.unwrap().path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("txt") {
                Some(path.display().to_string())
            } else {
                None
            }
        })
        .collect();

    if file_paths.is_empty() {
        println!("No files were found in the directory: {}", directory);
        new_file();
        return;
    } else {
        file_paths.push(String::from("Create a new file"));

        let selection = selection(file_paths.clone(), String::from("Select an existing file"));

        if file_paths[selection] == "Create a new file" {
            new_file()
        } else {
            open_file(&file_paths[selection]).expect("TODO: panic message");
        }
    }
}

/// Creates a new file
fn new_file() {
    println!("Name your file:");
    let mut s = String::new();

    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Error");

    while s.trim().is_empty() {
        println!("File name cannot be empty");
        println!("Name your file:");

        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Error");
    }

    create_file(&s.trim().to_string());
}
