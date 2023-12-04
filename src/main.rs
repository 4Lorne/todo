mod args;
mod functions;
mod read_config;

use args::TodoArgs;
use clap::Parser;
use functions::create_file::create_file;
use functions::delete_file::delete_file;
use functions::list_files::list_files;
use functions::open_file::open_file;
use read_config::{config_exists, read_config};

/// TODO
/// Escape to go back

fn main() {
    let config = read_config();
    let args_len = std::env::args().count() - 1;

    if config_exists() && args_len == 0 {
        list_files(config.directory);
    } else {
        let args: TodoArgs = TodoArgs::parse();
        // Clears the terminal
        print!("\x1B[2J");

        //Parses the arguments and calls the appropriate function
        match args.first_arg.as_str() {
            "create" => {
                create_file(args.second_arg.as_str());
                open_file(args.second_arg.as_str()).expect("TODO: panic message");
            }
            "open" => {
                open_file(args.second_arg.as_str()).expect("TODO: panic message");
            }
            "delete" => {
                delete_file(args.second_arg.as_str()).expect("TODO: panic message");
            }
            _ => {}
        }
    }
}
