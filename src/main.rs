use std::io::{stdin, stdout, Write};
use rand::Rng;

fn encrypt(message: &str, key: u32) -> String {
    let mut encrypted = String::new();
    for c in message.chars() {
        if c.is_ascii_alphabetic() {
            let shift = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let c = (c as u32 - shift as u32 + key) % 26 + shift as u32;
            encrypted.push(c as u8 as char);
        } else {
            encrypted.push(c);
        }
    }
    encrypted
}

fn decrypt(message: &str, key: u32) -> String {
    let mut decrypted = String::new();
    for c in message.chars() {
        if c.is_ascii_alphabetic() {
            let shift = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let c = (c as u32 - shift as u32 + 26 - key) % 26 + shift as u32;
            decrypted.push(c as u8 as char);
        } else {
            decrypted.push(c);
        }
    }
    decrypted
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut key = rng.gen_range(4..625);
    println!("Your random key is: {}", key);
    loop {
        print!("What would you like to do? (e/d/r): ");
        stdout().flush().ok().expect("Could not flush stdout");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "e" {
            print!("Enter a message to encrypt: ");
            stdout().flush().ok().expect("Could not flush stdout");
            let mut message = String::new();
            stdin().read_line(&mut message).unwrap();
            let message = message.trim();
            let encrypted = encrypt(message, (key as f64).sqrt() as u32);
            println!("{:03}{}", key, encrypted);
        } else if input == "d" {
            print!("Enter a message to decrypt: ");
            stdout().flush().ok().expect("Could not flush stdout");
            let mut message = String::new();
            stdin().read_line(&mut message).unwrap();
            let message = message.trim();
            // Split message into key and text
            let (tkey, tmessage) = message.split_at(3);
            let dkey: f64 = match tkey.trim().parse() {
                Ok(v) => v,
                Err(_) => 0.0
            };
            let dmessage = tmessage.trim();
            let decrypted = decrypt(dmessage, dkey.sqrt() as u32);
            println!("Your decrypted message is: {}", decrypted);
        } else if input == "r" {
            key = rng.gen_range(4..625);
            println!("Your random key is: {}", key);
        } else {
            println!("Invalid input. Please try again.");
        }
    }
}