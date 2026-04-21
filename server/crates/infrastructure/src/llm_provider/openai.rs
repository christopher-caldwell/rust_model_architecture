use async_trait::async_trait;
use serde::Serialize;
use serde_json::Value;

use super::port::LlmClient;
use super::types::{ChatCompletionRequest, ChatCompletionResponse, Role};

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub enum OpenAiModel {
    Gpt5_4,
    Gpt5_4Mini,
    Gpt5_4Nano,
    Gpt5_2,
    Gpt5_1,
    Gpt5,
    Gpt5Mini,
    Gpt5Nano,
}

impl OpenAiModel {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            OpenAiModel::Gpt5_4 => "gpt-5.4",
            OpenAiModel::Gpt5_4Mini => "gpt-5.4-mini",
            OpenAiModel::Gpt5_4Nano => "gpt-5.4-nano",
            OpenAiModel::Gpt5_2 => "gpt-5.2",
            OpenAiModel::Gpt5_1 => "gpt-5.1",
            OpenAiModel::Gpt5 => "gpt-5",
            OpenAiModel::Gpt5Mini => "gpt-5-mini",
            OpenAiModel::Gpt5Nano => "gpt-5-nano",
        }
    }
}

pub struct OpenAiClient {
    pub api_key: String,
    pub model: OpenAiModel,
    pub http_client: reqwest::Client,
}

#[async_trait]
impl LlmClient for OpenAiClient {
    async fn chat_completion(
        &self,
        req: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, String> {
        let mut messages = Vec::new();

        for msg in req.messages {
            let role_str = match msg.role {
                Role::System => "system",
                Role::User => "user",
                Role::Assistant => "assistant",
            };

            messages.push(serde_json::json!({
                "role": role_str,
                "content": msg.content,
            }));
        }

        let mut body = serde_json::json!({
            "model": self.model.as_str(),
            "messages": messages,
        });

        if let Some(tokens) = req.max_tokens {
            body["max_completion_tokens"] = serde_json::json!(tokens);
        }

        let res = self
            .http_client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json_res: Value = res.json().await.map_err(|e| e.to_string())?;
        tracing::debug!("JSON Res from OpenAI: {json_res}");
        let content = json_res["choices"][0]["message"]["content"]
            .as_str()
            .ok_or("Failed to parse response content from OpenAI")?
            .to_string();

        Ok(ChatCompletionResponse { content })
    }
}
