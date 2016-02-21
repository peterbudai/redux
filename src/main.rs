extern crate redux;

use std::io;
use std::io::Write;
use std::env;

use redux::model::Parameters;
use redux::model::AdaptiveTreeModel;

macro_rules! printlne(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

fn main() {
    let comp = if let Some(s) = env::args().nth(1) {
        if s == "-c" {
            true
        } else if s == "-d" {
            false
        } else {
            printlne!("Invalid argument");
            return;
        }
    } else {
        printlne!("Invalid argument");
        return;
    };

    let mut input = io::stdin();
    let mut output  = io::stdout();
    let model = AdaptiveTreeModel::new(Parameters::new(8, 30, 32).unwrap());
    if comp {
        match redux::compress(&mut input, &mut output, model) {
            Ok((i, o)) => { printlne!("Compressed {} bytes into {} bytes, compression ratio: {}%", i, o, (o as f64)/(i as f64) * 100f64); },
            Err(e) => { printlne!("Error: {}", e); }
        }
    } else {
        match redux::decompress(&mut input, &mut output, model) {
            Ok((i, o)) => { printlne!("Decompressed {} bytes from {} bytes, compression ratio: {}%", o, i, (i as f64)/(o as f64) * 100f64); },
            Err(e) => { printlne!("Error: {}", e); }
        }
    }
}
