mod shell;
mod utils;
pub mod auth;
mod plugin;
mod plugin_manager;

use crate::plugin_manager::PluginManager;
use crate::commands::roast::RoastMe;

mod commands {
    pub mod crabgpt;
    pub mod encrypt;
    pub mod crabtop;
    pub mod games;
    pub mod roast;
}

use std::sync::Arc;
use tokio::sync::Mutex;
use axum::{
    extract::Extension,
    routing::post,
    Json, Router,
    http::StatusCode,
};
use tower::ServiceBuilder;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CommandRequest {
    command: String,
    input: String,
}

#[derive(Serialize)]
struct CommandResponse {
    response: String,
}

#[tokio::main]
async fn main() {
    println!("Booting rustOS...");
    println!("🦀 Welcome to rustOS - Crab-powered terminal");

    let plugin_manager = Arc::new(Mutex::new(PluginManager::new()));

    // Register the RoastMe plugin
    {
        let mut manager = plugin_manager.lock().await;
        manager.register_plugin(Box::new(RoastMe));
    }

    // Debugging: Print registered plugins
    {
        let manager = plugin_manager.lock().await;
        println!("Registered plugins: {:?}", manager.list_plugins());
    }

    let app = Router::new()
        .route("/run-command", post(run_command))
        .layer(ServiceBuilder::new().layer(Extension(plugin_manager.clone())));

    tokio::spawn(async move {
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    // ✅ Fixed function call issue
    shell::start_shell().await; // No extra argument
}

// ✅ Fixed HTTP command execution
async fn run_command(
    Extension(manager): Extension<Arc<Mutex<PluginManager>>>,
    Json(payload): Json<CommandRequest>,
) -> Result<Json<CommandResponse>, StatusCode> {
    let manager = manager.lock().await;
    let response = manager
        .execute_command(&payload.command, &payload.input) // ✅ Fixed function call
        .unwrap_or_else(|| "Unknown command".to_string());

    Ok(Json(CommandResponse { response }))
}
