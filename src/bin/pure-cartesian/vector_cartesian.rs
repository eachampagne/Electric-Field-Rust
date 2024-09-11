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

    //Commentary: similarly to the above, I think this is technically not a constructor, but it
    //functions as one
    //This is copied from the original conversion function from my naive two-struct setup
    pub fn from_spherical(r:f64, theta:f64, phi:f64) -> Vector {
        let x = r * phi.cos() * theta.sin();
        let y = r * phi.sin() * theta.sin();
        let z = r * theta.cos();
        return Vector::new(x, y, z);
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

pub fn test() {
    println!("this is the other file");    
}
