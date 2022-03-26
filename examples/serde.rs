use inertia_core::*;
use inertia_core::ops::Pow;

macro_rules! test_bincode {
    ($x:ident, $t:ty) => {
        println!("Before: \n{}", $x);

        match $x.write_bincode("test.dat") {
            Err(e) => panic!("{}", e),
            Ok(_) => (),
        }

        if let Ok(y) = <$t>::read_bincode("test.dat") {
            println!("After: \n{}", y);
        }
    };
}

fn main() {
    let a = Integer::from("2").pow(10u32);
    test_bincode!(a, Integer);
    
    let a = Integer::from("21864736487264827439837428");
    test_bincode!(a, Integer);
   
    let a = Rational::from([100, 12]);
    test_bincode!(a, Rational);
    
    let a = IntPoly::from(vec![1,0,0,0,1]);
    test_bincode!(a, IntPoly);
    
    let a = IntMat::from(vec![vec![1, 2], vec![3, 4]]);
    test_bincode!(a, IntMat);
    
    let zn = IntModRing::init(12);
    let a = zn.new(321);
    test_bincode!(a, IntMod);
   
    /*
    let zn = FiniteField::init(3, 4);
    let a = zn.new(vec![1,0,0,0,0,1]);
    test_bincode!(a, FinFldElem);
    */
}

