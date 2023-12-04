use crate::functions::create_file::create_file;
use crate::functions::open_file::open_file;

use dialoguer::Select;
use std::{
    fs,
    io::{stdin, stdout, Write},
};

pub fn list_files(directory: String) {
    let entries: Vec<_> = fs::read_dir(&directory).unwrap().collect();

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

        let selection = Select::new()
            .with_prompt("Select an existing file:")
            .default(0)
            .items(&file_paths)
            .interact()
            .unwrap();

        if file_paths[selection] == "Create a new file" {
            new_file()
        } else {
            open_file(&file_paths[selection]).expect("TODO: panic message");
        }
    }
}

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
