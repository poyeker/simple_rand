use rand::distributions::uniform::*;
use simple_rand::*;
#[test]
fn test() {
    let mut r = make_rng!();

    println!("{}", r.n_of_weighted(&mut [0, 1, 2].iter(), &[1, 1, 2]));
}
