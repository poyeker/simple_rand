use simple_rand::*;
#[test]
fn test() {
    let mut r = make_rng!();

    eprintln!(
        "up_to_n_of([1,2,4],2) = {:#?}",
        r.up_to_n_of(&mut [1, 2, 4].iter(), 10)
    );
}
