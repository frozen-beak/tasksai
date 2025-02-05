use crate::{
    errors::AppError,
    prompts::{
        BUG_ANALYSIS_SYSTEM_PROMPT, PERFORMANCE_IMPROVEMENT_SYSTEM_PROMPT, PLAN_SYSTEM_PROMPT,
    },
};
use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::time::Duration;

pub struct GeminiClient {
    client: Client,
    api_key: String,
}

impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    /// Generate a detailed technical plan.
    pub fn generate_plan(&self, prompt: String) -> Result<String, AppError> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent?key={}",
            self.api_key
        );

        let payload = json!({
            "contents": [{
                "role": "user",
                "parts": [{ "text": prompt }]
            }],
            "systemInstruction": {
                "role": "system",
                "parts": [{ "text": PLAN_SYSTEM_PROMPT }]
            },
            "generationConfig": {
                "temperature": 1,
                "topK": 40,
                "topP": 0.95,
                "maxOutputTokens": 8192,
                "responseMimeType": "text/plain"
            }
        });

        let response_text = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .timeout(Duration::from_secs(60))
            .json(&payload)
            .send()?
            .error_for_status()?
            .text()?;

        extract_response_text_plain(&response_text)
    }

    /// Check provided code files for bugs and vulnerabilities.
    pub fn check_code(&self, code_input: String) -> Result<String, AppError> {
        let mut input_prompt = String::new();

        input_prompt.insert_str(0, &code_input);
        input_prompt.insert_str(0, "Check this code for potential bugs and security issues");

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent?key={}",
            self.api_key
        );

        // The payload includes a response schema to force JSON output.
        let payload = json!({
            "contents": [{
                "role": "user",
                "parts": [{ "text": input_prompt }]
            }],
            "systemInstruction": {
                "role": "system",
                "parts": [{ "text": BUG_ANALYSIS_SYSTEM_PROMPT }]
            },
            "generationConfig": {
                "temperature": 1,
                "topK": 40,
                "topP": 0.95,
                "maxOutputTokens": 8192,
                "responseMimeType": "application/json",
                "responseSchema": {
                    "type": "object",
                    "properties": {
                        "list": {
                            "type": "array",
                            "items": { "type": "string" }
                        }
                    },
                    "required": ["list"]
                }
            }
        });

        let response_text = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .timeout(Duration::from_secs(60))
            .json(&payload)
            .send()?
            .error_for_status()?
            .text()?;

        // In this case, we simply return the JSON response as a formatted string.
        extract_response_text_json(&response_text)
    }

    /// Check provided code files for bugs and vulnerabilities.
    pub fn improve_performance(&self, code_input: String) -> Result<String, AppError> {
        let mut input_prompt = String::new();

        input_prompt.insert_str(0, &code_input);
        input_prompt.insert_str(0, "Check this code for performance improvements");

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent?key={}",
            self.api_key
        );

        // The payload includes a response schema to force JSON output.
        let payload = json!({
            "contents": [{
                "role": "user",
                "parts": [{ "text": input_prompt }]
            }],
            "systemInstruction": {
                "role": "system",
                "parts": [{ "text": PERFORMANCE_IMPROVEMENT_SYSTEM_PROMPT }]
            },
            "generationConfig": {
                "temperature": 1,
                "topK": 40,
                "topP": 0.95,
                "maxOutputTokens": 8192,
                "responseMimeType": "application/json",
                "responseSchema": {
                    "type": "object",
                    "properties": {
                        "list": {
                            "type": "array",
                            "items": { "type": "string" }
                        }
                    },
                    "required": ["list"]
                }
            }
        });

        let response_text = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .timeout(Duration::from_secs(60))
            .json(&payload)
            .send()?
            .error_for_status()?
            .text()?;

        // In this case, we simply return the JSON response as a formatted string.
        extract_response_text_json(&response_text)
    }
}

/// Extracts text from a plain text Gemini API response.
fn extract_response_text_plain(response_body: &str) -> Result<String, AppError> {
    let parsed: Value = serde_json::from_str(response_body)
        .map_err(|e| AppError::InvalidResponse(e.to_string()))?;
    let text = parsed["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .ok_or_else(|| AppError::InvalidResponse("Missing text in response".to_string()))?
        .to_string();
    Ok(text)
}

/// Extracts the JSON response from a Gemini API response and formats it.
fn extract_response_text_json(response_body: &str) -> Result<String, AppError> {
    // println!("----------");
    // println!("{response_body}");
    // println!("----------");
    // Parse the outer response.
    let parsed: Value = serde_json::from_str(response_body)
        .map_err(|e| AppError::InvalidResponse(e.to_string()))?;

    // Extract the inner JSON string from the response.
    let inner_text = parsed["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .ok_or_else(|| AppError::InvalidResponse("Missing text in response".to_string()))?;

    // Parse the inner JSON string.
    let inner_json: Value =
        serde_json::from_str(inner_text).map_err(|e| AppError::InvalidResponse(e.to_string()))?;

    // Extract the bugs list; if it doesn't exist, default to an empty array.
    let bugs = inner_json
        .get("list")
        .cloned()
        .unwrap_or_else(|| Value::Array(vec![]));

    // Pretty-print the bugs list.
    let formatted = serde_json::to_string_pretty(&bugs)
        .map_err(|e| AppError::InvalidResponse(e.to_string()))?;

    Ok(formatted)
}
