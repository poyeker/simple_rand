#[macro_export]
macro_rules! make_rng {
    () => {{
        let mut r = Rand::new(Pcg64Mcg::from_entropy());
        r
    }};
    ($rng:ident) => {{
        let mut r = Rand::new($rng::from_entropy());
        r
    }};
}
