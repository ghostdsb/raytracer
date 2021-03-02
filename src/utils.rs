use rand::Rng;
use crate::constants;

pub fn random_number() -> f32{
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_number_ranged(l: f32, r: f32) -> f32{
    let mut rng = rand::thread_rng();
    rng.gen_range(l..r)
}

pub fn deg_to_radian(deg: f32) -> f32{
    deg*constants::PI / 180.0
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x<min {
        return min
    }
    if x>max {
        return max
    }

    x
}