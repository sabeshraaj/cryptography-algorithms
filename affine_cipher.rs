fn modinv(a: i32, m: i32) -> Option<i32> {
    for i in 1..m {
        if (a * i) % m == 1 {
            return Some(i);
        }
    }
    None
}

fn affine_encrypt(text: &str, a: i32, b: i32) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let x = c as i32 - base as i32;
                let enc = (a * x + b).rem_euclid(26) + base as i32;
                char::from_u32(enc as u32).unwrap()
            } else {
                c
            }
        })
        .collect()
}

fn affine_decrypt(text: &str, a: i32, b: i32) -> String {
    let a_inv = modinv(a, 26).expect("No modular inverse for a!");
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let y = c as i32 - base as i32;
                let dec = a_inv * (y - b);
                let dec_mod = dec.rem_euclid(26) + base as i32;
                char::from_u32(dec_mod as u32).unwrap()
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let plaintext = "Working of Affine Cipher";
    let a = 5;
    let b = 8;

    let encrypted = affine_encrypt(plaintext, a, b);
    let decrypted = affine_decrypt(&encrypted, a, b);

    println!("Original : {}", plaintext);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
