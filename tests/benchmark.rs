extern crate redux;
extern crate time;

use std::env;
use std::io;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use redux::model::Parameters;
use redux::model::adaptive_linear::AdaptiveLinearModel;

fn single_run(f: &Fn() -> redux::Result<(u64, u64)>) -> (u64, u64, f64) {
    let before = time::precise_time_s();
    let (a, b) = f().unwrap();
    let after = time::precise_time_s();
    (a, b, after - before)
}

fn single_codec(file: &Path, freq: &usize) {
    let (raw_len, comp_len1, comp_time) = single_run(&|| {
        let mut i = fs::File::open(file).unwrap();
        let mut o = fs::File::create("compressed.redux").unwrap();
        let p = Parameters::init(8, *freq, *freq + 2).unwrap();
        let m = AdaptiveLinearModel::init(p);
        return redux::compress_custom(&mut i, &mut o, m);
    });
    let (comp_len2, decomp_len, decomp_time) = single_run(&|| {
        let mut i = fs::File::open("compressed.redux").unwrap();
        let mut o = fs::File::create("decompressed.redux").unwrap();
        let p = Parameters::init(8, *freq, *freq + 2).unwrap();
        let m = AdaptiveLinearModel::init(p);
        return redux::decompress_custom(&mut i, &mut o, m);
    });
    assert_eq!(raw_len, decomp_len);
    assert_eq!(comp_len1, comp_len2);
    let ratio = (raw_len as f64) / (comp_len1 as f64);
    
    println!("  Original: {} B, Compressed: {} B, Ratio: {:.3}, Compression: {:.3} s, Decompression: {:.3} s", raw_len, comp_len1, ratio, comp_time, decomp_time);
}

fn single_file(file: &Path) {
    println!(" File: {}", match file.to_str() { Some(s) => s, None => { panic!() }});
    for freq in [14usize, 22, 30].iter() {
        single_codec(file, freq);
    }
}

fn recurse_files(p: &Path) {
    if fs::metadata(p).unwrap().is_dir() {
        println!("Directory: {}", match p.to_str() { Some(s) => s, None => { panic!() }});
        for d in fs::read_dir(p).unwrap() {
            recurse_files(&d.unwrap().path());
        }
    } else {
        single_file(p);
    }
}

#[test]
fn benchmark_files() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources");

    recurse_files(d.as_path());
}

