extern crate redux;

use std::io;
use std::io::Write;
use std::fs;
use std::path::PathBuf;
use std::env;
use std::process;

use redux::model::Parameters;
use redux::model::AdaptiveTreeModel;

macro_rules! println_err {
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
}

macro_rules! exit_with_error(
    ($code:expr, $($arg:tt)*) => {
        println_err!($($arg)*);
        process::exit($code);
    }
);

struct Options {
    compress: Option<bool>,
    input: Option<PathBuf>,
    output: Option<PathBuf>,
}

impl Options {
    fn from_args() -> Option<Options> {
        let mut options = Options { compress: None, input: None, output: None };
        let mut args = env::args().skip(1);
        while let Some(arg) = args.next() {
            match arg.as_ref() {
                "-c" => { options.compress = Some(true); },
                "-d" => { options.compress = Some(false); },
                "-i" => { 
                    if let Some(val) = args.next() {
                        options.input = Some(PathBuf::from(val));
                    } else {
                        return None;
                    }
                }
                "-o" => { 
                    if let Some(val) = args.next() {
                        options.output = Some(PathBuf::from(val));
                    } else {
                        return None;
                    }
                }
                _ => { return None; }
            }
        }
        if let None = options.compress { None } else { Some(options) }
    }

    fn is_compress(&self) -> bool {
        self.compress.unwrap()
    }

    fn is_stdin(&self) -> bool {
        self.input.is_none()
    }

    fn is_stdout(&self) -> bool {
        self.output.is_none()
    }

    fn input_file(&self) -> PathBuf {
        self.input.clone().unwrap()
    }

    fn output_file(&self) -> PathBuf {
        self.output.clone().unwrap()
    }
}

fn main() {
    let options = match Options::from_args() {
        Some(o) => o,
        None => { exit_with_error!(1, "Usage: redux (-c | -d) [-i <input file>] [-o <output file>]"); }
    };

    let mut input: Box<io::Read> = if options.is_stdin() {
        Box::new(io::stdin())
    } else {
        match fs::File::open(options.input_file()) {
            Ok(f) => Box::new(f),
            Err(e) => { exit_with_error!(2, "Error while opening input file {}: {}", options.output_file().to_string_lossy(), e); } 
        }
    };

    let mut output: Box<io::Write> = if options.is_stdout() {
        Box::new(io::stdout())
    } else {
        match fs::File::create(options.output_file()) {
            Ok(f) => Box::new(f),
            Err(e) => { exit_with_error!(2, "Error while opening output file {}: {}", options.output_file().to_string_lossy(), e); } 
        }
    };

    let model = AdaptiveTreeModel::new(Parameters::new(8, 30, 32).unwrap());

    if options.is_compress() {
        match redux::compress(&mut input, &mut output, model) {
            Ok((i, o)) => { println_err!("Compressed {} bytes into {} bytes, ratio: {:.3}", i, o, (i as f64)/(o as f64)); },
            Err(e) => { exit_with_error!(3, "Compression error: {}", e); }
        }
    } else {
        match redux::decompress(&mut input, &mut output, model) {
            Ok((i, o)) => { println_err!("Decompressed {} bytes from {} bytes, ratio: {:.3}", o, i, (o as f64)/(i as f64)); },
            Err(e) => { exit_with_error!(3, "Decompression error: {}", e); }
        }
    }
}
