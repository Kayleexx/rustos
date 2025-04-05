use crate::utils;
use crate::commands::*;
use crate::auth;
use std::io::{self, Write};
use crate::plugin::Plugin;

pub async fn start_shell() {
    let mut username: Option<String> = None;

    loop {
        print!("{}", "rustOS > ".to_string());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let args: Vec<&str> = input.split_whitespace().collect();

        if args.is_empty() {
            continue;
        }

        match args[0] {
            "register" => {
                if let Some(name) = args.get(1) {
                    auth::register(name);
                } else {
                    println!("Usage: register <username>");
                }
            },
            "login" => {
                if let Some(name) = args.get(1) {
                    if let Some(user) = auth::login(name) {
                        username = Some(user);
                    }
                } else {
                    println!("Usage: login <username>");
                }
            },
            "roast-me" => {
                let roast = roast::RoastMe {}; // Ensure correct struct initialization
                if let Some(name) = &username {
                    println!("{}", roast.execute(name.to_string()));
                } else {
                    println!("{}", roast.execute("stranger".to_string()));
                }
            },
            "logout" => {
                println!("Goodbye, {}", username.clone().unwrap_or("user".to_string()));
                username = None;
            },
            "summon" => {
                if let Some(&"crab") = args.get(1) {
                    utils::summon_crab();
                }
            },
            "panic" => utils::panic_mode(),
            "crabgpt" => crabgpt::handle_crabgpt(args),
            "encrypt" => encrypt::encrypt_file(args),
            "decrypt" => encrypt::decrypt_file(args),
            "crabtop" => crabtop::show_tasks(),
            "play" => games::play_game(args),
            "help" => utils::help(),
            "exit" => {
                println!("Shutting down rustOS...");
                break;
            },
            _ => println!("Command not found. Type `help`."),
        }
    }        
}