fn caesar_cipher(text: &str, shift: i32, encrypt: bool) -> String {
    let shift = if encrypt { shift } else { -shift };
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let shifted = ((c as u8 - base + (shift.rem_euclid(26)) as u8) % 26) + base;
                shifted as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let plaintext = "Sabesh Raaj S";
    let shift = 3;

    let encrypted = caesar_cipher(plaintext, shift, true);
    let decrypted = caesar_cipher(&encrypted, shift, false);

    println!("Original: {}", plaintext);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
