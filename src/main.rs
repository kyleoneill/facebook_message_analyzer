use std::process;
use std::env;
extern crate serde_json;
extern crate serde;
mod lib;

fn main() {
    let file_contents = open_file(std::env::args()).unwrap_or_else(|err| {
        println!("Error opening file: {}", err);
        process::exit(1);
    });

    if let Err(e) = lib::run(file_contents) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

pub fn open_file(mut args: env::Args) -> Result<String, &'static str> {
    args.next();
    let filename = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a file name")
    };
    std::fs::read_to_string(filename).map_err(|_err| {"Could not read file"})
}
