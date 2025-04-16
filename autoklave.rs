fn autoclave_encrypt(plaintext: &str, key: &str) -> String {
    let mut extended_key: Vec<char> = key.chars().collect();
    let mut result = String::new();

    for c in plaintext.chars() {
        if c.is_ascii_alphabetic() {
            let k = extended_key[result.chars().filter(|c| c.is_ascii_alphabetic()).count() % extended_key.len()];
            let p_val = c.to_ascii_uppercase() as i32 - 'A' as i32;
            let k_val = k.to_ascii_uppercase() as i32 - 'A' as i32;
            let encrypted = ((p_val + k_val) % 26) + 'A' as i32;
            let enc_char = char::from_u32(encrypted as u32).unwrap();
            result.push(enc_char);
            extended_key.push(c); // add plaintext to extended key
        } else {
            result.push(c);
        }
    }

    result
}

fn autoclave_decrypt(ciphertext: &str, key: &str) -> String {
    let mut result = String::new();
    let mut current_key: Vec<char> = key.chars().collect();

    for c in ciphertext.chars() {
        if c.is_ascii_alphabetic() {
            let k = current_key[result.chars().filter(|c| c.is_ascii_alphabetic()).count()];
            let c_val = c.to_ascii_uppercase() as i32 - 'A' as i32;
            let k_val = k.to_ascii_uppercase() as i32 - 'A' as i32;
            let decrypted = ((c_val - k_val + 26) % 26) + 'A' as i32;
            let decrypted_char = char::from_u32(decrypted as u32).unwrap();
            result.push(decrypted_char);
            current_key.push(decrypted_char); // grow key with decrypted letters
        } else {
            result.push(c);
        }
    }

    result
}

fn main() {
    let plaintext = "AUTOKLAVE OR RUNNING KEY CIPHER";
    let key = "CIA";

    let encrypted = autoclave_encrypt(plaintext, key);
    let decrypted = autoclave_decrypt(&encrypted, key);

    println!("Key       : {}", key);
    println!("Original  : {}", plaintext);
    println!("Encrypted : {}", encrypted);
    println!("Decrypted : {}", decrypted);
}
