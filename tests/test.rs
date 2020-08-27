use simple_rand::*;
#[test]
fn test() {
    let mut r = make_rng!();
    let v = ["1", "2", "3", "4", "5"]
        .iter()
        .map(|&x| String::from(x))
        .collect::<Vec<_>>();
    let v1 = r.shuffle(&v);
    eprintln!("v = {:#?}", v);
    eprintln!("v1 = {:#?}", v1);
}
