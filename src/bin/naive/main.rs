mod vectors_naive;

use crate::vectors_naive::*;

fn main() {
    println!("Hello, world!");
    test();

    let vec_test = Vector {x:1.0, y:2.0, z:3.0};
    vec_test.trace();

    let sphere_test = SphereVector {r:2.0, theta:10.0, phi:20.0};
    sphere_test.trace();

    println!("{}", vec_test.mag());
    vec_test.normalize().trace();
}
