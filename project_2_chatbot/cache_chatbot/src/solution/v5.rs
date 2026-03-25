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
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None =>{
                println!("get_history: {username} is not in the cache!");
                //TODO:The cache does not have the chat. What should you do?
                //Your code goes here.
                match file_library::load_chat_session_from_file(&filename) {
                    None => return Vec::new(),

                    Some (session) => {
                        let chat_session = self.model.chat().with_session(session.clone());
                        self.cache.insert_chat(username.clone(), chat_session);
                        return session
                        .history()
                        .iter()
                        .filter_map(|message: &kalosm::language::ChatMessage| {
                        match message.role() {
                            MessageType::UserMessage => {Some(format!("{}", message.content()))}
                            MessageType::ModelAnswer => {Some(format!("{}", message.content()))}
                            MessageType::SystemPrompt => None,
                        }
                        }
                        )
                        .collect();
                    }              
                }   
            }

            Some(chat_session) => {
                println!("get_history: {username} is in the cache! Nice!");
                //TODO: The cache has this chat. What should you do?
                //Your code goes here.
                match chat_session.session().ok() {
                    None => return Vec::new(),

                    Some(session) => 
                    return session
                    .history()
                    .iter()
                    .filter_map(|message: &kalosm::language::ChatMessage| {
                    match message.role() {
                        MessageType::UserMessage => {Some(format!("{}", message.content()))}
                        MessageType::ModelAnswer => {Some(format!("{}", message.content()))}
                        MessageType::SystemPrompt => None,
                    }
                    }      
                    )
                .collect(),
                }
            }
        }
    }
}