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

fn test_operation(model: fn(Parameters) -> Box<Model>, bits: usize, 
                 codec: fn(&mut Read, &mut Write, Box<Model>) -> redux::Result<(u64, u64)>,
                 input: &Vec<u8>, output: &mut Vec<u8>) -> f64 {
    let model = model(Parameters::new(8, bits, bits + 2).unwrap());
    let mut reader = Cursor::new(input);
    let before_time = time::precise_time_s();
    let (before_size, after_size) = codec(&mut reader, output, model).unwrap();
    let after_time = time::precise_time_s();
    assert_eq!(before_size, input.len() as u64);
    assert_eq!(after_size, output.len() as u64);
    after_time - before_time
}

fn test_file(model: fn(Parameters) -> Box<Model>, bits: usize, file: &Path) -> (u64, u64, f64, f64) {
    debug_println!("    File: {}", file.to_str().unwrap());

    let mut path = fs::File::open(file).unwrap();
    let mut decomp1 = Vec::<u8>::new();
    let mut decomp2 = Vec::<u8>::new();
    let mut comp = Vec::<u8>::new();

    let dlen = path.read_to_end(&mut decomp1).unwrap();
    assert_eq!(dlen, decomp1.len());
    let ctime = test_operation(model, bits, redux::compress, &decomp1, &mut comp);

    let clen = comp.len();
    let dtime = test_operation(model, bits, redux::decompress, &comp, &mut decomp2);
    assert_eq!(dlen, decomp2.len());

    assert_eq!(decomp1, decomp2);
    debug_println!("      OrigSize: {} B, CompSize: {} B, Ratio: {:.3}, EncTime: {:.3} s, DecTime: {:.3} s, EncSpeed: {:.2} MiB/s, DecSpeed: {:.2} MiB/s", dlen, clen, ratio!(dlen, clen), ctime, dtime, speed!(dlen, ctime), speed!(dlen, dtime));

    (dlen as u64, clen as u64, ctime, dtime)
}

fn test_model(corpus: &str, name: &str, model: fn(Parameters) -> Box<Model>, bits: usize) {
    debug_println!("  Corpus: {}, Model: {}, Bits: {}", corpus, name, bits);
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.push("resources");
    dir.push(corpus);

    let mut dlen = 0u64;
    let mut clen = 0u64;
    let mut ctime = 0f64;
    let mut dtime = 0f64;
    for file in fs::read_dir(dir).unwrap().map(|entry| entry.unwrap().path()) {
        let (d, c, ct, dt) = test_file(model, bits, &file);
        dlen += d;
        clen += c;
        ctime += ct;
        dtime += dt;
    }
    println!("  Corpus: {}, Model: {}, Bits: {}, AvgRatio: {:.3}, AvgEncSpeed: {:.2} MiB/s, AvgDecSpeed: {:.2} MiB/s", corpus, name, bits, ratio!(dlen, clen), speed!(dlen, ctime), speed!(dlen, dtime));
}

#[test]
fn test_artificial_linear14() {
    test_model("artificial", "Linear", AdaptiveLinearModel::new, 14usize);
}

#[test]
fn test_artificial_linear22() {
    test_model("artificial", "Linear", AdaptiveLinearModel::new, 22usize);
}

#[test]
fn test_artificial_linear30() {
    test_model("artificial", "Linear", AdaptiveLinearModel::new, 30usize);
}

#[test]
fn test_artificial_tree14() {
    test_model("artificial", "Tree", AdaptiveTreeModel::new, 14usize);
}

#[test]
fn test_artificial_tree22() {
    test_model("artificial", "Tree", AdaptiveTreeModel::new, 22usize);
}

#[test]
fn test_artificial_tree30() {
    test_model("artificial", "Tree", AdaptiveTreeModel::new, 30usize);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn test_calgary_linear14() {
    test_model("calgary", "Linear", AdaptiveLinearModel::new, 14usize);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn test_calgary_linear22() {
    test_model("calgary", "Linear", AdaptiveLinearModel::new, 22usize);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn test_calgary_linear30() {
    test_model("calgary", "Linear", AdaptiveLinearModel::new, 30usize);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn test_calgary_tree14() {
    test_model("calgary", "Tree", AdaptiveTreeModel::new, 14usize);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn test_calgary_tree22() {
    test_model("calgary", "Tree", AdaptiveTreeModel::new, 22usize);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn test_calgary_tree30() {
    test_model("calgary", "Tree", AdaptiveTreeModel::new, 30usize);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn test_canterbury_linear14() {
    test_model("canterbury", "Linear", AdaptiveLinearModel::new, 14usize);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn test_canterbury_linear22() {
    test_model("canterbury", "Linear", AdaptiveLinearModel::new, 22usize);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn test_canterbury_linear30() {
    test_model("canterbury", "Linear", AdaptiveLinearModel::new, 30usize);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn test_canterbury_tree14() {
    test_model("canterbury", "Tree", AdaptiveTreeModel::new, 14usize);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn test_canterbury_tree22() {
    test_model("canterbury", "Tree", AdaptiveTreeModel::new, 22usize);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn test_canterbury_tree30() {
    test_model("canterbury", "Tree", AdaptiveTreeModel::new, 30usize);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn test_large_linear14() {
    test_model("large", "Linear", AdaptiveLinearModel::new, 14usize);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn test_large_linear22() {
    test_model("large", "Linear", AdaptiveLinearModel::new, 22usize);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn test_large_linear30() {
    test_model("large", "Linear", AdaptiveLinearModel::new, 30usize);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn test_large_tree14() {
    test_model("large", "Tree", AdaptiveTreeModel::new, 14usize);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn test_large_tree22() {
    test_model("large", "Tree", AdaptiveTreeModel::new, 22usize);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn test_large_tree30() {
    test_model("large", "Tree", AdaptiveTreeModel::new, 30usize);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn test_misc_linear14() {
    test_model("misc", "Linear", AdaptiveLinearModel::new, 14usize);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn test_misc_linear22() {
    test_model("misc", "Linear", AdaptiveLinearModel::new, 22usize);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn test_misc_linear30() {
    test_model("misc", "Linear", AdaptiveLinearModel::new, 30usize);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn test_misc_tree14() {
    test_model("misc", "Tree", AdaptiveTreeModel::new, 14usize);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn test_misc_tree22() {
    test_model("misc", "Tree", AdaptiveTreeModel::new, 22usize);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn test_misc_tree30() {
    test_model("misc", "Tree", AdaptiveTreeModel::new, 30usize);
}
