mod lib;

use lib::CoarseLogProb;

fn main() {

    let a = CoarseLogProb::from(-40.0f32);
    let b = CoarseLogProb::from(-40.001f32);

    let av: f32 = a.into();
    let bv: f32 = b.into();
    println!("{:?}, {:?}, {}, {}", a, b, av, bv);
}
