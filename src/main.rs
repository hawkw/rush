extern crate rush;
use std::io;

fn main() {
    rush::Env::new().unwrap()
        .run_loop().unwrap();
}
