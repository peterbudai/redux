extern crate redux;
extern crate time;

use std::io::Write;
use std::io::Read;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use redux::model::Parameters;
use redux::model::Model;
use redux::model::AdaptiveLinearModel;

fn benchmark_func(codec: &Fn(&mut Read, &mut Write, Box<Model>) -> redux::Result<(u64, u64)>, 
                  ifile: &Path, ofile: &Path, model: &Fn(Parameters) -> Box<Model>, freq: &usize) -> (u64, u64, f64) {
    let mut i = fs::File::open(ifile).unwrap();
    let mut o = fs::File::create(ofile).unwrap();
    let m = model(Parameters::new(8, *freq, *freq + 2).unwrap());

    let before = time::precise_time_s();
    let (a, b) = codec(&mut i, &mut o, m).unwrap();
    let after = time::precise_time_s();
    (a, b, after - before)
}

fn benchmark_codec(file: &Path, tmpc: &Path, tmpd: &Path, model: &Fn(Parameters) -> Box<Model>, freq: &usize) {
    let (raw_len, comp_len1, comp_time) = benchmark_func(&redux::compress_custom, file, tmpc, model, freq);
    let (comp_len2, decomp_len, decomp_time) = benchmark_func(&redux::decompress_custom, tmpc, tmpd, model, freq);
    assert_eq!(raw_len, decomp_len);
    assert_eq!(comp_len1, comp_len2);
    let ratio = (raw_len as f64) / (comp_len1 as f64);

    println!("  OrigSize: {} B, CompSize: {} B, Ratio: {:.3}, EncTime: {:.3} s, DecTime: {:.3} s", raw_len, comp_len1, ratio, comp_time, decomp_time);
}

fn benchmark_path(p: &Path, tmpc: &Path, tmpd: &Path) {
    if fs::metadata(p).unwrap().is_dir() {
        println!("Directory: {}", match p.to_str() { Some(s) => s, None => { panic!() }});
        for d in fs::read_dir(p).unwrap() {
            benchmark_path(&d.unwrap().path(), tmpc, tmpd);
        }
    } else {
        println!(" File: {}", match p.to_str() { Some(s) => s, None => { panic!() }});
        for freq in [14usize, 22, 30].iter() {
            benchmark_codec(p, tmpc, tmpd, &|p: Parameters| AdaptiveLinearModel::new(p), freq);
        }
    }
}

#[test]
fn run_benchmark() {
    let mut indir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    indir.push("resources");
    let cfile = Path::new("compressed.redux");
    let dfile = Path::new("decompressed.redux");

    benchmark_path(indir.as_path(), cfile, dfile);
}

