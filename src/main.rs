use std::env;
use taurine::{help, version};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];

        match &command[..] {
            "version" | "--version" | "-v" =>  version(),
            "help" | "--help" | "-h" => help(),
            _ => println!("not a valid command")
        }
    }
}
