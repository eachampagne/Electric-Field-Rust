use std::f64::consts::PI;

fn main() {
    let n:u32 = 1000; //Number of divisions
    let R:f64 = 1.0; //arbitrary length
    let Q:f64 = 1.0; //arbitrary charge

    //Bounds of integration
    let theta1:f64 = 1.0 * PI /180.0;
    let theta2:f64 = PI;

    let dtheta = (theta2 - theta1) / n as f64;

    //Integrate over the infintesimal rings
    for i in 0..501 { //location along z-axis
        let z:f64 = i as f64 / 100.0;

        let mut E_z = 0.0; //z-component of electric field
                           //this doesn't need to be a vector because azimuthal symmetry guarantees
                           //vanishing x and y components

        for j in 0..n { //theta
            let theta = theta1 + (j as f64 + 0.5) * dtheta;

            //The electric field along the axis of a ring of charge is given as
            //E = (2 * pi * k * lambda * r' * z) / (r'^2+z^2)^(3/2)
            //where r' is the radius of the ring, lambda is the linear charge density, 
            //z is the height above the ring, and k encodes all
            //of the relevant constants (epsilon_naught, etc)
            //(I derived this on paper and checked against the Hyperphysics entry, so I used their
            //conventions)
            //Hyperphysics article: http://hyperphysics.phy-astr.gsu.edu/hbase/electric/potlin.html#c2
            //
            //When integrating over a number of rings, the contribution of each ring becomes
            //dE = (2 * pi * k * r' * z' * sigma * R) / (r'^2 + z'^2)^(3/2) dtheta
            //where sigma is the area charge density, R is the radius of the sphere (equals 1 in
            //this program), r' is the radius of each ring, and z' is the difference in z between
            //the ring and z, the point at which we want to compute the electric field.
            //
            //z' and r' are theta dependent.
            //
            //r' = sin(theta) and z' = z - cos(theta)
            //
            //So the contribution of each ring in terms of theta becomes
            //dE = (2 * pi * k * sigma * sin(theta)(z-cos(theta))) / (z^2 - 2zcos(theta) + 1) ^ (3/2) dtheta
            //
            //TODO: figure out which constants I actually included in the other program
            //I think I'm at least assuming k = 1
            //Adding a factor of 2 pi worked, apparently sigma and k are set to 1
            //and I have a sign discrepancy
            //I'm not actually certain which one is right - I mix up whether electric field lines
            //point to protons or electrons
            
            let dE = 2.0 * PI * (theta.sin()*(z-theta.cos())) / (z*z-2.0*z*theta.cos()+1.0).powf(1.5) * dtheta;
            E_z += dE;
        }

        println!("{}", E_z);

    }
}
