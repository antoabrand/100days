use rand::Rng;

pub fn generate_random_num(min_threshold: u32, max_threshold: u32) -> u32 {
    return rand::thread_rng().gen_range(min_threshold..=max_threshold);
}
