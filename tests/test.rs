use simple_rand::*;
#[test]
fn test() {
    let mut r = make_rng!();
    let v = r.n_of_weighted(0..10, 0..10, 3);
    eprintln!("v = {:#?}", v);
}
