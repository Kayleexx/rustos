mod shell;
mod utils;
pub mod auth;
mod commands {
    pub mod crabgpt;
    pub mod encrypt;
    pub mod crabtop;
    pub mod games;
}

#[tokio::main]
async fn main() {
    println!("Booting rustOS...");
    println!("🦀 Welcome to rustOS - Crab-powered terminal");
    shell::start_shell().await;
}
