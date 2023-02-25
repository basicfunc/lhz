mod huff_codec;
mod scrambler;
mod thresher;

use huff_codec::*;
use lhz::KEYS;
use scrambler::*;
use std::env;
use thresher::open;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() > 1 {
        let chunks = open(&args[1]).unwrap();

        let mut keys: Vec<KEYS> = vec![];

        for chunk in &chunks {
            let (d, m, y) = (
                *chunk.get(17).unwrap_or(&0),
                *chunk.get(01).unwrap_or(&0),
                *chunk.get(2001).unwrap_or(&0),
            );
            let s = d.wrapping_add(m).wrapping_add(y);

            keys.push(generate_key(s as u128));
        }

        let scrambled_data = scramble(&keys, &chunks);

        use std::collections::HashMap;

        for sd in &scrambled_data {
            let mut prefix = Vec::new();
            let mut codes = HashMap::new();
            let root = build_tree(sd);

            generate_codes(&root, &mut prefix, &mut codes);

            let encoded = encode(sd, &codes);

            let decoded = decode(&encoded, &root);
            let decoded = decoded.as_slice();
            assert_eq!(sd, decoded);

            if sd == decoded {
                println!("HUrray!")
            } else {
                println!("Ohno")
            }
        }

        println!("{}", chunks.len());

        let unscrambled_data = scramble(&keys, &scrambled_data);

        assert_eq!(chunks, unscrambled_data);
        // if chunks == unscrambled_data {
        //     println!("HUrray!")
        // } else {
        //     println!("Ohno")
        // }
    }
}
