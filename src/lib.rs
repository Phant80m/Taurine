use colored::Colorize; 
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

pub fn help() {
    println!("help text temp");
}

pub fn version() {
    println!("{} {}", NAME.to_string().blue(), VERSION.to_string().green());
}

