extern crate redux;
extern crate rand;

use redux::model::Parameters;
use redux::model::Model;
use redux::model::AdaptiveLinearModel;
use redux::model::AdaptiveTreeModel;

fn print_freq_tables(linear: &Vec<(u64, u64)>, tree: &Vec<(u64, u64)>) {
    for i in 0..linear.len() {
        let (ll, lh) = linear[i];
        let (tl, th) = tree[i];
        println!("  Symbol: {} -> Linear: {}-{}, Tree: {}-{}, Match: {}", i, ll, lh, tl, th, if ll == tl && lh == th { "Yes" } else { "No" });
    }
}

fn get_invalid_symbol(bits: usize) -> usize {
    (1usize << bits) + 1usize
}

fn get_valid_symbol(bits: usize) -> usize {
    rand::random::<usize>() % get_invalid_symbol(bits)
}

#[test]
fn compare_models_encode() {
    let mut linear = AdaptiveLinearModel::new(Parameters::new(4, 14, 16).unwrap());
    let mut tree = AdaptiveTreeModel::new(Parameters::new(4, 14, 16).unwrap());

    println!("Encode test");
    for count in 0..10000 {
        let lt = linear.total_frequency();
        let tt = tree.total_frequency();
        assert_eq!(lt, tt);
        let lf = linear.get_freq_table();
        let tf = tree.get_freq_table();
        print_freq_tables(&lf, &tf);
        assert_eq!(lf, tf);
        let symbol = get_valid_symbol(4); 
        let (ll, lh) = linear.get_frequency(symbol).unwrap();
        let (tl, th) = tree.get_frequency(symbol).unwrap();
        println!("Iteration: {}, Symbol: {}, Linear: {}-{}, Tree: {}-{}", count, symbol, ll, lh, tl, th);
        assert_eq!(ll, tl);
        assert_eq!(lh, th);
    }

    assert!(!linear.get_frequency(get_invalid_symbol(4)).is_ok());
    assert!(!linear.get_frequency(get_invalid_symbol(4) + 1).is_ok());
    assert!(!tree.get_frequency(get_invalid_symbol(4)).is_ok());
    assert!(!tree.get_frequency(get_invalid_symbol(4) + 1).is_ok());
}

#[test]
fn compare_models_decode() {
    let mut linear = AdaptiveLinearModel::new(Parameters::new(4, 14, 16).unwrap());
    let mut tree = AdaptiveTreeModel::new(Parameters::new(4, 14, 16).unwrap());

    println!("Decode test");
    for count in 0..10000 {
        let lt = linear.total_frequency();
        let tt = tree.total_frequency();
        assert_eq!(lt, tt);
        let lf = linear.get_freq_table();
        let tf = tree.get_freq_table();
        print_freq_tables(&lf, &tf);
        assert_eq!(lf, tf);
        let value = rand::random::<u64>() % lt;
        let (ls, ll, lh) = linear.get_symbol(value).unwrap();
        let (ts, tl, th) = tree.get_symbol(value).unwrap();
        println!("Iteration: {}, Value: {}, Linear: {}, {}-{}, Tree: {}, {}-{}", count, value, ls, ll, lh, ts, tl, th);
        assert_eq!(ls, ts);
        assert_eq!(ll, tl);
        assert_eq!(lh, th);
    }

    let lf = linear.total_frequency();
    assert!(!linear.get_symbol(lf).is_ok());
    assert!(!linear.get_symbol(lf + 1).is_ok());

    let tf = tree.total_frequency();
    assert!(!tree.get_symbol(tf).is_ok());
    assert!(!tree.get_symbol(tf + 1).is_ok());
}

