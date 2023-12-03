use std::fs::File;
use std::io;
use std::io::{stdin, stdout, BufRead, BufReader, Seek, SeekFrom, Write};

pub fn add_task(file: &mut File) {
    let mut s = String::new();

    print!("Specify a line to add or change, followed by the task you wish to add. \n");

    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Error");

    file.write(s.as_bytes()).expect("Write failed");
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
