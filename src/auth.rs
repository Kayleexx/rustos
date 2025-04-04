use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use serde::{Serialize, Deserialize};
use std::{collections::HashMap, fs, fs::File, io::Write};
use rpassword::read_password;

#[derive(Serialize, Deserialize)]
pub struct User {
    username: String,
    password_hash: String,
    salt: String,
}

#[derive(Serialize, Deserialize)]
pub struct Session {
    username: String,
}

pub fn load_users() -> HashMap<String, User> {
    fs::read_to_string("users.json")
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default()
}

pub fn save_users(users: &HashMap<String, User>) {
    let data = serde_json::to_string_pretty(users).unwrap();
    fs::write("users.json", data).unwrap();
}

pub fn save_session(username: &str) {
    let session = Session {
        username: username.to_string(),
    };
    let data = serde_json::to_string_pretty(&session).unwrap();
    fs::write("session.json", data).unwrap();
}

pub fn load_session() -> Option<String> {
    fs::read_to_string("session.json")
        .ok()
        .and_then(|data| serde_json::from_str::<Session>(&data).ok())
        .map(|session| session.username)
}

pub fn logout() {
    if fs::remove_file("session.json").is_ok() {
        println!("Successfully logged out.");
    } else {
        println!("No active session to log out from.");
    }
}

pub fn register(username: &str) {
    let mut users = load_users();

    if users.contains_key(username) {
        println!("Username '{}' already exists!", username);
        return;
    }

    println!("Enter a password:");
    let password = read_password().unwrap();

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let user = User {
        username: username.to_string(),
        password_hash,
        salt: salt.to_string(),
    };

    users.insert(username.to_string(), user);
    save_users(&users);

    println!("User '{}' registered successfully.", username);
}

pub fn login(username: &str) -> Option<String> {
    let users = load_users();
    if let Some(user) = users.get(username) {
        println!("Enter your password:");
        let password = read_password().unwrap();

        let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
        let argon2 = Argon2::default();

        if argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok() {
            println!("Login successful. Welcome, {}!", username);
            save_session(username); // save session on successful login
            return Some(username.to_string());
        } else {
            println!("Incorrect password.");
        }
    } else {
        println!("No such user found.");
    }
    None
}
