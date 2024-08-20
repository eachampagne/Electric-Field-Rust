mod vectors_naive;

use crate::vectors_naive::*;

fn main() {
    println!("Hello, world!");
    test();

    let vec_test = Vector::new(1.0, 2.0, 3.0);
    vec_test.trace();

    let sphere_test = SphereVector::new(2.0, 10.0, 20.0);
    sphere_test.trace();

    println!("{}", vec_test.mag());
    vec_test.normalize().trace();

    let test_2 = Vector::new(4.0, 6.0, 8.0);
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

    println!("{}", Vector::dot(vec_test, test_2));
    println!("{}", Vector::dot(test_2, vec_test));
    
    Vector::cross(vec_test, test_2).trace();
    Vector::cross(test_2, vec_test).trace();

}
