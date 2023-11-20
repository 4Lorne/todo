use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Seek, SeekFrom, stdin, stdout, Write};
use delete_file::file_exists;
use crate::functions::delete_file;

pub fn open_file(file_name: &str) -> std::io::Result<()> {
    if file_exists(file_name) {
        let mut file = OpenOptions::new()
            .append(true)
            .write(true)
            .read(true)
            .open(file_name)
            .expect("cannot open file");

        let reader = BufReader::new(&file);

        //Prints the contents of the file
        //TODO: Refactor this to a function
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

        file.seek(SeekFrom::End(0)).expect("seek to end failed");
        let mut exit = false;
        while !exit {
            let mut s = String::new();
            print!("Specify a line to add or change, followed by the task you wish to add. \n");
            let _ = stdout().flush();
            stdin().read_line(&mut s).expect("Error");
            if s.contains("!Exit") {
                exit = true;
            } else {
                file.write(s.as_bytes()).expect("Write failed");
            }
        }
    }

    Ok(())
}
