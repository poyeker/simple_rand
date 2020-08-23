use simple_rand::*;
#[test]
fn test() {
    let mut r = make_rng!();
    let v = r.one_of_weighted_by_key(0..10, |x| if x == 3 { 1 } else { 0 });

    eprintln!("v = {:#?}", v);
}
