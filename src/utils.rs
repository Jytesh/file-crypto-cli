use std::fs::{read_dir, write};

use super::crypto;
use super::ux;
use std::fs;

pub fn readdir(path: String) -> Result<Vec<String>, std::io::Error> {
    let mut array = vec![];
    let entries = read_dir(path)?;
    for path in entries {
        let file = path?.path();
        let file = file.display();
        array.push(format!("{}", file))
    }
    return Ok(array);
}

pub fn getpass() -> String {
    let passphrase = ux::pass("Enter your password").unwrap();
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
