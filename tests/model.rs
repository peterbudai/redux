extern crate redux;
extern crate rand;

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

fn get_invalid_symbol(bits: usize) -> usize {
    (1usize << bits) + 1usize
}

fn get_valid_symbol(bits: usize) -> usize {
    rand::random::<usize>() % get_invalid_symbol(bits)
}

fn get_invalid_value(m: &Model) -> u64 {
    m.total_frequency()
}

fn get_valid_value(m: &Model) -> u64 {
    rand::random::<u64>() % get_invalid_value(m)
}

fn compare_freq_total(linear: &Model, tree: &Model) {
    let lt = linear.total_frequency();
    let tt = tree.total_frequency();
    assert_eq!(lt, tt);
}

#[cfg(debug_assertions)]
fn compare_freq_tables(linear: &Model, tree: &Model) {
    compare_freq_total(linear, tree);
    let lf = linear.get_freq_table();
    let tf = tree.get_freq_table();
    assert_eq!(lf, tf);
}

#[cfg(not(debug_assertions))]
fn compare_freq_tables(linear: &Model, tree: &Model) {
    compare_freq_total(linear, tree);
}

fn compare_models_encode_single(bits: usize, freq: usize, code: usize, iter: usize) {
    let mut linear = AdaptiveLinearModel::new(Parameters::new(bits, freq, code).unwrap());
    let mut tree = AdaptiveTreeModel::new(Parameters::new(bits, freq, code).unwrap());

    println!("  Operation: Encode, Symbol: {} bits, Freq: {} bits, Code: {} bits, Iterations: {}", bits, freq, code, iter);
    for count in 0..iter {
        compare_freq_tables(&*linear, &*tree);
        let symbol = get_valid_symbol(bits); 
        let (ll, lh) = linear.get_frequency(symbol).unwrap();
        let (tl, th) = tree.get_frequency(symbol).unwrap();
        debug_println!("    Iteration: {}, Symbol: {}, Linear: {}-{}, Tree: {}-{}", count, symbol, ll, lh, tl, th);
        assert_eq!(ll, tl);
        assert_eq!(lh, th);
    }

    let symbol = get_invalid_symbol(bits);
    assert!(!linear.get_frequency(symbol).is_ok());
    assert!(!linear.get_frequency(symbol + 1).is_ok());
    assert!(!tree.get_frequency(symbol).is_ok());
    assert!(!tree.get_frequency(symbol).is_ok());
}

fn compare_models_decode_single(bits: usize, freq: usize, code: usize, iter: usize) {
    let mut linear = AdaptiveLinearModel::new(Parameters::new(bits, freq, code).unwrap());
    let mut tree = AdaptiveTreeModel::new(Parameters::new(bits, freq, code).unwrap());

    println!("  Operation: Decode, Symbol: {} bits, Freq: {} bits, Code: {} bits, Iterations: {}", bits, freq, code, iter);
    for count in 0..iter {
        compare_freq_tables(&*linear, &*tree);
        let value = get_valid_value(&*linear);
        let (ls, ll, lh) = linear.get_symbol(value).unwrap();
        let (ts, tl, th) = tree.get_symbol(value).unwrap();
        debug_println!("    Iteration: {}, Value: {}, Linear: {}, {}-{}, Tree: {}, {}-{}", count, value, ls, ll, lh, ts, tl, th);
        assert_eq!(ls, ts);
        assert_eq!(ll, tl);
        assert_eq!(lh, th);
    }

    let value = get_invalid_value(&*linear);
    assert!(!linear.get_symbol(value).is_ok());
    assert!(!linear.get_symbol(value + 1).is_ok());
    assert!(!tree.get_symbol(value).is_ok());
    assert!(!tree.get_symbol(value + 1).is_ok());
}

#[test]
fn compare_models_encode_4_10_16() {
    compare_models_encode_single(4, 10, 16, 10000);
}

#[test]
fn compare_models_encode_4_14_16() {
    compare_models_encode_single(4, 14, 16, 10000);
}

#[test]
fn compare_models_encode_4_22_24() {
    compare_models_encode_single(4, 22, 24, 100000);
}

#[test]
fn compare_models_encode_4_24_30() {
    compare_models_encode_single(4, 24, 30, 100000);
}

#[test]
fn compare_models_encode_4_30_32() {
    compare_models_encode_single(4, 30, 32, 200000);
}

#[test]
#[ignore]
fn compare_models_encode_8_10_16() {
    compare_models_encode_single(8, 10, 16, 10000);
}

#[test]
#[ignore]
fn compare_models_encode_8_14_16() {
    compare_models_encode_single(8, 14, 16, 10000);
}

#[test]
#[ignore]
fn compare_models_encode_8_22_24() {
    compare_models_encode_single(8, 22, 24, 100000);
}

#[test]
#[ignore]
fn compare_models_encode_8_24_30() {
    compare_models_encode_single(8, 24, 30, 100000);
}

#[test]
#[ignore]
fn compare_models_encode_8_30_32() {
    compare_models_encode_single(8, 30, 32, 200000);
}

#[test]
#[ignore]
fn compare_models_encode_12_14_16() {
    compare_models_encode_single(12, 14, 16, 10000);
}

#[test]
#[ignore]
fn compare_models_encode_12_22_24() {
    compare_models_encode_single(12, 22, 24, 100000);
}

#[test]
#[ignore]
fn compare_models_encode_12_24_30() {
    compare_models_encode_single(12, 24, 30, 100000);
}

#[test]
#[ignore]
fn compare_models_encode_12_30_32() {
    compare_models_encode_single(12, 30, 32, 200000);
}

#[test]
fn compare_models_decode_4_10_16() {
    compare_models_decode_single(4, 10, 16, 10000);
}

#[test]
fn compare_models_decode_4_14_16() {
    compare_models_decode_single(4, 14, 16, 10000);
}

#[test]
fn compare_models_decode_4_22_24() {
    compare_models_decode_single(4, 22, 24, 100000);
}

#[test]
fn compare_models_decode_4_24_30() {
    compare_models_decode_single(4, 24, 30, 100000);
}

#[test]
fn compare_models_decode_4_30_32() {
    compare_models_decode_single(4, 30, 32, 200000);
}

#[test]
#[ignore]
fn compare_models_decode_8_10_16() {
    compare_models_decode_single(8, 10, 16, 10000);
}

#[test]
#[ignore]
fn compare_models_decode_8_14_16() {
    compare_models_decode_single(8, 14, 16, 10000);
}

#[test]
#[ignore]
fn compare_models_decode_8_22_24() {
    compare_models_decode_single(8, 22, 24, 100000);
}

#[test]
#[ignore]
fn compare_models_decode_8_24_30() {
    compare_models_decode_single(8, 24, 30, 100000);
}

#[test]
#[ignore]
fn compare_models_decode_8_30_32() {
    compare_models_decode_single(8, 30, 32, 200000);
}

#[test]
#[ignore]
fn compare_models_decode_12_14_16() {
    compare_models_decode_single(12, 14, 16, 10000);
}

#[test]
#[ignore]
fn compare_models_decode_12_22_24() {
    compare_models_decode_single(12, 22, 24, 100000);
}

#[test]
#[ignore]
fn compare_models_decode_12_24_30() {
    compare_models_decode_single(12, 24, 30, 100000);
}

#[test]
#[ignore]
fn compare_models_decode_12_30_32() {
    compare_models_decode_single(12, 30, 32, 200000);
}
