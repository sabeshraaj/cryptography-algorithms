fn beaufort_cipher(text: &str, key: &str) -> String {
    let mut result = String::new();
    let mut key_iter = key.chars().cycle();

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let k = key_iter.next().unwrap().to_ascii_uppercase() as i32 - 'A' as i32;
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let p = c.to_ascii_uppercase() as i32 - 'A' as i32;
            let encrypted = ((k - p + 26).rem_euclid(26)) + base as i32;
            result.push(char::from_u32(encrypted as u32).unwrap());
        } else {
            result.push(c);
        }
    }

    result
}

fn main() {
    let plaintext = "Plan text : Beaufort Cipher";
    let key = "CIA";

    let encrypted = beaufort_cipher(plaintext, key);
    let decrypted = beaufort_cipher(&encrypted, key); // same function
    
    println!("Key : {}", key);
    println!("Original : {}", plaintext);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
