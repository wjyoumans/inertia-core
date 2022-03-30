# Inertia-core

Inertia is a (WIP) computational mathematics library for Rust. 

Inertia-core contains the core functionality of the [Inertia](https://github.com/wjyoumans/inertia) crate, providing high-level wrappers for the [FLINT](https://flintlib.org/doc/), [Arb](https://arblib.org/), and [Antic](https://github.com/wbhart/antic) C libraries.

## Performance

This figure compares big (> 64 bit) integer multiplication times between Inertia-core, the [num](https://crates.io/crates/num) `BigInt` type, and the [rug](https://crates.io/crates/rug) `Integer` type.

Note that `rug` integers are a high-level wrapper for the [GMP](https://crates.io/crates/rug) library, so the performance comparison here is essentially between `num`, `GMP`, and `FLINT` integer multiplication.
![Figure 1](/../bench/Integer-mul/report/lines.png?raw=true)
