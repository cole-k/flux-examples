use rand::Rng;

pub fn toss() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..10) > 5
}

pub fn replicate<T:Copy>(n:usize, val:T) -> Vec<T> {
    vec![val; n]
}
