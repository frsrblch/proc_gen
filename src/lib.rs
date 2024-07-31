//! Types and traits for deterministic pseudorandom number generators (PRNGs).
//!
//! To generate values of type `T`, a key must implement and [`KeyFor<T>`] and [`PrngKey`].
//!
//! [`Prng`] is used to instantiate the PRNG

#![warn(missing_docs)]

#[cfg(test)]
mod tests;

/// Seed values for procedurally generating deterministic pseudorandom numbers
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Seed(u128);

impl Seed {
    /// Generate a `Seed` by hashing an input `&str`
    #[inline]
    pub fn new_from_str(seed: &str) -> Self {
        let hash = &blake3::hash(seed.as_bytes());
        let bytes = std::array::from_fn(|i| hash.as_bytes()[i]);
        let u128 = u128::from_ne_bytes(bytes);
        Seed(u128)
    }
}

impl From<u128> for Seed {
    #[inline]
    fn from(value: u128) -> Self {
        Seed(value)
    }
}

impl rand::distributions::Distribution<Seed> for rand::distributions::Standard {
    #[inline]
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Seed {
        Seed(rng.gen())
    }
}

/// Generate a (seed, key, type)-specific PRNG
///
/// Higher keys will take longer to generate, working in `O(log₂(n))` time
pub trait Prng<Key>: Sized
where
    Key: KeyFor<Self>,
{
    /// Generate the PRNG for a (seed, key, type) tuple
    #[inline]
    fn prng(seed: Seed, key: Key) -> impl rand::Rng {
        let rng_seed = seed.0 ^ Key::XOR;

        // this bitshift gives us 256 unique values for each (seed, key, type) tuple
        let advance = (key.key() as u128) << Key::BITSHIFT;

        let mut rng = rand_pcg::Pcg64Mcg::new(rng_seed);
        rng.advance(advance);
        rng
    }
}

impl<T, Key> Prng<Key> for T where Key: KeyFor<T> {}

/// Values of this type can be used as keys when generating deterministic pseudrandom values
pub trait PrngKey: Copy {
    /// Convert the key into a `u64` value that is used to advance the PRNG such that it produces unique values for each key
    fn key(&self) -> u64;
}

/// Values of this type can be used to generate psuedorandom values of `T`
pub trait KeyFor<T>: PrngKey {
    /// A hard-coded random number that is xor'ed with the seed value to produce values that are unique to that (seed, key, type) tuple
    const XOR: u128;

    /// Key values are cast to u128 and bitshifted before being used to advance the PRNG.
    ///
    /// Shifting by the default value of 8 creates a PRNG with 256 unique values per key.
    ///
    /// This bitshift can be adjusted if more or fewer unique values are needed per key.
    const BITSHIFT: u32 = 8;
}
