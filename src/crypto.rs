use orion::aead as crypto;

use orion::kdf;

pub fn enrypt_string(
    string: String,
    password: String,
) -> Result<Vec<u8>, orion::errors::UnknownCryptoError> {
    println!("password {:?}", password);

    let passphrase = stretch_passphrase(password); 
    println!("Passphrase {:?}", passphrase.key.unprotected_as_bytes());
    let ciphertext = crypto::seal(&passphrase.key, &string.as_bytes()).expect("Failed to encrypt text");
    println!("Ciphertext {:?}", ciphertext);
    let cipertext_with_salt = [passphrase.salt.as_ref(), &ciphertext].concat();
    Ok(cipertext_with_salt)
}

pub fn decrypt_cipherstring(
    cipherstring: Vec<u8>,
    password: String,
) -> Result<Vec<u8>, orion::errors::UnknownCryptoError> {
    println!("password {:?}", password);
    let (salt, cipherstring) = cipherstring.split_at(16);
    let passphrase = stretch_passphrase_with_salt(password, salt); 
    println!("Passphrase {:?}", passphrase.unprotected_as_bytes());
    println!("Ciphertext {:?}", cipherstring);

    Ok(
        crypto::open(&passphrase, &cipherstring)
            .expect("Failed to decrypt text"),
    )
}
struct KeySalt {
    key: crypto::SecretKey,
    salt: kdf::Salt
}

fn stretch_passphrase(passphrase: String) -> KeySalt {
    let user_password = kdf::Password::from_slice(passphrase.as_bytes()).unwrap();
    let salt = kdf::Salt::default();

    let key = kdf::derive_key(&user_password, &salt, 3, 1<<16, 32).unwrap();
    return KeySalt { key, salt };
}

fn stretch_passphrase_with_salt(passphrase: String, salt: &[u8]) -> crypto::SecretKey {
    let salt = kdf::Salt::from_slice(salt).unwrap();
    let user_password = kdf::Password::from_slice(passphrase.as_bytes()).unwrap();
    kdf::derive_key(&user_password, &salt, 3, 1<<16, 32).unwrap()
}