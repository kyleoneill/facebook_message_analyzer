use std::fs;
use std::error::Error;
use serde_json;
mod message_structs;

pub fn open_file(args: Vec<String>) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("Need the name of the file to parse");
    }
    //let filename = args[1].clone();
    let filename = "message_2.json";
    fs::read_to_string(filename).map_err(|_err| {"Could not read file"})
}

pub fn run(file_contents: String) -> Result<(), Box<dyn Error>> {
    let message_thread: message_structs::MessageThread = serde_json::from_str(&file_contents).unwrap();
    for msg in message_thread.messages {

    }
    Ok(())
}
