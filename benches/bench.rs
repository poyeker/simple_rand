#![feature(test)]

extern crate test;

use rand_pcg::Mcg128Xsl64;
use simple_rand::*;
use test::Bencher;

#[bench]
fn bench1(b: &mut Bencher) {
    let mut rng = make_rng!(Pcg64Mcg);
    b.iter(|| rng.n_of(&(0..10000).collect::<Vec<i32>>(), 10));
}
