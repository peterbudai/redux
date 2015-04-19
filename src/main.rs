use std::io;
use std::env;

mod redux;

fn main() {
    let comp = if let Some(s) = env::args().nth(1) {
        s == "-c"
    } else {
        true
    };

    let mut input = io::stdin();
    let mut output  = io::stdout();
    if comp {
        match redux::compress(&mut input, &mut output) {
            Ok((i, o)) => { println!("Compressed {} bytes into {} bytes, compression ratio: {}%", i, o, (o as f64)/(i as f64) * 100f64); }
            Err(e) => { println!("error: {}", e); }
        }
    } else {
        match redux::decompress(&mut input, &mut output) {
            Ok((i, o)) => { println!("Decompressed {} bytes from {} bytes, compression ratio: {}%", o, i, (i as f64)/(o as f64) * 100f64); }
            Err(e) => { println!("error: {}", e); }
        }
    }
}
