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

    let test_2 = Vector {x:4.0, y:6.0, z:8.0};
    test_2.trace();
    
    (vec_test + test_2).trace();
    (vec_test-test_2).trace();
    (test_2+vec_test).trace();
    (test_2-vec_test).trace();
    (vec_test*3.0).trace();
    (3.0*vec_test).trace();

    sphere_test.trace();
    (5.0*sphere_test).trace();
    (sphere_test*5.0).trace();
}
