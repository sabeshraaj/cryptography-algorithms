fn atbash_cipher(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                (b'Z' - (c as u8 - b'A')) as char
            } else if c.is_ascii_lowercase() {
                (b'z' - (c as u8 - b'a')) as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let plaintext = "This is for atbash cipher";
    let encrypted = atbash_cipher(plaintext);
    let decrypted = atbash_cipher(&encrypted);

    println!("Original: {}", plaintext);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
