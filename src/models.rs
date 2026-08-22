use std::fs;
use std::time::{Duration, Instant};
pub struct HexData {
    pub data: Vec<u8>,
    pub filename: String,
}

impl HexData {
    pub fn from_file(path: &str) -> Result<HexData, std::io::Error> {
        let data = fs::read(path)?;
        Ok(HexData {
            data,
            filename: path.to_string(),
        })
    }
    pub fn save_to_file(&self) -> Result<(), std::io::Error> {
        fs::write(&self.filename, &self.data)?;
        Ok(())
    }
    pub fn edit_byte(&mut self, pos: usize, new_byte: u8) -> bool {
        if pos < self.data.len() {
            self.data[pos] = new_byte;
            true
        } else {
            false
        }
    }
}
#[derive(Debug)]
pub enum Mode {
    View,
    Edit { input: String },
    _Command { input: String },
}

pub struct Message {
    pub text: String,
    pub duration: Duration,
    pub level: LogLevel,
    pub created_at: Instant,
}

impl Message {
    pub fn new(text: &str, duration_secs: u64, level: LogLevel) -> Self {
        Self {
            text: text.to_string(),
            created_at: Instant::now(),
            duration: Duration::from_secs(duration_secs),
            level: level,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() >= self.duration
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    User,   // пользовательские действия
    System, // системные события
    Debug,  // отладочная информация
}
pub struct Selection {
    pub start: Option<usize>,
    pub end: Option<usize>,
}

impl Selection {
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.start.is_none() || self.end.is_none()
    }

    pub fn clear(&mut self) {
        self.start = None;
        self.end = None;
    }

    /// Возвращает диапазон (начало, конец) если выделение активно
    pub fn range(&self) -> Option<(usize, usize)> {
        if let (Some(s), Some(e)) = (self.start, self.end) {
            Some((s.min(e), s.max(e)))
        } else {
            None
        }
    }
}
