use async_trait::async_trait;
use serde::Serialize;
use serde_json::Value;

use super::port::LlmClient;
use super::types::{ChatCompletionRequest, ChatCompletionResponse, Role};

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub enum GoogleModel {
    Gemini3_1ProPreview,
    Gemini3FlashLitePreview,
    Gemini3FlashPreview,
}

impl GoogleModel {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            GoogleModel::Gemini3_1ProPreview => "gemini-3.1-pro-preview",
            GoogleModel::Gemini3FlashLitePreview => "gemini-3.1-flash-lite-preview",
            GoogleModel::Gemini3FlashPreview => "gemini-3-flash-preview",
        }
    }
}

pub struct GoogleClient {
    pub api_key: String,
    pub model: GoogleModel,
    pub http_client: reqwest::Client,
}

#[async_trait]
impl LlmClient for GoogleClient {
    async fn chat_completion(
        &self,
        req: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, String> {
        let mut system_parts = Vec::new();
        let mut contents = Vec::new();

        for msg in req.messages {
            match msg.role {
                Role::System => {
                    system_parts.push(serde_json::json!({ "text": msg.content }));
                }
                Role::User => {
                    contents.push(serde_json::json!({
                        "role": "user",
                        "parts": [{ "text": msg.content }]
                    }));
                }
                Role::Assistant => {
                    contents.push(serde_json::json!({
                        "role": "model",
                        "parts": [{ "text": msg.content }]
                    }));
                }
            }
        }

        let mut body = serde_json::json!({
            "contents": contents,
        });

        if !system_parts.is_empty() {
            body["system_instruction"] = serde_json::json!({ "parts": system_parts });
        }

        if let Some(tokens) = req.max_tokens {
            body["generationConfig"] = serde_json::json!({ "maxOutputTokens": tokens });
        }

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model.as_str(),
            self.api_key
        );

        let res = self
            .http_client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json_res: Value = res.json().await.map_err(|e| e.to_string())?;
        tracing::debug!("JSON Res from Google: {json_res}");

        if let Some(error) = json_res.get("error") {
            return Err(format!("Google API error: {error:?}"));
        }

        let content = json_res["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or("Failed to parse response content from Google")?
            .to_string();

        Ok(ChatCompletionResponse { content })
    }
}
