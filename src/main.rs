use passgen::*;
use std::env;

fn main() {
    let config = env::args().collect::<Vec<String>>();
    run(config);
}
