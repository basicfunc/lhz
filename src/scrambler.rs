use lhz::{CHUNK, CHUNKS, CHUNK_SIZE, KEYS};

const LFSR_POLY: u128 = 0xB5AD4ECEDA1CE2A9F7AA6EA63B8D4C84;

pub fn generate_key(seed: u128) -> KEYS {
    // Determine the size of the key (in bytes)
    let key_size = CHUNK_SIZE;

    // Initialize the LFSR state to the seed value
    let mut lfsr_state = seed;

    // Generate the pseudo-random key
    let mut key = [0; CHUNK_SIZE];
    for idx in 0..key_size {
        let mut byte = 0u8;
        for bit_index in 0..8 {
            let lsb = lfsr_state & 1;
            lfsr_state >>= 1;
            if lsb == 1 {
                lfsr_state = lfsr_state ^ LFSR_POLY;
            }
            byte = byte | (lsb << bit_index) as u8;
        }
        key[idx] = byte;
    }

    // Return the generated key
    key
}

pub fn scramble(keys: &Vec<KEYS>, data: &CHUNKS) -> CHUNKS {
    let mut scrambled_data: CHUNKS = vec![];

    for (k, d) in keys.iter().zip(data.iter()) {
        let mut s_data: CHUNK = [0; CHUNK_SIZE];
        for (idx, (key, data)) in k.iter().zip(d.iter()).enumerate() {
            s_data[idx] = data ^ key;
        }
        scrambled_data.push(s_data)
    }

    scrambled_data
}
