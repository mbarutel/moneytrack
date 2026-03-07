mod cli;

use cli::config;
use std;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = config::Config::build(&args);

    match config.cmd {
        config::Process::List => println!("list the transactions!"),
        config::Process::Summarize => println!("summarize the transactions"),
    }
}
