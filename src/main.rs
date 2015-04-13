use std::io::*;

mod redox;

fn main() {
    let mut input = std::io::stdin();
    let mut output  = std::io::stdout();
    redox::compress(&mut input, &mut output);
}
