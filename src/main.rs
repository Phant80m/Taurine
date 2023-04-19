use clap::{Parser, Subcommand};
use std::{fs, process::Command};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    subcommand: Init,
    
    ///install a pkg with pacman
    #[arg(short, long)]
    sync: String,
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
            Command::new("cargo")
                .arg("new")
                .arg(path)
                .output()
                .expect("Failed to init cargo");
        }
    }
    match _args.sync {
        _ => Command::new("sudo")
            .arg("pacman")
            .arg("-S")
            .arg(_args.sync)
            .output()
            .expect("failed to download pacman pkg"),
    };
}
