use base64::{engine::general_purpose, Engine};

pub fn encrypt_file(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Usage: encrypt <text>");
        return;
    }
    let data = args[1..].join(" ");
    let encoded = general_purpose::STANDARD.encode(data);
    println!("Encrypted: {}", encoded);
}

pub fn decrypt_file(args: Vec<&str>) {
    if args.len() < 2 {
        println!("Usage: decrypt <base64_string>");
        return;
    }

    let encoded = args[1];
    match general_purpose::STANDARD.decode(encoded) {
        Ok(bytes) => println!("Decrypted: {}", String::from_utf8_lossy(&bytes)),
        Err(_) => println!("Failed to decode. Crab says nope."),
    }
}
