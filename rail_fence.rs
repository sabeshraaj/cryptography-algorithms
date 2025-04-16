fn rail_fence_encrypt(plaintext: &str, rails: usize) -> String {
    if rails == 1 {
        return plaintext.to_string(); // No change needed if only one rail
    }

    let chars: Vec<char> = plaintext.chars().collect();
    let mut rail = vec![Vec::new(); rails];

    let mut current_rail = 0;
    let mut going_down = false;

    // Distribute the characters into the rails in a zigzag pattern
    for &ch in chars.iter() {
        rail[current_rail].push(ch);

        if current_rail == 0 || current_rail == rails - 1 {
            going_down = !going_down; // Change direction at the top and bottom
        }

        current_rail = if going_down { current_rail + 1 } else { current_rail - 1 };
    }

    // Collect characters from the rails to form the ciphertext
    let mut result = String::new();
    for r in rail.iter() {
        for &ch in r.iter() {
            result.push(ch);
        }
    }

    result
}

fn rail_fence_decrypt(ciphertext: &str, rails: usize) -> String {
    if rails == 1 {
        return ciphertext.to_string(); // No change needed if only one rail
    }

    let chars: Vec<char> = ciphertext.chars().collect();
    let mut rail = vec![Vec::new(); rails];
    let mut rail_lengths = vec![0; rails];
    let mut current_rail = 0;
    let mut going_down = false;

    // Determine the lengths of the rails
    for &ch in chars.iter() {
        rail_lengths[current_rail] += 1;

        if current_rail == 0 || current_rail == rails - 1 {
            going_down = !going_down; // Change direction at the top and bottom
        }

        current_rail = if going_down { current_rail + 1 } else { current_rail - 1 };
    }

    // Fill the rail matrix with the ciphertext characters
    let mut index = 0;
    for r in 0..rails {
        for _ in 0..rail_lengths[r] {
            rail[r].push(chars[index]);
            index += 1;
        }
    }

    // Read the characters from the rail matrix in zigzag order to decrypt
    let mut result = String::new();
    current_rail = 0;
    going_down = false;
    for _ in 0..ciphertext.len() {
        result.push(rail[current_rail].remove(0)); // Take the first character from the current rail

        if current_rail == 0 || current_rail == rails - 1 {
            going_down = !going_down; // Change direction at the top and bottom
        }

        current_rail = if going_down { current_rail + 1 } else { current_rail - 1 };
    }

    result
}

fn main() {
    let plaintext = "This is code for RAIL FENCE CIPHER";
    let rails = 3;

    let encrypted = rail_fence_encrypt(plaintext, rails);
    let decrypted = rail_fence_decrypt(&encrypted, rails);

    println!("Plaintext : {}", plaintext);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
