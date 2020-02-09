use std::env;
use std::process;
extern crate serde_json;
extern crate serde;
mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_contents = lib::open_file(args).unwrap_or_else(|err| {
        println!("Error opening file: {}", err);
        process::exit(1);
    });

    if let Err(e) = lib::run(file_contents) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
