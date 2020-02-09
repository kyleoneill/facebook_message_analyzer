use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub struct MessageCounts {
    pub normal_count: HashMap<String, i32>,
    pub sticker_count: HashMap<String, i32>
}
impl MessageCounts {
    pub fn new() -> MessageCounts {
        MessageCounts {
            normal_count: HashMap::new(),
            sticker_count: HashMap::new()
        }
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
pub struct Participant {
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
    pub participants: Vec<Participant>,
    pub messages: Vec<Message>,
    pub title: String,
    pub is_still_participant: bool,
    pub thread_type: String,
    pub thread_path: String
}
