use inertia_core::*;

fn main() {
    let a: Integer = "2".parse().unwrap();
    let a = a.pow(10u8);
    let b = Integer::from(10);
    let c = Rational::from([a, b]);
    println!("{}", c);

    let zn = IntModRing::init(10);
    let z = zn.new(12);
    println!("{}", Rational::from(z*7u32));

    let c0 = Integer::from(1);
    let c1 = Integer::from(2);
    let v = vec![c0, c1];

    let _ = IntPoly::from(v.as_slice());
    let p = IntPoly::from(v);
    println!("{}", &p);

    let mm = IntMatSpace::init(2, 2);
    let m = mm.new(&[
                   1, 2, 
                   3, 4
            ]);
    println!("{}", m);
}

