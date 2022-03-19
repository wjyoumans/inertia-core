use inertia_core::*;
use inertia_core::ops::Pow;

fn main() {
    let a = Integer::from("2").pow(10u32);
    let b = Integer::from(10);
    let c = Rational::from([a, b]);
    println!("{}", c);

    let zn = IntModRing::init(10);
    let z = zn.new("12");
    println!("{}", Rational::from(z*7u32));

    let c0 = Integer::from(1);
    let c1 = Integer::from(2);
    let _ = IntPoly::from(vec![&c0, &c1]);
    let _ = IntPoly::from(vec![&c0, &c1].as_slice());
    let p = IntPoly::from(vec![c0, c1]);
    println!("{}", &p);

    let mm = IntMatSpace::init(2, 2);
    let m = mm.new(vec![vec![1, 2], vec![3, 4]]);
    println!("{}", m);
}

