use std::fs::write;

use super::crypto;
use super::ux;
use std::fs;

pub fn getpass() -> String {
    let passphrase = ux::input("Enter your password").unwrap();
    let passphrase = passphrase.trim();
    return String::from(passphrase);
}

pub fn encrypt(input: String, output: String, password: &String) -> Result<(), std::io::Error> {
    let read_string = fs::read(input)?;
    let ciphertext = crypto::enrypt_string(read_string, password).unwrap();
    write(output, ciphertext)?;
    Ok(())
}

pub fn decrypt(input: String, output: String, password: &String) -> Result<(), std::io::Error> {
    let read_string = fs::read(input)?;
    let decrypted = crypto::decrypt_cipherstring(read_string, password).unwrap();
    write(output, decrypted)?;
    Ok(())
}
