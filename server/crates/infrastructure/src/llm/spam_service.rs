use application::contact_inquiry::ContactInquirySpamRatingPort;
use async_trait::async_trait;

use crate::llm_provider::port::LlmClient;
use crate::llm_provider::types::{ChatCompletionRequest, Message, Role};

pub struct ContactInquirySpamRatingAdapter<C: LlmClient> {
    pub llm_client: C,
}

const MAX_SCORE_ALLOWABLE: u8 = 100;

/*
 * TOD: Put entire submission in here. A simple message isn't enough context. Need the entire first name, email, website, etc.
 */
#[async_trait]
impl<C: LlmClient> ContactInquirySpamRatingPort for ContactInquirySpamRatingAdapter<C> {
    async fn get_spam_likelihood(&self, message: &str) -> Result<u8, String> {
        let system_message = Message {
            role: Role::System,
            content: "You are a spam detection system for a contact form. Analyze the submitted message and return a single integer from 0 to 100 representing the likelihood it is spam. 0 means definitely not spam. 100 means definitely spam. Output only the integer. No explanation, no punctuation, no percent sign, no decimal. Just the number.".into(),
        };
        let user_message = Message {
            role: Role::User,
            content: format!("Classify this contact form submission message:\n\n{message}"),
        };
        let req = ChatCompletionRequest {
            messages: vec![system_message, user_message],
            max_tokens: Some(500),
        };

        let response = self.llm_client.chat_completion(req).await?;

        // Parse the model's string reply into a u8
        let score: u8 = response
            .content
            .trim()
            .parse()
            .map_err(|_| format!("LLM returned non-numeric score: '{}'", response.content))?;

        if score > MAX_SCORE_ALLOWABLE {
            Err(format!("LLM returned non-numeric score: '{score}'"))
        } else {
            Ok(score)
        }
    }
}
