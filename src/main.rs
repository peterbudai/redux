use std::io::*;

mod redux;

fn main() {
    let mut input = std::io::stdin();
    let mut output  = std::io::stdout();
    match redux::compress(&mut input, &mut output) {
        Ok((i, o)) => { println!("Compressed {} bytes into {} bytes, compression ratio: {}%", i, o, (o as f64)/(i as f64) * 100f64); }
        Err(e) => { println!("error: {}", e); }
    }
}
