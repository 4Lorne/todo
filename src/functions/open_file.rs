use crate::functions::delete_file;
use crate::functions::tasks::{add_task, complete_task, delete_task, modify_task, selection};
use delete_file::file_exists;

use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::process::Command;
use std::vec;

pub fn open_file(file_name: &str) -> std::io::Result<()> {
    if file_exists(file_name) {
        let mut file = OpenOptions::new()
            .append(true)
            .write(true)
            .read(true)
            .open(file_name)
            .expect("cannot open file");

        loop {
            clear_and_print_file(&file);

            let items = vec![
                "Add a new Task",
                "Modify a Task",
                "Mark a Task as Complete",
                "Delete a Task",
                "Exit",
            ]
            .into_iter()
            .map(|s| s.to_string())
            .collect();

            let selection = selection(items, String::from("Select a task:"));

            file.seek(SeekFrom::Start(0)).expect("seek to start failed");

            match selection {
                0 => add_task(&mut file),

                1 => modify_task(&mut file, &file_name),

                2 => complete_task(&file, file_name),

                3 => delete_task(&mut file, file_name),

                4 => {
                    println!("Exiting...");
                    break;
                }
                _ => {
                    println!("Invalid input.");
                }
            }

            //Returns the cursor to the start of the file
            file.seek(SeekFrom::Start(0)).expect("seek to start failed");
        }
    }

    Ok(())
}

//Prints the contents of the file
fn print_file<T: BufRead>(reader: T) {
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
}

//Clears the terminal and prints the contents of the file
fn clear_and_print_file<F: Read>(file: F) {
    Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    let reader = BufReader::new(file);
    print_file(reader);
}
