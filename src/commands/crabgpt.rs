use std::env;
use dotenvy::dotenv;
use reqwest::Client;
use serde_json::json;
use rand::Rng;

pub async fn handle_crabgpt(args: Vec<&str>) -> Result<String, String> {
    if args.len() < 1 {
        return Err("Usage: crabgpt <your question>".to_string());
    }

    dotenv().ok();
    let api_key = env::var("TOGETHER_API_KEY").map_err(|_| "TOGETHER_API_KEY not found in .env".to_string())?;

    let question = args.join(" ");

    let answer = generate_response(&question, &api_key).await.map_err(|e| e.to_string())?;
    let roasted_reply = wrap_with_crab_personality(&question, &answer);

    Ok(format!("🦀 crabGPT: {}", roasted_reply))
}

async fn generate_response(prompt: &str, api_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let body = json!({
        "model": "mistralai/Mixtral-8x7B-Instruct-v0.1",
        "prompt": format!("Answer casually but smartly: {}", prompt),
        "max_tokens": 200,
        "temperature": 0.7,
        "top_p": 0.9,
        "stop": null
    });

    let res = client
        .post("https://api.together.xyz/v1/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    let json: serde_json::Value = res.json().await?;
    let text = json["choices"][0]["text"]
        .as_str()
        .unwrap_or("🤖 Couldn't think of a response.")
        .trim()
        .to_string();

    Ok(text)
}

fn wrap_with_crab_personality(question: &str, answer: &str) -> String {
    let snarky_openings = [
        "Oh wow, *another* question?",
        "Hold my shell, let me answer that.",
        "I was enjoying my beach day, but fine...",
        "Ugh, humans and their curiosity.",
        "CrabGPT reporting in. Barely.",
    ];

    let closings = [
        "Now go do something productive. Or don't. 🦀",
        "You’re welcome. Send snacks. 🦀",
        "Remember, I’m smarter than you... probably.",
        "Back to clicking claws now.",
        "That’s all the wisdom you get today.",
    ];

    let mut rng = rand::thread_rng();
    let open = snarky_openings[rng.gen_range(0..snarky_openings.len())];
    let close = closings[rng.gen_range(0..closings.len())];

    format!(
        "{}\nQ: {}\nA: {}\n{}",
        open,
        question,
        answer.trim(),
        close
    )
}
