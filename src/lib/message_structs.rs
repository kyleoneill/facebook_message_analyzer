use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub struct Participant {
    pub name: String,
    pub message_count: i32,
    pub sticker_count: i32,
    pub words_used: HashMap<String, i32>,
    pub stickers_used: HashMap<String, i32>
}
impl Participant {
    pub fn new(name: String) -> Participant {
        Participant {
            name: name,
            message_count: 0,
            sticker_count: 0,
            words_used: HashMap::new(),
            stickers_used: HashMap::new()
        }
    }
    pub fn new_as_vector(input: &Vec<MessageParticipant>) -> Vec<Participant> {
        let mut participants = Vec::new();
        for person in input {
            let new_participant = Participant::new(person.name.clone());
            participants.push(new_participant);
        }
        participants
    }
    pub fn print_information(&self) {
        println!("Messages from {0}: {1}", self.name, self.message_count);
        println!("Stickers from {0}: {1}", self.name, self.sticker_count);
    }
}

pub trait HashMapExtend {
    fn increment_map_count(&mut self, word: &str);
}
impl HashMapExtend for HashMap<String, i32> {
    fn increment_map_count(&mut self, word: &str) {
        let count = self.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
}

#[derive(Serialize, Deserialize)]
pub struct MessageParticipant {
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct Sticker {
    pub uri: String
}
impl Default for Sticker {
    fn default() -> Sticker {
        Sticker {
            uri: "Missing uri".to_string()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Share {
    pub link: String
}
impl Default for Share {
    fn default() -> Share {
        Share {
            link: "Missing link".to_string()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Photos {
    pub uri: String,
    pub creation_timestamp: i64
}
impl Default for Photos {
    fn default() -> Photos {
        Photos {
            uri: "Missing uri".to_string(),
            creation_timestamp: 0
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub sender_name: String,
    pub timestamp_ms: u64,
    #[serde(default)] pub content: String,
    #[serde(default)] pub sticker: Sticker,
    #[serde(default)] pub share: Share,
    #[serde(default)] pub photos: Vec<Photos>,
    pub r#type: String
}

#[derive(Serialize, Deserialize)]
pub struct MessageThread {
    pub participants: Vec<MessageParticipant>,
    pub messages: Vec<Message>,
    pub title: String,
    pub is_still_participant: bool,
    pub thread_type: String,
    pub thread_path: String
}
