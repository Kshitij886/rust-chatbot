use serde_json::json;

pub async fn get_suggestions(api_key: &str, prompt: String) -> Result<String, reqwest::Error> {
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemma-3-27b-it:generateContent?key={}", api_key);
    let body = json!({
        "contents": [
            {
                "parts": [
                    {
                        "text": prompt
                    }
                ]
            }
        ]
    });
    let resp = reqwest::Client::new().post(url).json(&body).send().await?;
    let resp_text = resp.text().await?;
    Ok(resp_text)
}