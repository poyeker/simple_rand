use std::ops::{Deref, DerefMut};

pub mod macros;
use itertools::*;
pub use rand::prelude::*;
use rand_distr::*;
pub use rand_pcg::Pcg64Mcg;

pub struct Rand<R: Rng> {
    rng: R,
}

impl<R: Rng> Deref for Rand<R> {
    type Target = R;
    fn deref(&self) -> &Self::Target {
        &self.rng
    }
}

impl<R: Rng> DerefMut for Rand<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rng
    }
}

impl<R: Rng> Rand<R> {
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

    pub fn choice<T, I>(&mut self, iterable: I) -> T
    where
        I: Iterator<Item = T>,
    {
        iterable.choose(&mut self.rng).unwrap()
    }

    pub fn n_of<T, I>(&mut self, iterable: I, amount: usize) -> Vec<T>
    where
        I: Iterator<Item = T>,
    {
        iterable.choose_multiple(&mut self.rng, amount)
    }

    pub fn one_of_weighted<X, T, I, W>(&mut self, iterable: I, weights: W) -> T
    where
        I: Iterator<Item = T> + Clone,
        W: IntoIterator,
        W::Item: rand::distributions::uniform::SampleBorrow<X>,
        X: rand::distributions::uniform::SampleUniform
            + PartialOrd
            + for<'a> ::core::ops::AddAssign<&'a X>
            + Clone
            + Default,
    {
        let w = rand::distributions::WeightedIndex::new(weights).unwrap();
        iterable.clone().nth(self.rng.sample(w)).unwrap()
    }
    pub fn shuffle<T, I>(&mut self, iterable: I) -> Vec<T>
    where
        T: Clone,
        I: Iterator<Item = T> + Clone,
    {
        let n = iterable.clone().count();
        self.choice(iterable.permutations(n))
    }
}
