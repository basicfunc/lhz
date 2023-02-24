mod fopen;
mod keygen;

use fopen::open_file;
use keygen::generate_key;
use lhz::KEYS;
use std::env;

use crate::keygen::scramble;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() > 1 {
        let chunks = open_file(std::path::PathBuf::from(&args[1])).unwrap();

        let mut keys: Vec<KEYS> = vec![];

        for chunk in &chunks {
            keys.push(generate_key(chunk[0] as u128));
        }

        let scrambled_data = scramble(&keys, &chunks);

        let unscrambled_data = scramble(&keys, &scrambled_data);

        assert_eq!(chunks, unscrambled_data);
    }
}
