mod cli;
mod tasks;

use structopt::StructOpt;

fn main() {
    println!("{:#?}", cli::CLArgs::from_args());
}
