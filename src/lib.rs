pub const CHUNK_SIZE: usize = if cfg!(target_pointer_width = "64") {
    64 * 1024
} else {
    32 * 1024
};
