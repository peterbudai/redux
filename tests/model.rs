extern crate redux;
extern crate rand;

use redux::model::Parameters;
use redux::model::Model;
use redux::model::AdaptiveLinearModel;
use redux::model::AdaptiveTreeModel;

#[test]
fn compare_models_encode() {
    let mut linear = AdaptiveLinearModel::new(Parameters::new(8, 14, 16).unwrap());
    let mut tree = AdaptiveTreeModel::new(Parameters::new(8, 14, 16).unwrap());

    println!("Encode test");
    for count in 0..100 {
        let symbol = rand::random::<u8>() as usize;
        let (ll, lh) = linear.get_frequency(symbol).unwrap();
        let (tl, th) = tree.get_frequency(symbol).unwrap();
        assert_eq!(ll, tl);
        assert_eq!(lh, th);
        println!("Iteration: {}, Symbol: {}, Linear: {}-{}, Tree: {}-{}", count, symbol, ll, lh, tl, th);
    }
}

#[test]
fn compare_models_decode() {
    let mut linear = AdaptiveLinearModel::new(Parameters::new(8, 14, 16).unwrap());
    let mut tree = AdaptiveTreeModel::new(Parameters::new(8, 14, 16).unwrap());

    println!("Decode test");
    for count in 0..100 {
        let value = (rand::random::<u16>() % (1u16 << 14)) as u64;
        let (ls, ll, lh) = linear.get_symbol(value).unwrap();
        let (ts, tl, th) = tree.get_symbol(value).unwrap();
        assert_eq!(ls, ts);
        assert_eq!(ll, tl);
        assert_eq!(lh, th);
        println!("Iteration: {}, Value: {}, Linear: {}, {}-{}, Tree: {}, {}-{}", count, value, ls, ll, lh, ts, tl, th);
    }
}

