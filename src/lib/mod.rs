use std::error::Error;
use serde_json;
mod message_structs;
use message_structs::HashMapExtend;
use message_structs::MessageThread;
use message_structs::Participant;

pub fn run(file_contents: String) -> Result<(), Box<dyn Error>> {
    let message_thread: MessageThread = serde_json::from_str(&file_contents).unwrap();
    parse_message(message_thread);
    Ok(())
}

pub fn parse_message(message_thread: MessageThread) {
    let mut participants = Participant::new_as_vector(&message_thread.participants);

    for msg in &message_thread.messages {
        for participant in &mut participants {
            if msg.sender_name != participant.name {
                continue;
            }
            if msg.content != "" {
                participant.message_count += 1;
                for word in msg.content.split_whitespace() {
                    let word = word.to_lowercase();
                    participant.words_used.increment_map_count(&word);
                }
            }
            if msg.sticker.uri != "Missing uri" {
                participant.sticker_count += 1;
                participant.stickers_used.increment_map_count(&msg.sticker.uri);
            }
        }
    }

    // for (key, val) in counts.word_count.iter() {
    //     println!("Word: \"{0}\" - Count {1}", key, val)
    // }

    println!("Total Messages: {}", message_thread.messages.len());
    for participant in participants {
        participant.print_information();
    }
}
