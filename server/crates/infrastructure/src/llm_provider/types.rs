#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct ChatCompletionRequest {
    pub messages: Vec<Message>,
    pub max_tokens: Option<u16>,
}

#[derive(Debug, Clone)]
pub struct ChatCompletionResponse {
    pub content: String,
}
