use kalosm::language::*;
use file_chatbot::solution::file_library;

use crate::solution::Cache;

pub struct ChatbotV5 {
    model: Llama,
    cache: Cache<Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        return ChatbotV5 {
            model: model,
            cache: Cache::new(3),
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = format!("{}.txt", username);

        if let Some(chat_session) = self.cache.get_chat(&username) {
            println!("chat_with_user: {username} is in the cache! Nice!");
            let output = chat_session.add_message(&message).await.unwrap_or_default();
            if let Ok(session) = chat_session.session() {
                file_library::save_chat_session_to_file(&filename, &session);
            }
            return output;
        }

        println!("chat_with_user: {username} is not in the cache!");
        let mut chat_session = self.model.chat();
        if let Some(session) = file_library::load_chat_session_from_file(&filename) {
            chat_session = chat_session.with_session(session);
        }

        let output = chat_session.add_message(&message).await.unwrap_or_default();
        if let Ok(session) = chat_session.session() {
            file_library::save_chat_session_to_file(&filename, &session);
        }
        self.cache.insert_chat(username.clone(), chat_session);
        output
    }

    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = format!("{}.txt", username);

        if let Some(chat_session) = self.cache.get_chat(&username) {
            println!("get_history: {username} is in the cache! Nice!");
            if let Ok(session) = chat_session.session() {
                return session
                    .history()
                    .into_iter()
                    .map(|msg| msg.content().to_string())
                    .collect();
            }
            return Vec::new();
        }

        println!("get_history: {username} is not in the cache!");
        if let Some(session) = file_library::load_chat_session_from_file(&filename) {
            let chat_session = self.model.chat().with_session(session.clone());
            self.cache.insert_chat(username.clone(), chat_session);
            return session
                .history()
                .into_iter()
                .map(|msg| msg.content().to_string())
                .collect();
        }

        Vec::new()
    }
}