extern crate redux;
extern crate time;

use std::io::Write;
use std::io::Read;
use std::io::Cursor;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use redux::model::Parameters;
use redux::model::Model;
use redux::model::AdaptiveLinearModel;
use redux::model::AdaptiveTreeModel;

macro_rules! debug_println {
    ($($arg:tt)*) => (
        if cfg!(debug_assertions) {
            println!( $($arg)* )
        }
    )
}

macro_rules! ratio {
    ($d:expr, $c:expr) => ($d as f64 / $c as f64)
}

macro_rules! speed {
    ($d:expr, $t:expr) => ($d as f64 / $t / 1024f64 / 1024f64)
}

struct ModelInfo {
    name: &'static str,
    model: fn(Parameters) -> Box<Model>,
    bits: usize,
    decompressed: u64,
    compressed: u64,
    compress_time: f64,
    decompress_time: f64
}

fn add_model(models: &mut Vec<ModelInfo>, name: &'static str, model: fn(Parameters) -> Box<Model>) {
    for bits in [14usize, 22, 30].iter() {
       models.push(ModelInfo { 
           name: name, 
           model: model,
           bits: *bits,
           decompressed: 0u64,
           compressed: 0u64,
           compress_time: 0f64,
           decompress_time: 0f64
       });
    }
}

fn collect_models() -> Vec<ModelInfo> {
    let mut models = Vec::<ModelInfo>::new();
    add_model(&mut models, "Linear", AdaptiveLinearModel::new);
    add_model(&mut models, "Tree", AdaptiveTreeModel::new);
    models
}

fn collect_files() -> Vec<PathBuf> {
    let mut dirs = vec![PathBuf::from(env!("CARGO_MANIFEST_DIR"))];
    dirs[0].push("resources");
    let mut files = Vec::<PathBuf>::new();
    while !dirs.is_empty() {
        for entry in fs::read_dir(dirs.pop().unwrap()).unwrap() {
            let path = entry.unwrap().path();
            if fs::metadata(&path).unwrap().is_dir() { &mut dirs } else { &mut files }.push(path);
        }
    }
    files
}

fn benchmark_dir(info: &ModelInfo, 
                 codec: fn(&mut Read, &mut Write, Box<Model>) -> redux::Result<(u64, u64)>,
                 input: &Vec<u8>,
                 output: &mut Vec<u8>) -> f64 {
    let model = (info.model)(Parameters::new(8, info.bits, info.bits + 2).unwrap());
    let mut reader = Cursor::new(input);
    let before_time = time::precise_time_s();
    let (before_size, after_size) = codec(&mut reader, output, model).unwrap();
    let after_time = time::precise_time_s();
    assert_eq!(before_size, input.len() as u64);
    assert_eq!(after_size, output.len() as u64);
    after_time - before_time
}

fn benchmark_single(info: &mut ModelInfo, file: &Path) {
    debug_println!("    File: {}", file.to_str().unwrap());

    let mut path = fs::File::open(file).unwrap();
    let mut decomp1 = Vec::<u8>::new();
    let mut decomp2 = Vec::<u8>::new();
    let mut comp = Vec::<u8>::new();

    let dlen = path.read_to_end(&mut decomp1).unwrap();
    assert_eq!(dlen, decomp1.len());
    let ctime = benchmark_dir(info, redux::compress_custom, &decomp1, &mut comp);

    let clen = comp.len();
    let dtime = benchmark_dir(info, redux::decompress_custom, &comp, &mut decomp2);
    assert_eq!(dlen, decomp2.len());

    assert_eq!(decomp1, decomp2);
    debug_println!("      OrigSize: {} B, CompSize: {} B, Ratio: {:.3}, EncTime: {:.3} s, DecTime: {:.3} s, EncSpeed: {:.2} MiB/s, DecSpeed: {:.2} MiB/s", dlen, clen, ratio!(dlen, clen), ctime, dtime, speed!(dlen, ctime), speed!(dlen, dtime));

    info.decompressed += dlen as u64;
    info.compressed += clen as u64;
    info.compress_time += ctime;
    info.decompress_time += dtime;
}

#[test]
fn benchmark_all() {
    let files = collect_files();
    let mut models = collect_models();

    for model in &mut models {
        debug_println!("  Model: {}, Bits: {}", model.name, model.bits);
        for file in &files {
            benchmark_single(model, file);
        }
        println!("  Model: {}, Bits: {}, AvgRatio: {:.3}, AvgEncSpeed: {:.2} MiB/s, AvgDecSpeed: {:.2} MiB/s", model.name, model.bits, ratio!(model.decompressed, model.compressed), speed!(model.decompressed, model.compress_time), speed!(model.decompressed, model.decompress_time));
    }
}
