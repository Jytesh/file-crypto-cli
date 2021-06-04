use std::fs::{read_dir, write};
use std::io::stdin;

use std::fs;
use super::crypto;

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
    let mut passphrase = String::new();
    println!("Enter the desired password!");
    stdin().read_line(&mut passphrase).unwrap();
    let passphrase = passphrase.trim();
    return String::from(passphrase);
}

pub fn encrypt(input: String, output: String, password: String) -> Result<(), std::io::Error>{
    let read_string = fs::read_to_string(input)?;
    let ciphertext = crypto::enrypt_string(read_string, password).unwrap();
    write(output, ciphertext)?;
    Ok(())
}

pub fn decrypt(input: String, output: String, password: String) -> Result<(), std::io::Error>{
    let read_string = fs::read(input)?;
    let decrypted = crypto::decrypt_cipherstring(read_string, password).unwrap();
    write(output, decrypted)?;
    Ok(())
}
