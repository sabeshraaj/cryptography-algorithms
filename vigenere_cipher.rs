fn vigenere_cipher(text: &str, key: &str, encrypt: bool) -> String {
    let mut result = String::new();
    let mut key_iter = key.chars().cycle();

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let k = key_iter.next().unwrap();
            let shift = (k.to_ascii_lowercase() as u8 - b'a') as i32;
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let shift = if encrypt { shift } else { -shift };
            let shifted = ((c as i32 - base as i32 + shift).rem_euclid(26)) + base as i32;
            result.push(char::from_u32(shifted as u32).unwrap());
        } else {
            result.push(c);
        }
    }

    result
}

fn main() {
    let plaintext = "Vigenere Cipher as plain text";
    let key = "KEY";

    let encrypted = vigenere_cipher(plaintext, key, true);
    let decrypted = vigenere_cipher(&encrypted, key, false);

    println!("Original : {}", plaintext);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
