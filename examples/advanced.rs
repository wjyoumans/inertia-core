use inertia_core::*;

fn main() {
    let qf = BinQuadForm::default();
    println!("{}", &qf);

    let qf = BinQuadForm::from([1, 0, -1]);
    println!("{}", &qf);

    let f = IntPoly::cyclotomic(7);
    let nf = NumFldCtx::new(f);
    println!("{}", &nf);

    let a = NumFldElem::new([1, 2, 3, 4, 5, 6, 7, 8], &nf);
    println!("{}", &a);
    
    // f = x^3 - x^2 - 2x - 8
    let f = IntPoly::new([-8, -2, -1, 1]);
    let nf = NumFldCtx::new(f);
    println!("{}", &nf);

    let a = NumFldElem::new([1, 1, 1, 1], &nf);
    println!("{}", &a);
}
