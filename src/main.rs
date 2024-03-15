use minigrep::CommandLine;
use std::{env, process};

fn main() {
    let command_line = CommandLine::new(env::args()).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1)
    });

    minigrep::run(&command_line).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1)
    });
}
