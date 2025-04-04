pub fn handle_crabgpt(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Usage: crabgpt <your question>");
        return;
    }

    let question = args[1..].join(" ");
    println!("🤖 crabGPT: {}", generate_response(&question));
}

fn generate_response(question: &str) -> String {
    if question.contains("weather") {
        "It's probably raining somewhere. Bring a shell.".to_string()
    } else if question.contains("love") {
        "Crabs don't love. They click.".to_string()
    } else {
        format!("I'm just a sarcastic crab, not ChatGPT. You said: {}", question)
    }
}
