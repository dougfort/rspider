extern crate hex;

pub fn from_hex(hex_string: &str) -> Result<[u8; 16], hex::FromHexError> {
    let seed_vec = hex::decode(hex_string)?;
    let mut seed: [u8; 16] = [0; 16];
    for (i, val) in seed_vec.iter().enumerate() {
        seed[i] = *val;
    }

    Ok(seed)
}