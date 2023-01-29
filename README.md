## inertia-core

**inertia-core** contains the core functionality of the
[Inertia](https://github.com/wjyoumans/inertia) crate, providing high-level 
wrappers for the [FLINT](https://flintlib.org/doc/), 
[Arb](https://arblib.org/), and [Antic](https://github.com/wbhart/antic) 
C libraries.

<!--
TODO:
 * rand - see Rug/gmp-mpfr-sys
 * tons of boilerplate, docs, TODO/FIXME comments
 * rest of Flint types, factorization, FFT, quadratic sieve etc.
 * Arb + Antic
 * serde
 * TryFrom conversions
 * borrows for FFI types
 * improve op guards to avoid seg faults
 * improved constructors - New/NewCtx may be suboptimal in some situations
 * better polynomial/matrix pretty printing
 * split into features or workspace (integer, rational, etc.)
-->
