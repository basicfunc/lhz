use lfhuffzip::CHUNK_SIZE;

const LFSR_SEED: u64 = 0xACE1; // LFSR initial seed value
const LFSR_POLY: u64 = 0x95A9; // LFSR polynomial

fn lfsr(mut state: u64) -> u64 {
    let bit = state.wrapping_mul(LFSR_POLY).count_ones() & 1;
    state = (state >> 1) | ((bit as u64) << 63);
    state
}

pub fn generate_key(data_block: &[u8]) -> [u8; CHUNK_SIZE] {
    let mut key = [0u8; CHUNK_SIZE];
    let mut lfsr_state = LFSR_SEED;

    for idx in 0..data_block.len() {
        lfsr_state = lfsr(lfsr_state);
        key[idx] = (lfsr_state & 0xFF) as u8;
    }

    key
}
