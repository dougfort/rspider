
use rand::random;

pub fn from_hex(hex_string: &str) -> Result<[u8; 16], hex::FromHexError> {
    let seed_vec = hex::decode(hex_string)?;
    let mut seed: [u8; 16] = [0; 16];
    for (i, val) in seed_vec.iter().enumerate() {
        if i < 16 {
            seed[i] = *val;
        }
    }

    Ok(seed)
}

pub fn from_random() -> [u8; 16] {
    let mut seed: [u8; 16] = [0; 16];
    for i in 0..16 {
        seed[i] = random::<u8>();
    }

    seed
}