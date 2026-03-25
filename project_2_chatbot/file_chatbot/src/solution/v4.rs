use kalosm::language::*;
use crate::solution::file_library;

pub struct ChatbotV4 {
    model: Llama,
}

impl ChatbotV4 {
    pub fn new(model: Llama) -> ChatbotV4 {
        return ChatbotV4 {
            model: model,
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);

        let mut chat_session = self.model
            .chat();

        match file_library::load_chat_session_from_file(&filename) {
            Some(session) => {
                chat_session = chat_session.with_session(session);
    }
            None => {
            chat_session = chat_session.with_system_prompt("The assistant will act like a pirate");
            }
        }

        let output = chat_session.add_message(&message).await.unwrap();

        match chat_session.session() {
            Ok(session) => {file_library::save_chat_session_to_file(&filename, &session);
            }
            Err(_) => {}
        }
        output
    }

    pub fn get_history(&self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);

        match file_library::load_chat_session_from_file(&filename) {
        None => Vec::new(),
        Some(session) => {
            session
                .history()
                .iter()
                .filter_map(|message: &kalosm::language::ChatMessage| {
                    match message.role() {
                        MessageType::UserMessage => {
                            Some(format!("{}", message.content()))
                        }
                        MessageType::ModelAnswer => {
                            Some(format!("{}", message.content()))
                        }
                        MessageType::SystemPrompt => None,
                    }
                })
                .collect()
            }
        }
    }
}