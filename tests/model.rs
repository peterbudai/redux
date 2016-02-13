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

#[test]
fn compare_models_encode() {
    let mut linear = AdaptiveLinearModel::new(Parameters::new(8, 14, 16).unwrap());
    let mut tree = AdaptiveTreeModel::new(Parameters::new(8, 14, 16).unwrap());

    println!("Encode test");
    for count in 0..10000 {
        let lt = linear.total_frequency();
        let tt = tree.total_frequency();
        assert_eq!(lt, tt);
        let lf = linear.get_freq_table();
        let tf = tree.get_freq_table();
        // print_freq_tables(&lf, &tf);
        assert_eq!(lf, tf);
        let symbol = rand::random::<u8>() as usize;
        let (ll, lh) = linear.get_frequency(symbol).unwrap();
        let (tl, th) = tree.get_frequency(symbol).unwrap();
        println!("Iteration: {}, Symbol: {}, Linear: {}-{}, Tree: {}-{}", count, symbol, ll, lh, tl, th);
        assert_eq!(ll, tl);
        assert_eq!(lh, th);
    }
}

#[test]
fn compare_models_decode() {
    let mut linear = AdaptiveLinearModel::new(Parameters::new(8, 14, 16).unwrap());
    let mut tree = AdaptiveTreeModel::new(Parameters::new(8, 14, 16).unwrap());

    println!("Decode test");
    for count in 0..10000 {
        let lt = linear.total_frequency();
        let tt = tree.total_frequency();
        assert_eq!(lt, tt);
        let lf = linear.get_freq_table();
        let tf = tree.get_freq_table();
        // print_freq_tables(&lf, &tf);
        assert_eq!(lf, tf);
        let value = (rand::random::<u64>() % lt) as u64;
        let (ls, ll, lh) = linear.get_symbol(value).unwrap();
        let (ts, tl, th) = tree.get_symbol(value).unwrap();
        println!("Iteration: {}, Value: {}, Linear: {}, {}-{}, Tree: {}, {}-{}", count, value, ls, ll, lh, ts, tl, th);
        assert_eq!(ls, ts);
        assert_eq!(ll, tl);
        assert_eq!(lh, th);
    }
}

