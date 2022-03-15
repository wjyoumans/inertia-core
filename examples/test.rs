use inertia_core::{Integer, IntModRing};

fn main() {
    let zn = IntModRing::init(10);
    let z = zn.new("12");
    println!("{}", z);
}

