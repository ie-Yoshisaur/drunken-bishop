use base64::Engine;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Reads an SSH public key file, extracts the base64-encoded part, and decodes it into raw bytes.
pub fn parse_ssh_pubkey(pubkey_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // 1. Open the file at the given path.
    let file = File::open(pubkey_path)?;
    // 2. Create a buffered reader to read the file line by line.
    let mut lines = BufReader::new(file).lines();

    // 3. Extract the first line, which is expected to contain the public key data.
    let line = match lines.next() {
        Some(Ok(l)) => l,
        _ => return Err("Failed to read pubkey file".into()),
    };

    // 4. Split the line by whitespace. A typical SSH pubkey line looks like:
    //    ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIJgu2rDTYkrZlP7Hj1unjmUhIHXJCuHs6/Z4iE2S/DIJ user@host
    let parts: Vec<&str> = line.split_whitespace().collect();
    // 5. Check that there are at least two parts: key type (e.g., "ssh-ed25519") and the base64-encoded key.
    if parts.len() < 2 {
        return Err("Invalid pubkey format".into());
    }

    // 6. Decode the base64-encoded string into raw bytes.
    let bin = base64::prelude::BASE64_STANDARD.decode(parts[1])?;
    Ok(bin)
}
