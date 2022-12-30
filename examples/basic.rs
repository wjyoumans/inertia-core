use inertia_core::*;

// - demonstrate construction and use of each type
// - some examples of ops between types
// - some examples of Assign and From 

fn main() {
    // `new` is the standard constructor.
    let x = Integer::new(10);
    println!("x = {}", &x);

    // Integers and rationals can be parsed from a string literal.
    let y = "1/2".parse::<Rational>().unwrap();
    println!("y = {}", &y);

    // Rationals can also be constructed from an array of length 2.
    let z = Rational::new([1, 2]);
    println!("z = {}", &z);

    // Polynomials can be constructed from an array.
    let f = IntPoly::new([1, 0, 0, 0, -1]);
    println!("f = {}", &f);

    // They can also be constructed from a slice.
    let coeffs = vec![1, 1, 1];
    let g = RatPoly::new(&coeffs[..]);
    println!("g = {}", &g);

    // Operations are implemented wherever they make sense and for all 
    // combinations of owned or borrowed values.
    let h = (&f + (g/2i32 * &x)).pow(2u32);
    println!("(f + (g/2 + x))^2 = {}", &h);

    // To work with objects like integers modulo n, finite field elements, etc.
    // we need to make a context object first.
    // This is a context for the ring of integers modulo 12.
    let zn_ctx = IntModCtx::new(12);
    println!("zn_ctx = {}", &zn_ctx);

    // The context is needed in the constructor.
    let a = IntMod::new(13, &zn_ctx);
    let b = IntMod::one(&zn_ctx);
    assert_eq!(&a, &b);

    // This is a context for the finite field with 3^2 = 9 elements.
    let fq_ctx = FinFldCtx::new(3, 2);
    println!("fq_ctx = {}", &fq_ctx);

    // Create finite field elements by specifying their coefficients as a polynomial.
    let a = FinFldElem::new([-1, -1], &fq_ctx);
    println!("a = {}", &a);

    // Construct a 4x4 integer matrix filled with zeros.
    let m1 = IntMat::zero(4, 4);
    println!("m1 = \n{}", &m1);

    // Construct an identity integer matrix.
    let m2 = IntMat::one(4);
    println!("m2 = \n{}", &m2);

    // Specify coefficients with a slice or array.
    let m3 = IntMat::new(
        [1, 2, 3, 4, 
         2, 3, 4, 1, 
         3, 4, 1, 2,
         4, 1, 2, 3], 4, 4);
    println!("m3 = \n{}", &m3);

    let t = (&m1 + 2*&m2) * m3;
    println!("(m1 + 2*m2) * m3 = \n{}", &t);

    // Construct rational functions just like construct rationals
    let g = IntPoly::new([1, 2, 3]);
    let h = RatFunc::from([f, g]);
    println!("h = {}", &h);
}
