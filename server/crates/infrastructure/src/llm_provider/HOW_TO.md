# LLM Provider Implementation Guide

This guide walks through implementing the `LlmProvider` trait and wiring it into the
existing hexagonal architecture. It covers all three vendors (Anthropic, OpenAI, Google Gemini),
the Rust-specific gotchas you'll hit along the way, and how to wire everything into `main.rs`.

---

## Table of Contents

1. [Add Dependencies](#1-add-dependencies)
2. [Define the LlmProvider Trait](#2-define-the-llmprovider-trait)
3. [Implement the Three Providers](#3-implement-the-three-providers)
4. [Update the Spam Service Struct](#4-update-the-spam-service-struct)
5. [Wire It Into main.rs](#5-wire-it-into-mainrs)
6. [Environment Variables](#6-environment-variables)
7. [Common Rust Pitfalls](#7-common-rust-pitfalls)
8. [Vendor-Specific Gotchas](#8-vendor-specific-gotchas)
9. [Testing](#9-testing)

---

## 1. Add Dependencies

In `infrastructure/Cargo.toml`, add:

```toml
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }
serde_json = "1"
serde = { version = "1", features = ["derive"] }
```

### Why these features?

- `json` — lets you call `.json(&body)` on requests and `.json::<T>()` on responses.
- `rustls-tls` — pure-Rust TLS. Without a TLS feature, reqwest cannot make HTTPS requests
  and will fail at runtime with a confusing "no connector" error. The alternative is
  `native-tls` which uses your OS's TLS library (OpenSSL on Linux, Secure Transport on macOS),
  but `rustls-tls` is simpler to compile and has no system-level dependencies.

### Rust pitfall: Feature flags are additive

If you accidentally add `reqwest` twice (once here, once in another crate) with different
features, Cargo merges them. This is usually fine, but be aware that you can't *disable* a
feature another crate already enabled.

---

## 2. Define the LlmProvider Trait

**File:** `llm_provider/port.rs`

```rust
use serde_json::Value;

pub trait LlmProvider: Send + Sync {
    fn api_url(&self) -> &str;
    fn auth_headers(&self) -> Vec<(String, String)>;
    fn build_request_body(&self, system_prompt: &str, user_prompt: &str) -> Value;
    fn parse_response(&self, body: &str) -> Result<String, String>;
}
```

### Design decisions

- **`Send + Sync` bounds** — Required because this trait will be used inside an `Arc` that
  gets shared across Tokio tasks (which may run on different threads). Without these bounds,
  the compiler will reject your code when you try to use the struct in an async context.
  You'll see an error like:
  ```
  `*mut ()` cannot be shared between threads safely
  ```
  This is the compiler telling you a trait object doesn't satisfy `Send + Sync`.

- **`system_prompt` and `user_prompt` as separate params** — Each vendor puts the system
  instruction in a different place in the JSON. Passing them separately to `build_request_body`
  keeps the provider implementation clean. The caller (spam service) owns the prompt text;
  the provider only knows how to *format* it.

- **Returns `Vec<(String, String)>` for headers, not `(&str, &str)`** — Returning borrowed
  references from `auth_headers` creates lifetime headaches. The header values are built
  from `self.api_key` (formatted into `"Bearer {key}"`), so the formatted `String` would
  be dropped before the reference could be used. Using owned `String`s avoids this entirely.

### Rust pitfall: "temporary value dropped while borrowed"

If you try to return `&str` from a function that formats a string:

```rust
fn auth_header(&self) -> (&str, &str) {
    ("Authorization", &format!("Bearer {}", self.api_key))
    //                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ temporary String dropped here
}
```

The `format!` macro creates a temporary `String`. The reference to it can't outlive the
function. Solution: return `String` instead of `&str`, or store the formatted value in the
struct and return a reference to the field.

---

## 3. Implement the Three Providers

### 3a. Anthropic (`anthropic.rs`)

```rust
pub struct AnthropicProvider {
    pub api_key: String,
    pub model: String,   // e.g. "claude-sonnet-4-6"
}
```

**Endpoint:** `https://api.anthropic.com/v1/messages`

**Auth headers:**
```rust
vec![
    ("x-api-key".into(), self.api_key.clone()),
    ("anthropic-version".into(), "2023-06-01".into()),
    ("content-type".into(), "application/json".into()),
]
```

**Request body:**
```json
{
  "model": "claude-sonnet-4-6",
  "max_tokens": 16,
  "system": "You are a spam detection assistant...",
  "messages": [
    { "role": "user", "content": "Rate this message..." }
  ]
}
```

Key points:
- `system` is a **top-level string field**, NOT a message in the `messages` array.
- `max_tokens` is **required** — Anthropic will reject the request without it.
- The `anthropic-version` header is required. Without it you get a 400 error.

**Response path:** `response["content"][0]["text"]`

---

### 3b. OpenAI (`openai.rs`)

```rust
pub struct OpenAiProvider {
    pub api_key: String,
    pub model: String,   // e.g. "gpt-4o"
}
```

**Endpoint:** `https://api.openai.com/v1/chat/completions`

**Auth headers:**
```rust
vec![
    ("Authorization".into(), format!("Bearer {}", self.api_key)),
    ("Content-Type".into(), "application/json".into()),
]
```

**Request body:**
```json
{
  "model": "gpt-4o",
  "max_tokens": 16,
  "messages": [
    { "role": "system", "content": "You are a spam detection assistant..." },
    { "role": "user", "content": "Rate this message..." }
  ]
}
```

Key points:
- System prompt goes as the **first message** with `"role": "system"`.
- `max_tokens` is optional but recommended for cost control.

**Response path:** `response["choices"][0]["message"]["content"]`

---

### 3c. Google Gemini (`google.rs`)

```rust
pub struct GeminiProvider {
    pub api_key: String,
    pub model: String,     // e.g. "gemini-2.0-flash"
    pub endpoint: String,  // Pre-built full URL — see below
}
```

**Endpoint:** `https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent?key={api_key}`

The API key and model name are both in the URL. Because `api_url()` returns `&str`,
you can't `format!` inside the method (temporary value problem). Instead, build the
URL once at construction time and store it:

```rust
impl GeminiProvider {
    pub fn new(api_key: String, model: String) -> Self {
        let endpoint = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model, api_key
        );
        Self { api_key, model, endpoint }
    }
}
```

**Auth headers:** Empty `vec![]` — auth is in the URL.

**Request body:**
```json
{
  "system_instruction": {
    "parts": [{ "text": "You are a spam detection assistant..." }]
  },
  "contents": [
    {
      "role": "user",
      "parts": [{ "text": "Rate this message..." }]
    }
  ],
  "generationConfig": {
    "maxOutputTokens": 16
  }
}
```

Key points:
- Everything is wrapped in `"parts"` arrays — even single values.
- `system_instruction` is its own top-level key (not inside `contents`).
- `generationConfig` uses camelCase (not snake_case).
- The `role` values are `"user"` and `"model"` (not `"assistant"`).

**Response path:** `response["candidates"][0]["content"]["parts"][0]["text"]`

---

## 4. Update the Spam Service Struct

**File:** `contact_inquiry/spam_service.rs`

### Step A: Add the generic parameter

```rust
pub struct ContactInquirySpamRatingLLM<P: LlmProvider> {
    pub provider: P,
    pub http: reqwest::Client,
}
```

### Step B: Implement the domain port

```rust
#[async_trait]
impl<P: LlmProvider> ContactInquirySpamRatingPort for ContactInquirySpamRatingLLM<P> {
    async fn get_spam_likelihood(&self, message: &str) -> Result<u8, String> {
        // 1. Build the system and user prompts
        // 2. Call self.provider.build_request_body(system, user)
        // 3. Build the HTTP request:
        //      let mut req = self.http.post(self.provider.api_url());
        //      for (name, value) in self.provider.auth_headers() {
        //          req = req.header(name, value);
        //      }
        //      let resp = req.json(&body).send().await.map_err(|e| e.to_string())?;
        // 4. Check for non-2xx status BEFORE parsing body:
        //      let status = resp.status();
        //      let text = resp.text().await.map_err(|e| e.to_string())?;
        //      if !status.is_success() {
        //          return Err(format!("LLM API returned {}: {}", status, text));
        //      }
        // 5. Extract the model's reply:  self.provider.parse_response(&text)?
        // 6. Parse to u8:  reply.trim().parse::<u8>()
        // 7. Clamp:  score.min(100)
        todo!()
    }
}
```

### Rust pitfall: `impl<P: LlmProvider>` syntax

When your struct has a generic, the `impl` block must also declare the generic.
Forgetting this gives a confusing "cannot find type `P`" error:

```rust
// WRONG — missing generic on impl
impl ContactInquirySpamRatingPort for ContactInquirySpamRatingLLM<P> { ... }

// CORRECT
impl<P: LlmProvider> ContactInquirySpamRatingPort for ContactInquirySpamRatingLLM<P> { ... }
```

### Rust pitfall: `reqwest::RequestBuilder` is consumed on each method call

```rust
// WRONG — req is moved on the first .header(), can't call .header() again
let req = self.http.post(url);
req.header("x-api-key", key);   // moves req
req.header("anthropic-version", ver);  // ERROR: use of moved value

// CORRECT — reassign each time (builder pattern returns ownership)
let mut req = self.http.post(url);
for (name, value) in headers {
    req = req.header(name, value);
}
```

### The prompt

Define a helper function or constants:

```rust
const SYSTEM_PROMPT: &str = "\
You are a spam detection assistant. You will be given a message from a contact form submission. \
Respond with ONLY a single integer between 0 and 100. \
0 means the message is completely legitimate. \
100 means it is certainly spam or an advertisement. \
Do not include any other text, explanation, or punctuation — just the number.";

fn build_user_prompt(message: &str) -> String {
    format!("Rate the following contact form submission for spam likelihood:\n\n{message}")
}
```

---

## 5. Wire It Into main.rs

Currently the DI line looks like:

```rust
let contact_inquiry_spam =
    Arc::new(ContactInquirySpamRatingLLM {}) as Arc<dyn ContactInquirySpamRatingPort>;
```

After your changes, you'll pick a provider and instantiate:

```rust
let llm_api_key = std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must be set");

let provider = AnthropicProvider {
    api_key: llm_api_key,
    model: "claude-haiku-4-5-20251001".into(),  // cheap + fast for classification
};

let contact_inquiry_spam = Arc::new(ContactInquirySpamRatingLLM {
    provider,
    http: reqwest::Client::new(),
}) as Arc<dyn ContactInquirySpamRatingPort>;
```

To switch vendors, change the provider here. Nothing else in the codebase changes.

### Rust pitfall: `Arc<dyn Trait>` erases the generic

This is a good thing. The rest of your app sees `Arc<dyn ContactInquirySpamRatingPort>` —
it has no idea what `P` is. The generic only exists at the construction site in `main.rs`.
If the compiler complains that `ContactInquirySpamRatingLLM<AnthropicProvider>` doesn't
implement `ContactInquirySpamRatingPort`, double-check that your `impl<P: LlmProvider>`
block compiles and that `AnthropicProvider` satisfies `Send + Sync`.

### Rust pitfall: Creating `reqwest::Client` is expensive

`reqwest::Client::new()` allocates a connection pool and TLS context. Create it **once**
in `main.rs` and pass it into the struct. Never create a new `Client` per request. If
you later have multiple services needing HTTP, consider creating one `Client` in `main.rs`
and `.clone()`-ing it — clones share the same pool (it's an `Arc` internally).

---

## 6. Environment Variables

Add to your `.env`:

```bash
# Pick one (or all, if you want to toggle in code)
ANTHROPIC_API_KEY=sk-ant-...
OPENAI_API_KEY=sk-...
GOOGLE_AI_API_KEY=AI...
```

Read whichever one you need in `main.rs` with `std::env::var()`.

### Rust pitfall: `std::env::var` returns `Result<String, VarError>`

Don't use `.unwrap()` — use `.expect("MESSAGE")` so the panic message tells you *which*
variable is missing. In production, consider logging a clean error instead of panicking.

---

## 7. Common Rust Pitfalls

### 7a. Ownership with `serde_json::json!`

The `json!` macro moves values:

```rust
let model = self.model;  // assume this is String
let body = serde_json::json!({ "model": model });
// model is MOVED into the json Value — you can no longer use it

// Fix: clone, or use a reference (json! will call .into() on &str)
let body = serde_json::json!({ "model": &self.model });
```

Using `&self.model` works because `serde_json` implements `From<&str>` for `Value`.

### 7b. `.await` requires the future to be `Send`

If your async function holds a non-`Send` type (like `Rc` or `RefCell`) across an `.await`
point, the compiler will reject it with a very long error. Everything in `reqwest` is
`Send`-safe, so you shouldn't hit this. But if you add something like a `Rc<String>` to
your provider struct, you'll see this error. Solution: use `Arc<String>` instead of `Rc`.

### 7c. Error handling with `?` and `map_err`

`reqwest` errors and `serde_json` errors are different types. Your port returns
`Result<u8, String>`, so you need to convert both to `String`:

```rust
let resp = self.http.post(url).send().await.map_err(|e| e.to_string())?;
let text = resp.text().await.map_err(|e| e.to_string())?;
let json: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
```

The `?` operator propagates the `Err` variant early. Without `.map_err(|e| e.to_string())`
you'll get a type mismatch because `reqwest::Error` is not `String`.

### 7d. `parse::<u8>()` max value is 255, not 100

`u8` holds 0–255. The LLM should return 0–100 but could return, say, "150".
`"150".parse::<u8>()` succeeds (150 fits in u8), so add `.min(100)` to clamp.
However, `"300".parse::<u8>()` returns `Err` because 300 overflows. Handle both:

```rust
let raw = reply.trim();
let score: u8 = raw.parse().map_err(|_| format!("LLM returned unparseable score: '{raw}'"))?;
let score = score.min(100);
```

### 7e. `Content-Type` header

`reqwest`'s `.json(&body)` automatically sets `Content-Type: application/json`.
Do NOT manually set it in your `auth_headers` *in addition* to using `.json()` — it
works fine but is redundant. If you use `.body(string)` instead of `.json()`, then
you DO need to set the header manually.

---

## 8. Vendor-Specific Gotchas

### Anthropic

- **`max_tokens` is required.** Omitting it returns a 400.
- **`system` is a top-level string**, not a message object. Putting the system prompt
  inside `messages` as `{"role": "system", ...}` will return a 400 — Anthropic does not
  recognize "system" as a message role.
- **Rate limits** return 429 with a `retry-after` header. For a contact form, you probably
  won't hit this, but be aware.

### OpenAI

- **`max_tokens` is being replaced by `max_completion_tokens`** in newer models. For
  gpt-4o, both work. Check the docs if you switch models.
- The response may include `"finish_reason": "length"` if the model's reply was truncated.
  For a 1-3 digit number with `max_tokens: 16`, this shouldn't happen.

### Google Gemini

- **The API key is in the URL.** Be careful not to log the full URL in production.
- **`candidates` may be empty** if the model's safety filters trigger. A spam-checking
  prompt is unlikely to trip this, but the response would look like:
  ```json
  { "candidates": [], "promptFeedback": { "blockReason": "SAFETY" } }
  ```
  Handle this with a check like `candidates.get(0).ok_or("No response from Gemini")?`.
- **`role` is `"model"` not `"assistant"`** in Gemini's API. This only matters if you
  later do multi-turn conversations; for single-shot, you only send `"user"`.

---

## 9. Testing

### Unit testing `parse_response`

Each provider's `parse_response` is pure logic — no HTTP, no async. You can test it
directly with sample JSON strings:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anthropic_parses_valid_response() {
        let provider = AnthropicProvider {
            api_key: "unused".into(),
            model: "unused".into(),
        };
        let body = r#"{"content":[{"type":"text","text":"73"}]}"#;
        assert_eq!(provider.parse_response(body).unwrap(), "73");
    }
}
```

### Integration testing the full flow

For the actual HTTP call, consider:

1. **Mock server with `wiremock`** — spins up a local HTTP server that returns canned
   responses. You can assert that your code sends the right request shape.
2. **Live test behind a feature flag** — add `#[ignore]` to tests that hit real APIs,
   run them manually with `cargo test -- --ignored` when you want to verify end-to-end.

### Rust pitfall: async tests need `#[tokio::test]`

```rust
// WRONG — test will fail to compile (async fn in non-async context)
#[test]
async fn it_works() { ... }

// CORRECT
#[tokio::test]
async fn it_works() { ... }
```

You already have `tokio` in your dependencies with `macros` and `rt-multi-thread`, so
`#[tokio::test]` is available.


Done. The guide covers:

- **Dependency setup** — what to add to `Cargo.toml` and why each feature flag matters
- **Trait design** — why `Send + Sync`, why owned `String` returns instead of `&str`
- **All three vendors** — exact endpoints, auth patterns, request/response JSON shapes, and what's unique about each
- **Wiring into main.rs** — how the generic gets erased by `Arc<dyn Trait>` so the rest of the app is unaffected
- **7 Rust pitfalls** — temporary value lifetimes, move semantics with `json!`, `RequestBuilder` consumption, `u8` clamping, `map_err` for `?` operator, async `Send` bounds, and async test macros
- **Vendor-specific gotchas** — Anthropic's required `max_tokens` and non-standard system prompt placement, OpenAI's `max_completion_tokens` transition, Gemini's API-key-in-URL pattern and empty `candidates` on safety blocks
- **Testing** — unit testing `parse_response` directly, `wiremock` for integration tests, `#[tokio::test]
