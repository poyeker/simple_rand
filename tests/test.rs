use simple_rand::*;

#[test]
fn test() {
    let mut r = make_rng!();

    println!("{}", r.rand_normal(10., 1.));
}
