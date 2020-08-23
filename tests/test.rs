use simple_rand::*;
use std::rc::Rc;
#[test]
fn test() {
    let mut r = make_rng!();
    let v: Vec<_> = (0..100).map(Rc::new).collect();

    eprintln!("r.shuffle(v.iter()) = {:#?}", r.shuffle(&mut v.iter()));
    eprintln!("v = {:#?}", v);
}
