# Inertia-core

Inertia is a (WIP) computational mathematics library for Rust. 

Inertia-core contains the core functionality of the [Inertia](https://github.com/wjyoumans/inertia) crate, providing high-level wrappers for the [FLINT](https://flintlib.org/doc/), [Arb](https://arblib.org/), and [Antic](https://github.com/wbhart/antic) C libraries.

## Performance

The following is just meant to give a rough idea of the performance of Inertia-core vs. standard big integer libraries. Any feedback or suggestions to improve benchmarking are welcome. The full benchmark output is available [here](https://htmlpreview.github.io/../bench/report/index.html).

This figure compares big (> 64 bit) integer multiplication times between Inertia-core, the [num](https://crates.io/crates/num) `BigInt` type, and the [rug](https://crates.io/crates/rug) `Integer` type.
More precisely, we simply multiply 2^64 by 2^64^x for x = 1, 2, 4, and 8 (exponentiation time is not included in the benchmarks). 

Note that rug integers are a high-level wrapper for the [GMP](https://crates.io/crates/rug) library, so the performance comparison here is essentially between num, GMP, and FLINT integer multiplication.
![Figure 1](/../bench/Integer-mul/report/lines.svg?raw=true&sanitize=true)
