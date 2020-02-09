use std::fs;
use std::error::Error;
use serde_json;
mod message_structs;
use self::message_structs::HashMapExtend;

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
    parse_message(message_thread);
    Ok(())
}

pub fn parse_message(message_thread: message_structs::MessageThread) {
    let mut counts: message_structs::MessageCounts = message_structs::MessageCounts::new();
    for msg in &message_thread.messages {
        if msg.content != "" {
            for word in msg.content.split_whitespace() {
                let word = word.to_lowercase();
                counts.normal_count.increment_map_count(&word);
            }
        }
        if msg.sticker.uri != "Missing uri" {
            counts.sticker_count.increment_map_count(&msg.sticker.uri);
        }
    }
    for (key, val) in counts.normal_count.iter() {
        println!("Word: \"{0}\" - Count {1}", key, val)
    }
    println!("Total messages: {}", message_thread.messages.len());
}
