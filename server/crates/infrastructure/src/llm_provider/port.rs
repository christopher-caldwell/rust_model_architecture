use async_trait::async_trait;

use super::types::{ChatCompletionRequest, ChatCompletionResponse};

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn chat_completion(
        &self,
        req: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, String>;
}
