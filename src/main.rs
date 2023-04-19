use clap::{Parser, Subcommand};
use std::{fs, process::Command};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    subcommand: Init
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
        Init::Init { path } => {
           println!("{}", path);
        }
    }
}
