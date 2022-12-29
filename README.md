## inertia-core

This is a work in progress!

**inertia-core** contains the core functionality of the
[Inertia](https://github.com/wjyoumans/inertia) crate, providing high-level 
wrappers for the [FLINT](https://flintlib.org/doc/), 
[Arb](https://arblib.org/), and [Antic](https://github.com/wbhart/antic) 
C libraries.


TODO:
 * tons of boilerplate, docs, TODO/FIXME comments
 * rest of Flint
 * Arb + Antic
 * serde
 * borrows for FFI types
 * improve op guards to avoid seg faults
 * improved constructors - New/NewCtx may be suboptimal in some situations
 * better polynomial/matrix pretty printing
 * split into features or workspace (integer, rational, etc.)
