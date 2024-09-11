mod vector_cartesian;

use crate::vector_cartesian::*;
use std::f64::consts::PI;

fn main() {
    //Comment out all the "tests"
    //println!("Hello, world!");
    //test();

    //let vec_test = Vector::new(1.0, 2.0, 3.0);
    //vec_test.trace();

    //let sphere_test = SphereVector::new(2.0, 10.0, 20.0);
    //sphere_test.trace();

    //println!("{}", vec_test.mag());
    //vec_test.normalize().trace();

    //let test_2 = Vector::new(4.0, 6.0, 8.0);
    //test_2.trace();
    
    //(vec_test + test_2).trace();
    //(vec_test-test_2).trace();
    //(test_2+vec_test).trace();
    //(test_2-vec_test).trace();
    //(vec_test*3.0).trace();
    //(3.0*vec_test).trace();

    //sphere_test.trace();
    //(5.0*sphere_test).trace();
    //(sphere_test*5.0).trace();

    //println!("{}", Vector::dot(vec_test, test_2));
    //println!("{}", Vector::dot(test_2, vec_test));
    
    //Vector::cross(vec_test, test_2).trace();
    //Vector::cross(test_2, vec_test).trace();

    //sphere_test.trace();
    //sphere_test.toCartesian().trace();
    //sphere_test.toCartesian().toSpherical().trace();

    //vec_test.trace();
    //vec_test.toSpherical().trace();
    //vec_test.toSpherical().toCartesian().trace();
    
    let mut integralSum = Vector::new(0.0,0.0,0.0); //Commentary: why is this here? It gets overwritten in the loop
    
    //A few constants
    let TAU:f64 = 2.0 * PI;
    let EPSILON_NAUGHT:f64 = 8.854187817*10.0_f64.powf(-12.0); //F/m

    let n:u32 = 1000; //Number of divisions
    let R:f64 = 1.0; //arbitrary length
    let Q:f64 = 1.0; //arbitrary charge

    //Bounds of integration
    let theta1:f64 = 1.0 * PI / 180.0;
    let theta2:f64 = PI;
    let phi1 = 0.0;
    let phi2 = TAU; //Commentary: did I define a constant just for this one line? I bet I did. I was on the tau bandwagon for a while
    
    let dtheta = (theta2 - theta1) / n as f64;
    let dphi = (phi2 - phi1) / n as f64;

    //Integrate for A, i.e. the area of the charged shell
    //which is *not* the area of a sphere because of the hole
    //The proper area is important for getting the charge density right
    //Although it looks like I never used these values at all
    //At least it lets me test my for loops?
    let mut A:f64 = 0.0;
    for j in 0..n { //theta
        let theta:f64 = theta1 + (j as f64 + 0.5) * dtheta;
        for k in 0..n { //phi
            let phi:f64 = phi1 + (k as f64 + 0.5) * dphi;
            A += theta.sin() * dtheta * dphi * R.powf(2.0);
        }
    }

    let SIGMA:f64 = Q/A; //Surface charge density
    let CONSTANTS = SIGMA / (4.0 * PI * EPSILON_NAUGHT); //I have a comment saying I had decided "this" (presumably referring to trying to deal with physical units?) was a bad idea, but I never removed the code for that

    //Integrate the actual field
    for i in 0..501 { //where along z-axis we're measuring
        let z:f64 = i as f64 / 100.0;
        let mut integralSum = Vector::new(0.0, 0.0, 0.0);
        let testPoint = Vector::new(0.0, 0.0, z);
        for j in 0..n {//theta
            let theta = theta1 + (j as f64 + 0.5) * dtheta;
            for k in 0..n { //phi
                let phi = phi1 + (k as f64 + 0.5) * dphi;
                let currentVector:Vector = Vector::from_spherical(R, theta, phi); //Coordinates of point on shell
                let shellVector:Vector = currentVector - testPoint; //from z-axis to shell
                //let toShell:SphereVector = shellVector.toSpherical(); //from z-axis to shell, in spherical coordinates
                //let mut currentField:SphereVector = SphereVector::new(0.0,0.0,0.0); //Commentary: why???
                let currentField = shellVector.normalize() * (1.0 * theta.sin() * dtheta * dphi * shellVector.mag().powf(-2.0));
                integralSum = integralSum + currentField;
            }
        }
        integralSum.trace();
    }

}
