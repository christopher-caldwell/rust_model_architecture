use async_trait::async_trait;
use serde::Serialize;
use serde_json::Value;

use super::port::LlmClient;
use super::types::{ChatCompletionRequest, ChatCompletionResponse, Role};

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub enum AnthropicModel {
    Claude4_6Opus,
    Claude4_6Sonnet,
    Claude4_5Haiku,
}

impl AnthropicModel {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            AnthropicModel::Claude4_6Opus => "claude-opus-4-6",
            AnthropicModel::Claude4_6Sonnet => "claude-sonnet-4-6",
            AnthropicModel::Claude4_5Haiku => "claude-haiku-4-5",
        }
    }
}

pub struct AnthropicClient {
    pub api_key: String,
    pub model: AnthropicModel,
    pub http_client: reqwest::Client,
}

#[async_trait]
impl LlmClient for AnthropicClient {
    async fn chat_completion(
        &self,
        req: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, String> {
        let mut system_prompts = Vec::new();
        let mut anthropic_messages = Vec::new();

        for msg in req.messages {
            match msg.role {
                Role::System => {
                    system_prompts.push(msg.content);
                }
                Role::User => {
                    anthropic_messages.push(serde_json::json!({
                        "role": "user",
                        "content": msg.content
                    }));
                }
                Role::Assistant => {
                    anthropic_messages.push(serde_json::json!({
                        "role": "assistant",
                        "content": msg.content
                    }));
                }
            }
        }

        let system_string = system_prompts.join("\n\n");

        let mut body = serde_json::json!({
            "model": self.model.as_str(),
            "max_tokens": req.max_tokens.unwrap_or(1024),
            "messages": anthropic_messages,
        });

        if !system_string.is_empty() {
            body["system"] = serde_json::Value::String(system_string);
        }

        // if let Some(temp) = req.temperature {
        //     body["temperature"] = serde_json::json!(temp);
        // }

        let res = self
            .http_client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json_res: Value = res.json().await.map_err(|e| e.to_string())?;

        if let Some(error) = json_res.get("error") {
            return Err(format!("Anthropic API error: {error:?}"));
        }

        let content = json_res["content"][0]["text"]
            .as_str()
            .ok_or("Failed to parse response content from Anthropic")?
            .to_string();

        Ok(ChatCompletionResponse { content })
    }
}
