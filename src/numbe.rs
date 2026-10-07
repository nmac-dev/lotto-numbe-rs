//! Generate a random number between a given range (inclusive).

use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};
use log::{warn, error};

/// A fast, lightweight 64-bit Pseudo-Random Number Generator (PRNG).
///
/// Uses George Marsaglia's XorShift algorithm (`[13, 7, 17]`) with a cycle period of $2^{64} - 1$.
pub struct Rng {
    seed: u64,
}

impl Rng {

    /// Fallback seed used when a seed of `0` is provided to prevent generator lockup.
    const FALLBACK_SEED: u64 = 0xDEADBEEF;

    /// Creates a new [`Rng`] instance seeded with current system time nanoseconds.
    ///
    /// - Returns: An initialized [`Rng`] instance.
    pub fn new() -> Self {
        Self { seed: Self::gen_seed() }
    }

    /// Creates a new [`Rng`] instance initialized with a specific seed.
    ///
    /// - `seed`: Initial 64-bit seed (sanitized to [`Self::FALLBACK_SEED`] if `0`).
    /// - Returns: An initialized [`Rng`] instance.
    pub fn from_seed(seed: u64) -> Self {
        Self { seed: Self::sanitise_seed(seed) }
    }

    /// Ensures a seed value is non-zero.
    ///
    /// - `seed`: Raw seed to inspect.
    /// - Returns: The input `seed` if `seed != 0`, otherwise [`Self::FALLBACK_SEED`].
    fn sanitise_seed(seed: u64) -> u64 {
        if seed != 0 { seed } else { Self::FALLBACK_SEED }
    }

    /// Generates a non-zero seed using the current system time.
    ///
    /// - Returns: Non-zero nanosecond timestamp relative to [`UNIX_EPOCH`].
    fn gen_seed() -> u64 {
        let nano_ts: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;

        Self::sanitise_seed(nano_ts)
    }

    /// Generates the next pseudo-random 64-bit unsigned integer using XorShift `[13, 7, 17]`.
    ///
    /// - Returns: The next pseudo-random `u64` in the sequence.
    pub fn next(&mut self) -> u64 {

        const MARSAGLIA_SHIFTS: [u32; 3] = [13, 7, 17];

        let mut bd: u64 = self.seed; // bit diffusion

        bd ^= bd << MARSAGLIA_SHIFTS[0]; // mix lower bits
        bd ^= bd >> MARSAGLIA_SHIFTS[1]; // mix upper bits
        bd ^= bd << MARSAGLIA_SHIFTS[2]; // eliminate residual patterns
        self.seed = bd;

        self.seed
    }

    /// Generates a pseudo-random integer within an inclusive range.
    ///
    /// - `min`: Lower bound of the range (inclusive).
    /// - `max`: Upper bound of the range (inclusive).
    /// - Returns: A pseudo-random `u64` in `[min, max]`, or `min` if `min >= max`.
    pub fn next_range(&mut self, min: u64, max: u64) -> u64 {

        if min >= max { return min; }
        let diff: u64 = max - min + 1;

        // Unbiased Range Scaling (Lemire's Method)

        let mut mpl: u128 = (self.next() as u128) * (diff as u128); // multiple/product
        let mut lb:  u64  = mpl as u64;                             // lower bits

        if lb < diff {
            let thres: u64 = diff.wrapping_neg() % diff;
            while lb < thres {
                mpl = (self.next() as u128) * (diff as u128);
                lb  = mpl as u64;
            }
        }

        min + (mpl >> 64) as u64
    }

    /// Generates a series of pseudo-random numbers within an inclusive range.
    ///
    /// - `count`:  Number of pseudo-random values to generate.
    /// - `lb`:     Lower bound of the range (inclusive).
    /// - `ub`:     Upper bound of the range (inclusive).
    /// - `unique`: If `true`, the series will contain unique numbers without duplicates.
    /// - Returns: A `Vec<u64>` containing the generated pseudo-random numbers.
    pub fn next_series(&mut self, count: usize, mut lb: u64, mut ub: u64, unique: bool) -> Vec<u64> {

        // validate arguments

        if lb >= ub {
            warn!("lower bounds ({}) > upper bounds ({}) ... bounds will be swapped", lb, ub);
            std::mem::swap(&mut lb, &mut ub);
        }

        let diff: u64 = ub - lb; // bounds difference

        let mut fault_flags: u32 = 0;

        fault_flags |= if count == 0                      { 0b0001 } else {0}; // count of 0 is invalid
        fault_flags |= if diff  == 0                      { 0b0010 } else {0}; // bounds difference of 0 is invalid
        fault_flags |= if unique && count > diff as usize { 0b0100 } else {0}; // bounds difference too small for unique series

        if fault_flags != 0 {
            error!("Invalid parameter math returns empty series... fault_flags: {:04b}", fault_flags);
            return Vec::new();
        };

        // generate series

        let mut series: Vec<u64> = Vec::with_capacity(count);

        if unique {
            let mut set: HashSet<u64> = HashSet::with_capacity(count);
            while series.len() < count {
                let v: u64 = self.next_range(lb, ub);
                if set.insert(v) { series.push(v); }
            }
        } else {
            while series.len() < count { series.push(self.next_range(lb, ub)); }
        }

        series
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    const TEST_SEED: u64 = 8192;

    #[test]
    fn new_init() {
        let rng = Rng::new();
        assert_ne!(rng.seed, 0);
    }

    #[test]
    fn zero_seed_fallback() {
        let rng = Rng::from_seed(0);
        assert_eq!(rng.seed, Rng::FALLBACK_SEED);
    }

    #[test]
    fn determinism() {
        let mut rng1 = Rng::from_seed(TEST_SEED);
        let mut rng2 = Rng::from_seed(TEST_SEED);

        assert_eq!(rng1.next(), rng2.next());
        assert_eq!(rng1.next(), rng2.next());
    }

    #[test]
    fn range_bounds() {
        let mut rng = Rng::from_seed(TEST_SEED);
        let lb: u64 = 5;
        let ub: u64 = 15;

        for _ in 0..100 {
            let val: u64 = rng.next_range(lb, ub);
            assert!((lb..=ub).contains(&val));
        }
    }

    #[test]
    fn next_range_inverted_or_equal() {
        let mut rng = Rng::from_seed(TEST_SEED);

        let eq_bounds: u64 = 7;
        assert_eq!(rng.next_range(eq_bounds, eq_bounds), eq_bounds);

        let inv_max:   u64 = 5;
        let inv_min:   u64 = 10;
        assert_eq!(rng.next_range(inv_min, inv_max), inv_min);
    }

    #[test]
    fn next_series() {
        let mut rng = Rng::from_seed(TEST_SEED);
        let count: usize = 5;
        let lb:    u64   = 0;
        let ub:    u64   = 47;

        let series: Vec<u64> = rng.next_series(count, lb, ub, true);
        assert_eq!(series.len(), count);

        let set: HashSet<_> = series.iter().collect();
        assert_eq!(set.len(), count, "Elements within series must be unique");
        for &val in &series {
            assert!((lb..=ub).contains(&val));
        }
    }

    #[test]
    fn next_series_unique_exceeds_range() {
        let mut rng = Rng::from_seed(TEST_SEED);
        let count: usize = 10;
        let lb:    u64   = 0;
        let ub:    u64   = 3;

        let series: Vec<u64> = rng.next_series(count, lb, ub, true);
        assert!(series.is_empty());
    }

    #[test]
    fn next_series_non_unique() {
        let mut rng = Rng::from_seed(TEST_SEED);
        let count: usize = 20;
        let lb:    u64   = 0;
        let ub:    u64   = 1;

        let series: Vec<u64> = rng.next_series(count, lb, ub, false);
        assert_eq!(series.len(), count);
        for &val in &series {
            assert!((lb..=ub).contains(&val));
        }
    }

    #[test]
    fn next_series_zero_count() {
        let mut rng = Rng::from_seed(TEST_SEED);
        let series = rng.next_series(0, 0, 10, false);
        assert!(series.is_empty());
    }

    #[test]
    fn next_series_inverted_bounds_swapped() {
        let mut rng = Rng::from_seed(TEST_SEED);
        let count: usize = 5;
        let lb:    u64   = 50;
        let ub:    u64   = 10;

        let series = rng.next_series(count, lb, ub, false);
        assert_eq!(series.len(), count);
        for &val in &series {
            assert!((10..=50).contains(&val));
        }
    }
}
