use kalosm::language::*;
use std::collections::HashMap;
#[allow(dead_code)]
pub struct ChatbotV3 {
    model: Llama, sessions: HashMap<String, Chat<Llama>>,

    // What should you store inside your Chatbot type?
    // The model? The chat_session?
    // Storing a single chat session is not enough: it mixes messages from different users
    // together!
    // Need to store one chat session per user.
    // Think of some kind of data structure that can help you with this.
    
}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        return ChatbotV3 {
            // Make sure you initialize your struct members here 
            model: model.clone(), sessions: HashMap::new()           
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        // Add your code for chatting with the agent while keeping conversation history here.
        // Notice, you are given both the `message` and also the `username`.
        // Use this information to select the correct chat session for that user and keep it
        // separated from the sessions of other users.
        let chat = 
            self.sessions.entry(username.clone()).or_insert_with(|| {self.model.chat().with_system_prompt("The assistant will act Australian")
        });

        let output = chat
        .add_message(message) 
        .await
        .expect("failure");
        return output.to_string();    
    }

    #[allow(dead_code)]
<<<<<<< HEAD
pub fn get_history(&self, username: String) -> Vec<String> {
    match self.sessions.get(&username) {
        Some(chat) => {
            match chat.session() {
                Ok(session) => session
                    .history()
                    .iter()
                    .filter_map(|m| match m.role() {
                        MessageType::UserMessage => Some(m.content().to_string()),
                        MessageType::ModelAnswer => Some(m.content().to_string()),
                        MessageType::SystemPrompt => None,
                    })
                    .collect(),
                Err(_) => Vec::new(),
=======
    pub fn get_history(&self, username: String) -> Vec<String> {
        match self.sessions.get(&username) {
            Some(chat) => {
                match chat.session() {
                    Ok(session) => session
                        .history()
                        .iter()
                        .filter_map(|m| match m.role() {
                         MessageType::UserMessage => Some(m.content().to_string()),
                         MessageType::ModelAnswer => Some(m.content().to_string()),
                         MessageType::SystemPrompt => None,
                       })
                       .collect(),
                 Err(_) => Vec::new(),
                }
>>>>>>> cache_submission
            }
            None => Vec::new(),
        }
    }
}
<<<<<<< HEAD
}
=======
>>>>>>> cache_submission
