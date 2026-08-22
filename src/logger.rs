use crate::models::{LogLevel, Message};
pub struct Logger {
    messages: Vec<Message>,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    /// Добавляет пользовательский лог
    pub fn add_user_log(&mut self, text: &str, duration_secs: u64) {
        self.messages
            .push(Message::new(text, duration_secs, LogLevel::User));
    }

    /// Добавляет системный лог
    pub fn add_system_log(&mut self, text: &str, duration_secs: u64) {
        self.messages
            .push(Message::new(text, duration_secs, LogLevel::System));
    }

    /// Добавляет отладочный лог
    pub fn add_debug_log(&mut self, text: &str, duration_secs: u64) {
        self.messages
            .push(Message::new(text, duration_secs, LogLevel::Debug));
    }

    /// Возвращает ссылку на все сообщения
    pub fn get_messages(&self) -> &[Message] {
        &self.messages
    }
    pub fn retain(&mut self) {
        self.messages.retain(|msg| !msg.is_expired());
    }
    /// Очищает все сообщения
    pub fn clear(&mut self) {
        self.messages.clear();
    }
}
