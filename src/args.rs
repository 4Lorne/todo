use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct TodoArgs {
    /// First argument
    pub first_arg: String,
    /// Second argument
    pub second_arg: String,
}
