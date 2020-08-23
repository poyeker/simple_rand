use simple_rand::*;
#[test]
fn test() {
    let mut r = make_rng!();
    eprintln!(
        "up_to_n_of([1,2,4],2) = {:#?}",
        r.shuffle([1, 2, 4].iter()).len()
    );
}
