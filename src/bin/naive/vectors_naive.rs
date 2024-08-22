use std::ops;
use std::f64::consts::PI; 

#[derive(Copy, Clone)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    
    //Commentary:
    //I don't like the order I defined these functions in, but for the purpose of creating a
    //line-by-line replica I guess I'll keep it at least in this version

    //Commentary: I believe that technically, the only true "constructor" is the curly brace
    //syntax (see https://doc.rust-lang.org/nomicon/constructors.html). This is simply a function that
    //acts like a constructor. This does allow me to keep the fields private however, and I think is
    //slightly easier to write out.
    pub fn new(x:f64, y:f64, z:f64) -> Vector {
        return Vector{x:x, y:y, z:z};
    }

    //Commentary:
    //I have no idea why I didn't make this edit the vector in place
    //In Rust, this would work even if the vector isn't mutable
    //But that wouldn't have been a concern in Python
    pub fn normalize(&self) -> Vector {
        let mag = self.mag();
        return Vector::new(self.x/mag, self.y/mag, self.z/mag);
    }

    pub fn trace(&self) {
        println!("<{0}, {1}, {2}>", self.x, self.y, self.z);
    }

    pub fn toSpherical(&self) -> SphereVector {
        let r = self.mag();
        let mut theta = 0.0;
        let mut phi = 0.0;

        if self.z > 0.0 {
            theta = (self.z/r).acos();
        } else if self.z < 0.0 {
            theta = PI - (-self.z/r).acos();
        } else if self.z == 0.0 {
            theta = PI / 2.0;
        }

        if self.x > 0.0 { //Quadrants I and IV
            if self.y > 0.0 { //Quadrant I
                phi = (self.y/self.x).atan();
            } else if self.y < 0.0 { //Quadrant IV
                phi = 3.0 * PI * 0.5 + (self.x/ -self.y).atan();
            } else if self.y == 0.0 { //Positive x-axis
                phi = 0.0;
            }
        } else if self.x < 0.0 { //Quadrants II and III
            if self.y > 0.0 { //Quadrant II
                phi = PI * 0.5 + (-self.x/self.y).atan();
            } else if self.y < 0.0 { //Quadrant III
                phi = PI + (self.y/self.x).atan();
            } else if self.y == 0.0 { //Negative x-axis
                phi = PI;
            }
        } else if self.x == 0.0 { //Y axis
            if self.y > 0.0 { //Positive y-axis
                phi = PI / 2.0;
            } else if self.y < 0.0 { //Negative y-axis
                phi = 3.0 * PI / 2.0;
            } else if self.y == 0.0 { //On the z-axis, i.e., the origin
                phi = 0.0; //Since this is the origin the angle doesn't really matter, as long as it's defined
            } 
        }

        let spherical = SphereVector::new(r, theta, phi);
        return spherical;
    }

    pub fn dot(vect1:Vector, vect2:Vector) -> f64 {
        return vect1.x * vect2.x + vect1.y * vect2.y + vect1.z * vect2.z;
    }

    pub fn cross(vect1:Vector, vect2:Vector) -> Vector {
        let newX = vect1.y * vect2.z - vect1.z * vect2.y;
        let newY = -(vect1.x * vect2.z - vect1.z * vect2.x);
        let newZ = vect1.x * vect2.y - vect1.y * vect2.x;
        return Vector::new(newX, newY, newZ);
    }

    //Commentary:
    //I'm not sure offhand how to do fractional powers in Rust
    pub fn mag(&self) -> f64 {
        let magnitude:f64 = f64::sqrt(self.x*self.x+self.y*self.y+self.z*self.z);
        return magnitude;
    }

}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, _rhs: Vector) -> Vector {
        let x:f64 = self.x + _rhs.x; 
        let y:f64 = self.y + _rhs.y;
        let z:f64 = self.z + _rhs.z;
        return Vector::new(x, y, z);
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, _rhs: Vector) -> Vector {
        let x:f64 = self.x-_rhs.x;
        let y:f64 = self.y-_rhs.y;
        let z:f64 = self.z-_rhs.z;
        return Vector::new(x, y, z);
    }
}

//Scalar multiplication of form vector * scalar
impl ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, _rhs: f64) -> Vector {
        let x:f64 = self.x*_rhs;
        let y:f64 = self.y*_rhs;
        let z:f64 = self.z*_rhs;
        return Vector::new(x, y, z);
    }
}

//Scalar multiplication of form scalar * vector
impl ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, _rhs: Vector) -> Vector {
        let x:f64 = self*_rhs.x;
        let y:f64 = self*_rhs.y;
        let z:f64 = self*_rhs.z;
        return Vector::new(x, y, z);
    }
}

//Commentary: this makes no attempts to constrain the angles to [0, pi) or [0, 2pi) for theta and
//phi respectively. I suppose everything would still work if they looped over, but that was
//careless of me.
//Also, I can only assume that my old code used radians, but I didn't write a comment saying so...
#[derive(Copy, Clone)]
pub struct SphereVector {
    r: f64,
    theta: f64,
    phi: f64,
}

impl SphereVector {
    pub fn new(r:f64, theta:f64, phi:f64) -> SphereVector {
        return SphereVector{r:r, theta:theta, phi:phi};
    }

    pub fn trace(&self) {
        println!("<{0}, {1}, {2}>", self.r, self.theta, self.phi);
    }

    pub fn toCartesian(&self) -> Vector {
        let x = self.r * self.phi.cos() * self.theta.sin();
        let y = self.r * self.phi.sin() * self.theta.sin();
        let z = self.r * self.theta.cos();
        return Vector::new(x, y, z);
    }

    pub fn normalize(&self) -> SphereVector {
        return SphereVector::new(1.0, self.theta, self.phi);
    }

    //Commentary: added this function so I could keep my struct's fields private
    pub fn mag(&self) -> f64 {
        return self.r;
    }

}

//Scalar multiplication of form vector * scalar
impl ops::Mul<f64> for SphereVector {
    type Output = SphereVector;

    fn mul(self, _rhs: f64) -> SphereVector {
        let newR:f64 = self.r*_rhs;
        return SphereVector::new(newR, self.theta, self.phi);
    }
}

//Scalar multiplication of form scalar * vector
impl ops::Mul<SphereVector> for f64 {
    type Output = SphereVector;

    fn mul(self, _rhs: SphereVector) -> SphereVector {
        let newR:f64 = self*_rhs.r;
        return SphereVector::new(newR, _rhs.theta, _rhs.phi);
    }
}

pub fn test() {
    println!("this is the other file");    
}
