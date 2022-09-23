use num::Float as SubFloat;
use rand::Rng;

pub trait Float: SubFloat + Sized {
    fn random() -> Self;
    fn random_range(low: Self, high: Self) -> Self;
    fn from_i32(i: i32) -> Self;
}

impl Float for f32 {
    fn random() -> f32 {
        let mut rng = rand::thread_rng();
        rng.gen()
    }
    fn random_range(low: Self, high: Self) -> f32 {
        Self::random() * (high - low) + low
    }
    fn from_i32(i: i32) -> f32 {
        i as f32
    }
}
impl Float for f64 {
    fn random() -> f64 {
        let mut rng = rand::thread_rng();
        rng.gen()
    }
    fn random_range(low: Self, high: Self) -> f64 {
        Self::random() * (high - low) + low
    }
    fn from_i32(i: i32) -> f64 {
        i as f64
    }
}

#[cfg(test)]
mod tests {
    use crate::Float;

    #[test]
    fn f32_random() {
        let i: f32 = Float::random();
        assert!(i.is_finite());
    }

    #[test]
    fn f64_random() {
        let i: f64 = Float::random();
        assert!(i.is_finite());
    }

    #[test]
    fn f32_random_range() {
        let i: f32 = Float::random_range(-1f32, 1f32);
        assert!(i > -1f32 && i < 1f32);
    }

    #[test]
    fn f64_random_range() {
        let i: f64 = Float::random_range(-1f64, 1f64);
        assert!(i > -1f64 && i < 1f64);
    }

    #[test]
    fn f64_from_i32() {
        let i: f64 = Float::from_i32(-2);
        assert!(i == -2f64);
    }
}
