mod args;
mod functions;

use args::TodoArgs;
use clap::Parser;
use functions::create_file::create_file;
use functions::open_file::open_file;
use crate::functions::delete_file::delete_file;

/// TODO
/// Installation
/// Ask for directory, if none provided use default directory
/// Using
/// Provide list of files to select
///https://crates.io/crates/aarty

fn main() {
    let args = TodoArgs::parse();
    print!("\x1B[2J");


    match args.first_arg.as_str() {
        "create" => {
            create_file(args.second_arg.as_str()).expect("TODO: panic message");
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
