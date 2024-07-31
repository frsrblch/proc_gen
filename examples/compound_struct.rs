use proc_gen::{KeyFor, Prng, PrngKey, Seed};
use rand::{thread_rng, Rng};

#[allow(clippy::disallowed_names)]
fn main() {
    let mut rng = thread_rng();
    let seed = rng.gen();
    let key = Key(rng.gen());

    let foo = Foo::gen(seed, key, FooCtx);
    let bar = Bar::gen(seed, key, BarCtx);
    let ctx = BazCtx {
        foo: FooCtx,
        bar: BarCtx,
    };
    let baz = Baz::gen(seed, key, ctx);

    assert_eq!(Baz { foo, bar }, baz);
}

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Foo(f64);

impl Foo {
    pub fn gen(seed: Seed, key: Key, _ctx: FooCtx) -> Self {
        Self(Self::prng(seed, key).gen())
    }
}

pub struct FooCtx;

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Bar(f64);

impl Bar {
    pub fn gen(seed: Seed, key: Key, _ctx: BarCtx) -> Self {
        Self(Self::prng(seed, key).gen())
    }
}

pub struct BarCtx;

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Baz {
    pub foo: Foo,
    pub bar: Bar,
}

impl Baz {
    pub fn gen(seed: Seed, key: Key, ctx: BazCtx) -> Self {
        Self {
            foo: Foo::gen(seed, key, ctx.foo),
            bar: Bar::gen(seed, key, ctx.bar),
        }
    }
}

pub struct BazCtx {
    foo: FooCtx,
    bar: BarCtx,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Key(u64);

impl PrngKey for Key {
    fn key(&self) -> u64 {
        self.0
    }
}

impl KeyFor<Foo> for Key {
    const XOR: u128 = 1234567890;
}

impl KeyFor<Bar> for Key {
    const XOR: u128 = 321654987;
}

impl KeyFor<Baz> for Key {
    const XOR: u128 = 741852963;
}
