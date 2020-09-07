#[macro_export]
macro_rules! make_rng {
    () => {{
        let mut r = Rand::new(Pcg64Mcg::from_rng(thread_rng()).unwrap());
        r
    }};
    ($rng:ident) => {{
        let mut r = Rand::new($rng::from_rng(thread_rng()).unwrap());
        r
    }};
}
