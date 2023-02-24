pub const CHUNK_SIZE: usize = if cfg!(target_pointer_width = "64") {
    64 * 1024
} else {
    32 * 1024
};

pub type CHUNK = [u8; CHUNK_SIZE];
pub type KEYS = [u8; CHUNK_SIZE];
pub type CHUNKS = Vec<CHUNK>;
