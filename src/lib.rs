use std::ops::{Deref, DerefMut};

use itertools::*;
pub use rand::prelude::*;
pub use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand_distr::*;
pub use rand_pcg::{Pcg32, Pcg64, Pcg64Mcg};
pub use rand_xorshift::XorShiftRng;
pub use rand_xoshiro::*;

pub struct Rand<R: SeedableRng + Rng> {
    rng: R,
}

impl<R: SeedableRng + Rng> Default for Rand<R> {
    fn default() -> Self {
        Rand {
            rng: R::from_rng(thread_rng()).unwrap(),
        }
    }
}

impl<R: SeedableRng + Rng> Deref for Rand<R> {
    type Target = R;
    fn deref(&self) -> &Self::Target {
        &self.rng
    }
}

impl<R: SeedableRng + Rng> DerefMut for Rand<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rng
    }
}

impl<R: SeedableRng + Rng> Rand<R> {
    pub fn new(rng: R) -> Self {
        Rand { rng }
    }

    pub fn rand_bool(&mut self, p: f64) -> bool {
        self.gen_bool(p)
    }

    pub fn rand_int(&mut self, low: i64, high: i64) -> i64 {
        self.gen_range(low, high)
    }

    pub fn rand_float(&mut self, low: f64, high: f64) -> f64 {
        self.gen_range(low, high)
    }

    pub fn rand_exp(&mut self, mean: f64) -> f64 {
        let distr = Exp::new(1. / mean).unwrap();
        self.sample(distr)
    }

    pub fn rand_normal(&mut self, mean: f64, std: f64) -> f64 {
        let distr = Normal::new(mean, std).unwrap();
        self.sample(distr)
    }

    pub fn rand_gamma(&mut self, shape: f64, scale: f64) -> f64 {
        let distr = Gamma::new(shape, scale).unwrap();
        self.sample(distr)
    }

    pub fn one_of<T>(&mut self, slice: &[T]) -> T
    where
        T: Clone,
    {
        slice.choose(&mut self.rng).unwrap().clone()
    }

    pub fn n_of<T>(&mut self, slice: &[T], amount: usize) -> Vec<T>
    where
        T: Clone,
    {
        slice
            .choose_multiple(&mut self.rng, amount)
            .cloned()
            .collect()
    }

    pub fn one_of_weighted<X, T, I, W>(&mut self, slice: &[T], weights: W) -> T
    where
        T: Clone,
        W: IntoIterator,
        W::Item: rand::distributions::uniform::SampleBorrow<X>,
        X: rand::distributions::uniform::SampleUniform
            + PartialOrd
            + for<'a> ::core::ops::AddAssign<&'a X>
            + Clone
            + Default,
    {
        let w = rand::distributions::WeightedIndex::new(weights).unwrap();
        slice[self.rng.sample(w)].clone()
    }

    pub fn one_of_weighted_by_key<T, K, F>(&mut self, slice: &[T], key: F) -> T
    where
        T: Clone,
        K: Ord
            + Clone
            + Default
            + rand_distr::uniform::SampleUniform
            + for<'a> std::ops::AddAssign<&'a K>,
        F: FnMut(&T) -> K,
    {
        let weights = slice.iter().map(key);
        let w = rand::distributions::WeightedIndex::new(weights).unwrap();
        slice[self.rng.sample(w)].clone()
    }

    pub fn n_of_weighted_by_key<T, K, F>(&mut self, slice: &[T], amount: usize, key: F) -> Vec<T>
    where
        T: Clone,
        K: Ord
            + Clone
            + Default
            + rand_distr::uniform::SampleUniform
            + for<'a> std::ops::AddAssign<&'a K>,
        F: FnMut(&T) -> K,
    {
        let weights = slice.iter().map(key);
        let w = rand::distributions::WeightedIndex::new(weights).unwrap();
        (0..amount)
            .map(|_| slice[self.sample(&w)].clone())
            .collect()
    }

    pub fn n_of_weighted<X, T, W>(&mut self, slice: &[T], weights: W, amount: usize) -> Vec<T>
    where
        T: Clone,
        W: IntoIterator,
        W::Item: rand::distributions::uniform::SampleBorrow<X>,
        X: rand::distributions::uniform::SampleUniform
            + PartialOrd
            + for<'a> ::core::ops::AddAssign<&'a X>
            + Clone
            + Default,
    {
        let w = rand::distributions::WeightedIndex::new(weights).unwrap();
        (0..amount)
            .map(|_| slice[self.sample(&w)].clone())
            .collect()
    }

    pub fn shuffle<T>(&mut self, slice: &[T]) -> Vec<T>
    where
        T: Clone,
    {
        let mut idx: Vec<_> = (0..slice.len()).collect();
        idx.shuffle(&mut self.rng);
        let sorted_tuples = idx.iter().zip(slice.iter()).sorted_by_key(|x| *x.0);
        sorted_tuples.map(|x| x.1.clone()).collect()
    }
}
