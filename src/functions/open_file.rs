use crate::functions::delete_file;
use crate::functions::tasks::{add_task, delete_task, modify_task};
use delete_file::file_exists;
use dialoguer::Select;

use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
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
            ];
            let selection = Select::new()
                .with_prompt("Select an action")
                .default(0)
                .items(&items[..])
                .interact()
                .unwrap();

            file.seek(SeekFrom::Start(0)).expect("seek to start failed");

            match selection {
                0 => add_task(&mut file),

                1 => modify_task(&mut file, &file_name),

                2 => {
                    //Mark task as complete
                }
                3 => {
                    delete_task(&mut file).expect("TODO: panic message");
                }
                4 => {
                    println!("Exiting...");
                    break;
                }
                _ => {
                    println!("Invalid input. Please enter [1] or [Exit].");
                }
            }

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
    print!("\x1B[2J");
    let reader = BufReader::new(file);
    print_file(reader);
}
