fn gronsfeld_cipher(text: &str, key: &str, encrypt: bool) -> String {
    let mut result = String::new();
    let mut key_iter = key.chars().cycle();

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let k = key_iter.next().unwrap();
            let shift = k.to_digit(10).expect("Key must be numeric!") as i32;
            let shift = if encrypt { shift } else { -shift };
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let shifted = ((c as i32 - base as i32 + shift).rem_euclid(26)) + base as i32;
            result.push(char::from_u32(shifted as u32).unwrap());
        } else {
            result.push(c);
        }
    }

    result
}

fn main() {
    let plaintext = "This is for Gronsfeld Cipher";
    let key = "314159"; // repeat as needed

    let encrypted = gronsfeld_cipher(plaintext, key, true);
    let decrypted = gronsfeld_cipher(&encrypted, key, false);
    
    println!("the key used is: {}",key);
    println!("Original : {}", plaintext);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
