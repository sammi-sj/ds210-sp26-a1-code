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

        // Load previous chat history if it exists
        if let Some(session) = file_library::load_chat_session_from_file(&filename) {
            chat_session = chat_session.with_session(session);
        }
        else {
            println!("chat_with_user: {username} has no previous chat history.");
            chat_session = chat_session.with_system_prompt("The assistant will act like a pirate");
        }

        // Get response from model
        let output = chat_session.add_message(&message).await.unwrap_or_default();

        // Save updated session to file
        if let Ok(session) = chat_session.session() {
            file_library::save_chat_session_to_file(&filename, &session);
        }

        output
    }

    pub fn get_history(&self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);

        match file_library::load_chat_session_from_file(&filename) {
            None => Vec::new(),
            Some(session) => {
                let history = session.history();
                history
                    .iter()
                    .filter_map(|message| {
                        match message.role() {
                            MessageType::UserMessage => Some(format!("{}", message.content())),
                            MessageType::ModelAnswer => Some(format!("{}", message.content())),
                            MessageType::SystemPrompt => None,
                        }
                    })
                .collect()
            }
        }
    }
}