fn august_cipher(text: &str, encrypt: bool) -> String {
    let mut result = String::new();
    let mut shift = 1;

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let shift_amount = if encrypt { shift } else { -shift };
            let shifted = ((c as i32 - base as i32 + shift_amount).rem_euclid(26)) + base as i32;
            result.push(char::from_u32(shifted as u32).unwrap());
            shift += 1;
        } else {
            result.push(c);
        }
    }

    result
}

fn main() {
    let plaintext = "AUGUST Cipher";
    let encrypted = august_cipher(plaintext, true);
    let decrypted = august_cipher(&encrypted, false);

    println!("Original : {}", plaintext);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
