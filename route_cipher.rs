fn route_cipher_encrypt(plaintext: &str, rows: usize, cols: usize) -> String {
    let mut matrix = vec![vec![' '; cols]; rows];
    let mut chars = plaintext.chars();

    for i in 0..rows {
        for j in 0..cols {
            if let Some(ch) = chars.next() {
                matrix[i][j] = ch;
            }
        }
    }

    let mut encrypted = String::new();
    for j in 0..cols {
        for i in 0..rows {
            encrypted.push(matrix[i][j]);
        }
    }

    encrypted
}

fn route_cipher_decrypt(ciphertext: &str, rows: usize, cols: usize) -> String {
    let mut matrix = vec![vec![' '; cols]; rows];
    let mut chars = ciphertext.chars();

    
    for j in 0..cols {
        for i in 0..rows {
            if let Some(ch) = chars.next() {
                matrix[i][j] = ch;
            }
        }
    }

    let mut decrypted = String::new();
    for i in 0..rows {
        for j in 0..cols {
            decrypted.push(matrix[i][j]);
        }
    }

    decrypted
}

fn main() {
    let plaintext = "This is for Route Cipher";
    let rows = 5;
    let cols = 5;

    let encrypted = route_cipher_encrypt(plaintext, rows, cols);
    let decrypted = route_cipher_decrypt(&encrypted, rows, cols);

    println!("Plaintext: {}", plaintext);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
