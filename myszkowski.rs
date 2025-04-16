fn get_key_order(key: &str) -> Vec<usize> {
    let mut key_chars: Vec<(usize, char)> = key.chars().enumerate().collect();
    key_chars.sort_by_key(|&(_, ch)| ch);

    let mut order = vec![0; key.len()];
    let mut current_rank = 0;
    for i in 0..key.len() {
        if i > 0 && key_chars[i].1 != key_chars[i - 1].1 {
            current_rank += 1;
        }
        order[key_chars[i].0] = current_rank;
    }
    order
}

fn encrypt(plaintext: &str, key: &str) -> String {
    let key_order = get_key_order(key);
    let cols = key.len();
    let rows = (plaintext.len() + cols - 1) / cols;

    let mut matrix = vec![vec![' '; cols]; rows];

    for (i, ch) in plaintext.chars().enumerate() {
        matrix[i / cols][i % cols] = ch;
    }

    let mut result = String::new();
    let mut rank = 0;
    while key_order.contains(&rank) {
        for (i, &r) in key_order.iter().enumerate() {
            if r == rank {
                for row in 0..rows {
                    let c = matrix[row][i];
                    if c != ' ' {
                        result.push(c);
                    }
                }
            }
        }
        rank += 1;
    }

    result
}

fn decrypt(ciphertext: &str, key: &str) -> String {
    let key_order = get_key_order(key);
    let cols = key.len();
    let rows = (ciphertext.len() + cols - 1) / cols;
    let mut matrix = vec![vec![' '; cols]; rows];

    let mut lengths = vec![0; cols];
    let mut rank = 0;
    let mut total = 0;

    while key_order.contains(&rank) {
        for (i, &r) in key_order.iter().enumerate() {
            if r == rank {
                for row in 0..rows {
                    if row * cols + i < ciphertext.len() {
                        lengths[i] += 1;
                        total += 1;
                    }
                }
            }
        }
        rank += 1;
    }

    let mut index = 0;
    rank = 0;
    while key_order.contains(&rank) {
        for (i, &r) in key_order.iter().enumerate() {
            if r == rank {
                for row in 0..rows {
                    if lengths[i] > 0 {
                        matrix[row][i] = ciphertext.chars().nth(index).unwrap();
                        index += 1;
                        lengths[i] -= 1;
                    }
                }
            }
        }
        rank += 1;
    }

    let mut result = String::new();
    for row in matrix {
        for c in row {
            if c != ' ' {
                result.push(c);
            }
        }
    }

    result
}

fn main() {
    let key = "CIAISTHEKEY";
    let plaintext = "THISISAPLAINTEXT";

    let encrypted = encrypt(plaintext, key);
    let decrypted = decrypt(&encrypted, key);

    println!("Key       : {}", key);
    println!("Plaintext : {}", plaintext);
    println!("Encrypted : {}", encrypted);
    println!("Decrypted : {}", decrypted);
}
