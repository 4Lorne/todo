use crate::functions::delete_file;
use crate::functions::tasks::{add_task, delete_task};
use delete_file::file_exists;
use dialoguer::Select;

use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::vec;

pub fn open_file(file_name: &str) -> std::io::Result<()> {
    if file_exists(file_name) {
        let mut file = OpenOptions::new()
            .append(true)
            .write(true)
            .read(true)
            .open(file_name)
            .expect("cannot open file");

        let reader = BufReader::new(&file);

        print_file(reader);

        file.seek(SeekFrom::End(0)).expect("seek to end failed");

        loop {
            let items = vec!["Add a new Task", "Delete a Task", "Exit"];
            let selection = Select::new()
                .with_prompt("Select an action")
                .default(0)
                .items(&items[..])
                .interact()
                .unwrap();

            match selection {
                0 => {
                    add_task(&mut file);
                }
                1 => {
                    delete_task(&mut file).expect("TODO: panic message");
                }
                2 => {
                    println!("Exiting...");
                }
                _ => {
                    println!("Invalid input. Please enter [1] or [Exit].");
                }
            }
        }

        // loop {
        //     let mut s = String::new();

        //     print!("Specify an action. [1] to add a new task. [Exit] to exit.\n");

        //     let _ = stdout().flush();
        //     stdin().read_line(&mut s).expect("Error");

        //     match s.trim() {
        //         "1" => {
        //             add_task(&mut file);
        //         }
        //         "2" => {
        //             delete_task(&mut file).expect("TODO: panic message");
        //         }
        //         "0" => {
        //             break;
        //         }
        //         _ => {
        //             println!("Invalid input. Please enter [1] or [Exit].");
        //         }
        //     }
        // }
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
