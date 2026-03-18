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
        if !self.sessions.contains_key(&username) {
            let chat_session = self.model.chat()
                .with_system_prompt("The assistant will act like a fat cat");
            self.sessions.insert(username.clone(), chat_session);
        }
        let chat_session = self.sessions.get_mut(&username).unwrap();
        let output = chat_session.add_message(message).await.expect("Failure");
        return output.to_string();
    }

    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
        // Extract the chat message history for the given username
        // Hint: think of how you can retrieve the Chat object for that user, when you retrieve it
        // you may want to use https://docs.rs/kalosm/0.4.0/kalosm/language/struct.Chat.html#method.session
        // to then retrieve the history!
        match self.sessions.get(&username) {
        Some(chat) => {
            match chat.session() {
                Ok(session) => session
                    .history()
                    .iter()
                    .map(|m| format!("{:?}", m)) //Looked up ways to traverse through HashMap and retrieve the messages as a string
                    .collect(),
                Err(_) => Vec::new(),
            }
        }
        None => Vec::new(),
    }
    }
}