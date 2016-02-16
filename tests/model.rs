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

struct Config(usize, usize, usize, usize);

const CONFIGS: [Config; 14] = [
    Config(4, 10, 16, 10000),
    Config(4, 14, 16, 10000),
    Config(4, 22, 24, 100000),
    Config(4, 24, 30, 100000),
    Config(4, 30, 32, 200000),
    Config(8, 10, 16, 10000),
    Config(8, 14, 16, 50000),
    Config(8, 22, 24, 100000),
    Config(8, 24, 30, 100000),
    Config(8, 30, 32, 200000),
    Config(12, 14, 16, 10000),
    Config(12, 22, 24, 100000),
    Config(12, 24, 30, 200000),
    Config(12, 30, 32, 400000),
];

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
    for i in 0..lf.len() {
        let (ll, lh) = lf[i];
        let (tl, th) = tf[i];
        println!("      Symbol: {} -> Linear: {}-{}, Tree: {}-{}, Match: {}", i, ll, lh, tl, th, if ll == tl && lh == th { "Yes" } else { "No" });
    }
    assert_eq!(lf, tf);
}

#[cfg(not(debug_assertions))]
fn compare_freq_tables(linear: &Model, tree: &Model) {
    compare_freq_total(linear, tree);
}

fn compare_models_encode_single(config: &Config) {
    let mut linear = AdaptiveLinearModel::new(Parameters::new(config.0, config.1, config.2).unwrap());
    let mut tree = AdaptiveTreeModel::new(Parameters::new(config.0, config.1, config.2).unwrap());

    println!("  Operation: Encode, Symbol: {} bits, Freq: {} bits, Code: {} bits, Iterations: {}", config.0, config.1, config.2, config.3);
    for count in 0..config.3 {
        compare_freq_tables(&*linear, &*tree);
        let symbol = get_valid_symbol(config.0); 
        let (ll, lh) = linear.get_frequency(symbol).unwrap();
        let (tl, th) = tree.get_frequency(symbol).unwrap();
        debug_println!("    Iteration: {}, Symbol: {}, Linear: {}-{}, Tree: {}-{}", count, symbol, ll, lh, tl, th);
        assert_eq!(ll, tl);
        assert_eq!(lh, th);
    }

    let symbol = get_invalid_symbol(config.0);
    assert!(!linear.get_frequency(symbol).is_ok());
    assert!(!linear.get_frequency(symbol + 1).is_ok());
    assert!(!tree.get_frequency(symbol).is_ok());
    assert!(!tree.get_frequency(symbol).is_ok());
}

fn compare_models_decode_single(config: &Config) {
    let mut linear = AdaptiveLinearModel::new(Parameters::new(config.0, config.1, config.2).unwrap());
    let mut tree = AdaptiveTreeModel::new(Parameters::new(config.0, config.1, config.2).unwrap());

    println!("  Operation: Decode, Symbol: {} bits, Freq: {} bits, Code: {} bits, Iterations: {}", config.0, config.1, config.2, config.3);
    for count in 0..config.3 {
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
fn compare_models_encode_all() {
    for config in CONFIGS.iter() {
        compare_models_encode_single(&*config);
    }
}

#[test]
fn compare_models_decode_all() {
    for config in CONFIGS.iter() {
        compare_models_decode_single(&*config);
    }
}
