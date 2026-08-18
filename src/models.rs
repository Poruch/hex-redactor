use std::time::{Duration, Instant};
pub struct HexData {
    pub data: Vec<u8>,
    pub filename: String,
}
pub enum Mode {
    View,
    Edit { input: String },
    Command { input: String },
}

pub struct Message {
    pub text: String,
    pub duration: Duration,
    pub created_at: Instant,
}

impl Message {
    pub fn new(text: &str, duration_secs: u64) -> Self {
        Self {
            text: text.to_string(),
            created_at: Instant::now(),
            duration: Duration::from_secs(duration_secs),
        }
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() >= self.duration
    }
}
