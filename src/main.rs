mod fopen;
mod keygen;

use fopen::open_file;
use keygen::generate_key;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() > 1 {
        let chunks = open_file(std::path::PathBuf::from(&args[1])).unwrap();
        for chunk in chunks {
            let key = generate_key(&chunk);
            println!("{key:?}");
        }
    }
}
