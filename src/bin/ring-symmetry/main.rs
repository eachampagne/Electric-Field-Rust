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
            let theta = theta1 + j as f64 + 0.5) * dtheta;

            //The electric field along the axis of a ring of charge is given as
            //E = (2 * pi * k * lambda * r' * z) / (r'^2+z^2)^(3/2)
            //where r' is the radius of the ring, z is the height above the ring, and k encodes all
            //of the relevant constants (epsilon_naught, etc)
            //(I derived this on paper and checked against the Hyperphysics entry, so I used their
            //conventions)
            //TODO: add link to Hyperphysics
            //
            //To 
        }

    }
}
