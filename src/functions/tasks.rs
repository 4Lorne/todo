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
        .with_prompt("Select a task to modify:")
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

/// Modifies a line with a strikethrough
pub fn complete_task(file: &File, file_name: &str) {
    let mut lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error reading line"))
        .collect();

    if lines.is_empty() {
        println!("No tasks to select from.");
        return;
    }

    let selection = Select::new()
        .with_prompt("Select a task to mark completed:")
        .default(0)
        .items(&lines[..])
        .interact()
        .unwrap();

    lines[selection] = format!("~~{}~~", lines[selection]);

    let mut file = OpenOptions::new()
        .write(true)
        .open(file_name)
        .expect("cannot open file");

    for line in &lines {
        writeln!(file, "{}", line).expect("cannot write to file");
    }
}

/// Deletes a task from the list
pub fn delete_task(file: &mut File, file_name: &str) {
    let mut lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.expect("Error reading line"))
        .collect();

    if lines.is_empty() {
        println!("No tasks to select from.");
        return;
    }

    let selection = Select::new()
        .with_prompt("Select a task to delete:")
        .default(0)
        .items(&lines[..])
        .interact()
        .unwrap();

    lines.remove(selection);

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_name)
        .expect("cannot open file");

    for line in &lines {
        writeln!(file, "{}", line).expect("cannot write to file");
    }
}
