use std::io::*;

mod redux;

fn main() {
    let mut input = std::io::stdin();
    let mut output  = std::io::stdout();
    redux::compress(&mut input, &mut output);
}
