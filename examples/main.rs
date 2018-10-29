extern crate base64;
extern crate chrono;
extern crate crypto;
extern crate csrf_token;

use chrono::Duration;
use csrf_token::{default_csrf_token_generator, CsrfTokenVerification};
use std::io::{stdin, stdout, Write};

fn secret() -> Vec<u8> {
    b"0123456789abcedf0123456789abcdef".to_vec()
}

fn main() {
    let mut generator = default_csrf_token_generator(secret(), Duration::minutes(10));

    let token = generator.generate();
    let token_encoded = base64::encode(&token);
    println!(
        "This token should be embedded in response body: {}",
        token_encoded
    );

    print!("Input a token sent to the server: ");
    stdout().flush().unwrap();
    let mut given_token = String::new();
    stdin().read_line(&mut given_token).unwrap();
    let given_token = given_token.trim_end();

    match base64::decode(&given_token) {
        Ok(decoded) => match generator.verify(&decoded) {
            CsrfTokenVerification::Success => println!("Verification success"),
            CsrfTokenVerification::Expired => println!("Verification failed: the token is expired"),
            CsrfTokenVerification::Invalid => println!("Verification failed: the token is invalid"),
        },
        Err(_) => println!("base64 decode error"),
    }
}
