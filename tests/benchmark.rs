extern crate redux;
extern crate time;

use std::env;
use std::io;
use std::fs;
use std::path::Path;

fn single_run(f: &Fn() -> redux::Result<(u64, u64)>) -> (u64, u64, time::Duration) {
    let before = time::precise_time_ns() as i64;
    let (a, b) = f().unwrap();
    let after = time::precise_time_ns() as i64;
    (a, b, time::Duration::nanoseconds(after - before))
}

fn single_codec(file: &Path, freq: &usize) {
    let (raw_len, comp_len1, comp_time) = single_run(&|| {
        let mut infile = fs::File::open(file).unwrap();
        let mut outfile = fs::File::create("compressed.redux").unwrap();
        redux::compress(&mut infile, &mut outfile) 
    });
    let (comp_len2, decomp_len, decomp_time) = single_run(&|| {
        let mut i = fs::File::open("compressed.redux").unwrap();
        let mut o = fs::File::create("decompressed.redux").unwrap();
        redux::decompress(&mut i, &mut o)
    });
    assert_eq!(raw_len, decomp_len);
    assert_eq!(comp_len1, comp_len2);
    let ratio = (raw_len as f64) / (comp_len1 as f64);
    
    println!(" Compressed {} bytes into {} bytes (ratio: {}), took: {} and {}", raw_len, comp_len1, ratio, comp_time, decomp_time);
}

fn single_file(file: &Path) {
    println!("File: {}", match file.to_str() { Some(s) => s, None => { panic!() }});
    for freq in [14us, 22, 30].iter() {
        single_codec(file, freq);
    }
}

fn recurse_files(p: &Path) {
    if fs::metadata(p).unwrap().is_dir() {
        for d in fs::read_dir(p).unwrap() {
            recurse_files(&d.unwrap().path());
        }
    } else {
        single_file(p);
    }
}

#[test]
fn benchmark_files() {
    if let Some(arg) = env::args().nth(1) {
        recurse_files(&Path::new(&arg));
    } else {
        panic!();
    }
}

