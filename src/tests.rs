use super::*;

use rand::{thread_rng, Rng};

#[derive(Debug, Copy, Clone)]
pub struct ValueKey(u64);

impl ValueKey {
    pub fn new(index: u64) -> Self {
        ValueKey(index)
    }
}

impl PrngKey for ValueKey {
    fn key(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, PartialEq)]
struct Value1(f64);

impl Value1 {
    pub fn gen(seed: Seed, key: ValueKey) -> Self {
        Self(Self::prng(seed, key).gen())
    }
}

impl KeyFor<Value1> for ValueKey {
    const XOR: u128 = 1;
}

#[derive(Debug, PartialEq)]
struct Value2(f64);

impl Value2 {
    pub fn gen(seed: Seed, key: ValueKey) -> Self {
        Self(Self::prng(seed, key).gen())
    }
}

impl PartialEq<Value2> for Value1 {
    fn eq(&self, other: &Value2) -> bool {
        self.0.eq(&other.0)
    }
}

impl KeyFor<Value2> for ValueKey {
    const XOR: u128 = 2;
}

#[test]
fn same_key_and_same_type_returns_same_values() {
    let seed = Seed::new_from_str("value test");
    let key = ValueKey::new(7);
    let value1a = Value1::gen(seed, key);
    let value1b = Value1::gen(seed, key);
    assert_eq!(value1a, value1b);
}

#[test]
fn same_key_and_different_type_returns_different_values() {
    let seed = Seed::new_from_str("value test");
    let key = ValueKey::new(7);
    let value1 = Value1::gen(seed, key);
    let value2 = Value2::gen(seed, key);
    assert_ne!(value1, value2);
}

#[test]
fn unit_key_return_consistent_values() {
    let seed = Seed::new_from_str("global test");

    /// Prng global values
    #[derive(Debug, PartialEq)]
    pub struct Global(f64);

    impl KeyFor<Global> for () {
        const XOR: u128 = 635184615;
    }

    impl rand::distributions::Distribution<Global> for rand::distributions::Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Global {
            Global(rng.gen())
        }
    }

    let global1 = Global::prng(seed, ()).gen::<Global>();
    let global2 = Global::prng(seed, ()).gen::<Global>();

    assert_eq!(global1, global2);
}

#[test]
fn zero_and_one_generate_different_values() {
    let seed = Seed::new_from_str("test");
    let k1 = ValueKey(0);
    let k2 = ValueKey(1);
    assert_ne!(
        Value1(Value1::prng(seed, k1).gen()),
        Value1(Value1::prng(seed, k2).gen())
    );
}

#[test]
fn seed_from_string_zero_at_end() {
    let a = Seed::new_from_str("test");
    let b = Seed::new_from_str("test0");

    assert_ne!(a, b);
    dbg!((a.0 ^ b.0).count_ones());
}

#[test]
fn seed_from_string_spaces_at_end() {
    let a = Seed::new_from_str("test");
    let b = Seed::new_from_str("test ");

    assert_ne!(a, b);
    dbg!((a.0 ^ b.0).count_ones());
}

#[test]
fn seed_from_string_empty_vs_space() {
    let a = Seed::new_from_str("");
    let b = Seed::new_from_str(" ");

    assert_ne!(a, b);
    dbg!((a.0 ^ b.0).count_ones());
}

#[test]
fn uniqueness_by_key() {
    let mut rng = thread_rng();

    let seed = rng.gen();
    let key = rng.gen();

    let mut a = Value1::prng(seed, ValueKey(key));
    let mut b = Value1::prng(seed, ValueKey(key + 1));

    let a_values = (0..256)
        .map(|_| a.gen::<u64>())
        .collect::<std::collections::HashSet<_>>();
    let b_values = (0..256)
        .map(|_| b.gen::<u64>())
        .collect::<std::collections::HashSet<_>>();

    assert!(!a_values.iter().any(|n| b_values.contains(n)));
}

#[test]
fn prng_overlap() {
    struct Values([u64; 2]);
    struct Key(u64);

    impl KeyFor<Values> for Key {
        const XOR: u128 = 0;
        const BITSHIFT: u32 = 0;
    }

    impl PrngKey for Key {
        fn key(&self) -> u64 {
            self.0
        }
    }

    impl Values {
        pub fn new(seed: Seed, key: Key) -> Self {
            Self(rand::Rng::gen(&mut Self::prng(seed, key)))
        }
    }

    let seed = Seed(0);

    let a = Values::new(seed, Key(0)).0;
    let b = Values::new(seed, Key(1)).0;

    // these two values overlap because a bitshift of zero creates a PRNG with one unique value
    assert_eq!(a[1], b[0]);
}
