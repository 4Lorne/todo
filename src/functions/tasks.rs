//use crate::functions::open_file::clear_and_print_file;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader};
use std::io::{stdin, stdout, Write};

use dialoguer::Select;

/// Adds a new task to the list
pub fn add_task(file: &mut File) {
    let mut s = String::new();

    print!("Add a new Task \n");

    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Error");

    file.write(s.as_bytes()).expect("Write failed");
}

/// Modifies an existing task on the list
pub fn modify_task(file: &File, file_name: &str) {
    // Reads the file into a vector
    let mut lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error reading line"))
        .collect();

    if lines.is_empty() {
        println!("No tasks to select from.");
        return;
    }

    let selection = Select::new()
        .with_prompt("Select a line to modify:")
        .default(0)
        .items(&lines[..])
        .interact()
        .unwrap();

    // Directly modifying the line in the vector
    lines[selection] = modify_line(lines[selection].to_string(), selection);

    // Rewrites the file with the modified line
    let mut file = OpenOptions::new()
        .write(true)
        .open(file_name)
        .expect("cannot open file");

    for line in &lines {
        writeln!(file, "{}", line).expect("cannot write to file");
    }
}

pub fn modify_line(line: String, index: usize) -> String {
    let mut s = String::new();

    print!("Modifying Task #{}: {} \n", index + 1, line);

    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Error");
    return s.trim_end().to_string();
}

pub fn delete_task(file: &mut File) -> io::Result<&File> {
    // let mut s = String::new();

    // print!("Specify a line number to delete: ");
    // let _ = io::stdout().flush();
    // io::stdin().read_line(&mut s)?;

    // // Parse the input as a line number
    // let line_number: usize = match s.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => {
    //         println!("Invalid input. Please enter a valid line number.");
    //         return Ok(file);
    //     }
    // };

    // // Read the file line by line
    // let reader = BufReader::new(&file);
    // let lines: Vec<String> = reader
    //     .lines()
    //     .map(|line| line.expect("Error reading line"))
    //     .collect();

    // // Check if the line number is valid
    // if line_number > 0 && line_number <= lines.len() {
    //     // Create a new Vec excluding the line to be deleted
    //     let updated_lines: Vec<&str> = lines
    //         .iter()
    //         .enumerate()
    //         .filter(|&(index, _)| index + 1 != line_number)
    //         .map(|(_, line)| line.as_str())
    //         .collect();

    //     // Clear the file and write the updated lines back
    //     file.seek(SeekFrom::Start(0))?; // Move the cursor to the beginning of the file
    //     file.set_len(0)?; // Clear the file

    //     for line in updated_lines {
    //         writeln!(file, "{}", line)?;
    //     }

    //     println!("Line {} deleted successfully.", line_number);
    // } else {
    //     println!("Invalid line number. Please enter a valid line number.");
    // }

    Ok(file)
}
