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

fn assert_freq_total(linear: &Model, tree: &Model) {
    let lt = linear.total_frequency();
    let tt = tree.total_frequency();
    assert_eq!(lt, tt);
}

#[cfg(debug_assertions)]
fn assert_freq_tables(linear: &Model, tree: &Model) {
    assert_freq_total(linear, tree);
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
fn assert_freq_tables(linear: &Model, tree: &Model) {
    assert_freq_total(linear, tree);
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

#[test]
fn compare_models_encode() {
    let mut linear = AdaptiveLinearModel::new(Parameters::new(8, 14, 16).unwrap());
    let mut tree = AdaptiveTreeModel::new(Parameters::new(8, 14, 16).unwrap());

    debug_println!("  Encode test");
    for count in 0..10000 {
        assert_freq_tables(&*linear, &*tree);
        let symbol = get_valid_symbol(8); 
        let (ll, lh) = linear.get_frequency(symbol).unwrap();
        let (tl, th) = tree.get_frequency(symbol).unwrap();
        debug_println!("    Iteration: {}, Symbol: {}, Linear: {}-{}, Tree: {}-{}", count, symbol, ll, lh, tl, th);
        assert_eq!(ll, tl);
        assert_eq!(lh, th);
    }

    let symbol = get_invalid_symbol(8);
    assert!(!linear.get_frequency(symbol).is_ok());
    assert!(!linear.get_frequency(symbol + 1).is_ok());
    assert!(!tree.get_frequency(symbol).is_ok());
    assert!(!tree.get_frequency(symbol).is_ok());
}

#[test]
fn compare_models_decode() {
    let mut linear = AdaptiveLinearModel::new(Parameters::new(8, 14, 16).unwrap());
    let mut tree = AdaptiveTreeModel::new(Parameters::new(8, 14, 16).unwrap());

    debug_println!("  Decode test");
    for count in 0..10000 {
        assert_freq_tables(&*linear, &*tree);
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

