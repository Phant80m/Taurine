use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    subcommand: Option<Init>,
    
}

// sub command
#[derive(Subcommand, Debug)]
enum Init{
    Init {
        #[arg(short, long)]
        path: String,
    },

}

fn main() {
    let _args = Args::parse();
    
    match _args.subcommand {
        Some(Init::Init { path }) => {
            Command::new("cargo")
                .arg("new")
                .arg(path)
                .output()
                .expect("Failed to init cargo");
        }
        None => eprintln!("error init cargo")
    }

}
