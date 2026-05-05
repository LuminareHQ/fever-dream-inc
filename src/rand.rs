use std::cell::Cell;
use std::ops::Range;

thread_local! {
    static RNG_STATE: Cell<u64> = const { Cell::new(0) };
}

fn seed() -> u64 {
    let mut buf = [0u8; 8];
    if getrandom::fill(&mut buf).is_err() {
        return 0x9E3779B97F4A7C15;
    }
    u64::from_le_bytes(buf)
}

fn next_u64() -> u64 {
    RNG_STATE.with(|s| {
        let mut x = s.get();
        if x == 0 {
            x = seed();
            if x == 0 {
                x = 0x9E3779B97F4A7C15;
            }
        }
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        s.set(x);
        x
    })
}

pub trait RandomRange {
    fn random_range(range: Range<Self>) -> Self
    where
        Self: Sized;
}

impl RandomRange for f32 {
    fn random_range(range: Range<f32>) -> f32 {
        let unit = (next_u64() >> 40) as f32 / (1u64 << 24) as f32;
        range.start + unit * (range.end - range.start)
    }
}

impl RandomRange for f64 {
    fn random_range(range: Range<f64>) -> f64 {
        let unit = (next_u64() >> 11) as f64 / (1u64 << 53) as f64;
        range.start + unit * (range.end - range.start)
    }
}

impl RandomRange for usize {
    fn random_range(range: Range<usize>) -> usize {
        let span = range.end - range.start;
        range.start + (next_u64() as usize) % span
    }
}

impl RandomRange for u32 {
    fn random_range(range: Range<u32>) -> u32 {
        let span = range.end - range.start;
        range.start + (next_u64() as u32) % span
    }
}

impl RandomRange for i32 {
    fn random_range(range: Range<i32>) -> i32 {
        let span = (range.end - range.start) as u32;
        range.start + ((next_u64() as u32) % span) as i32
    }
}

pub fn random_range<T: RandomRange>(range: Range<T>) -> T {
    T::random_range(range)
}
