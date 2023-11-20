mod args;
mod functions;

use args::TodoArgs;
use clap::Parser;
use functions::create_file::create_file;
use functions::open_file::open_file;
use crate::functions::delete_file::delete_file;

fn main() {
    let args = TodoArgs::parse();

    match args.first_arg.as_str() {
        "create" => {
            create_file(args.second_arg.as_str()).expect("TODO: panic message");
            open_file(args.second_arg.as_str()).expect("TODO: panic message");
        },
        "open" => {
            open_file(args.second_arg.as_str()).expect("TODO: panic message");
        },
        "delete" => {
            delete_file(args.second_arg.as_str()).expect("TODO: panic message");
        }
        _ => {}
    }
}
