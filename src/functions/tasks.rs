use std::fs::{File, OpenOptions};
use std::io::{stdin, stdout, Write};
use std::io::{BufRead, BufReader};

use dialoguer::Select;

/// Adds a new task to the list
pub fn add_task(file: &mut File) {
    let mut s = String::new();

    print!("Add a new Task \n");

    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Error");

    if s.trim().is_empty() {
        return;
    }

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
        .with_prompt("Select a task to modify")
        .default(0)
        .items(&lines[..])
        .interact()
        .unwrap();

    // Directly modifying the line in the vector
    lines[selection] = modify_line(lines[selection].to_string(), selection);

    if lines[selection].is_empty() {
        return;
    }

    create_new_file(lines, file_name);
}

// Modifies the specified line
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

    let selection = selection(
        lines.clone(),
        String::from("Select a task to mark completed"),
    );

    if lines[selection].contains("~~") {
        lines[selection] = lines[selection].replace("~~", "");
        return;
    } else {
        lines[selection] = format!("~~{}~~", lines[selection]);
    }

    create_new_file(lines, file_name);
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

    let selection = selection(lines.clone(), String::from("Select a task to delete"));

    lines.remove(selection);

    create_new_file(lines, file_name)
}

/// Creating a new file with the updated vector to replace the old file
fn create_new_file(lines: Vec<String>, file_name: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_name)
        .expect("cannot open file");

    for line in &lines {
        writeln!(file, "{}", line).expect("cannot write to file");
    }
}

pub fn selection(items: Vec<String>, prompt: String) -> usize {
    let selection = Select::new()
        .with_prompt(prompt)
        .default(0)
        .items(&items[..])
        .interact()
        .unwrap();

    return selection;
}
