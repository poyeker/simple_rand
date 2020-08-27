use simple_rand::*;
#[test]
fn test() {
    let mut r = make_rng!();
    let v = r.n_of(0..10000,100);

    eprintln!("v = {:#?}", v);
}
