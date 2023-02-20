mod fopen;

use fopen::open;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() > 1 {
        open(&args[1])
    }
}
