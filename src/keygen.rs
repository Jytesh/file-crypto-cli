use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;
use openssl::symm::Cipher;
use std::fs::write;
use std::io::stdin;

pub fn keygen() {
    let pair = generate_pair();
    let mut passphrase = String::new();
    println!("Enter the desired password!");
    stdin().read_line(&mut passphrase).unwrap();
    let passphrase = passphrase.trim();

    println!("Your passphrase is {}", passphrase);

    let priv_key: Vec<u8> = pair
        .private_key_to_pem_pkcs8_passphrase(Cipher::aes_128_cbc(), &passphrase[..].as_bytes())
        .unwrap();
    let pub_key: Vec<u8> = pair.public_key_to_pem().unwrap();

    let write_path = format!(
        "{}.id_rsa",
        std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string()
    );

    write(&write_path, priv_key).unwrap();
    write(format!("{}.pub", &write_path), pub_key).unwrap();
}

fn generate_pair() -> PKey<Private> {
    // Generate a keypair
    let keypair = Rsa::generate(2048).unwrap();

    let keypair = PKey::from_rsa(keypair).unwrap();
    keypair
}
