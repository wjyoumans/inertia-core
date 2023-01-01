use inertia_core::*;

fn main() {
    let qf = BinQuadForm::default();
    println!("{}", &qf);

    let qf = BinQuadForm::from([1, 0, -1]);
    println!("{}", &qf);
}
