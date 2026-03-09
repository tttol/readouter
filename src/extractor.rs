use anyhow::{Context, Result};
use aws_sdk_bedrockruntime::primitives::Blob;
use aws_sdk_bedrockruntime::Client;

/// Extracts the content of the <title> tag from HTML, sanitized for use as a filename.
pub fn extract_title(html: &str) -> String {
    let title = html
        .find("<title>")
        .and_then(|start| {
            let rest = &html[start + 7..];
            rest.find("</title>").map(|end| rest[..end].trim().to_string())
        })
        .unwrap_or_else(|| "untitled".to_string());
    title
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect()
}

/// Fetches raw HTML content from the given URL using an async HTTP GET request.
pub async fn fetch_html(url: &str) -> Result<String> {
    let html = reqwest::get(url)
        .await
        .context("Failed to fetch URL")?
        .text()
        .await
        .context("Failed to read response body")?;
    Ok(html)
}

/// Sends the raw HTML to Amazon Bedrock (Claude) and returns the extracted plain article text,
/// stripping navigation, ads, headers, footers, and other non-article content.
pub async fn extract_article(client: &Client, model_id: &str, html: &str) -> Result<String> {
    let prompt = format!(
        "Extract only the main article body text from this HTML. \
        Remove all navigation, ads, headers, footers, and sidebars. \
        Return plain text only.\n\nHTML:\n{}",
        html
    );
    let request_body = serde_json::json!({
        "anthropic_version": "bedrock-2023-05-31",
        "max_tokens": 4096,
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ]
    });
    let body_bytes = serde_json::to_vec(&request_body).context("Failed to serialize request")?;
    let response = client
        .invoke_model()
        .model_id(model_id)
        .content_type("application/json")
        .body(Blob::new(body_bytes))
        .send()
        .await
        .context("Failed to invoke Bedrock model")?;
    let response_bytes = response.body().as_ref();
    let response_json: serde_json::Value =
        serde_json::from_slice(response_bytes).context("Failed to parse Bedrock response")?;
    let text = response_json["content"][0]["text"]
        .as_str()
        .context("Failed to extract text from Bedrock response")?
        .to_string();
    Ok(text)
}
