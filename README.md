## inertia-core

**inertia-core** contains the core functionality of the
[Inertia](https://github.com/wjyoumans/inertia) crate, providing high-level 
wrappers for the [FLINT](https://flintlib.org/doc/), 
[Arb](https://arblib.org/), and [Antic](https://github.com/wbhart/antic) 
C libraries.

<!--
TODO:
 * swap op
 * better name distancing it from inertia
 * split into features or workspace (maybe make workspace with flint, arb, antic 
 crates with individual features?)
 * add LGPL
 * rand - see Rug/gmp-mpfr-sys
 * tons of boilerplate, docs, TODO/FIXME comments
 * rest of Flint, Arb, Antic types
 * serde
 * TryFrom conversions
 * borrows for FFI types
 * improve op guards to avoid seg faults in C
 * improved constructors - New/NewCtx may be suboptimal in some situations
 * better polynomial/matrix pretty printing
 * (unsafe?) shallow copies could avoid unnecessary allocations in some contexts, like
 some hash impls
 * combine From/Assign impl macros, maybe derive From from Assign impls?
 * macros could use general improvements, proc macro crate for op and From impls
 in inertia-generic and inertia-core might be ideal
 * split into features or workspace (integer, rational, etc.)
-->
