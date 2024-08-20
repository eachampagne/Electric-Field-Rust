use std::ops;

//Commentary: how much do I care about this being line-for-line? Because right now my operator
//overloads are more compact than my original code

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    
    //Commentary:
    //I don't like the order I defined these functions in, but for the purpose of creating a
    //line-by-line replica I guess I'll keep it at least in this version

    //TODO: proper constructor

    //Commentary:
    //I have no idea why I didn't make this edit the vector in place
    //In Rust, this would work even if the vector isn't mutable
    //But that wouldn't have been a concern in Python
    //TODO: fix once I've figured out how constructors work
    pub fn normalize(&self) -> Vector {
        let mag = self.mag();
        return Vector{x:self.x/mag, y:self.y/mag, z:self.z/mag};
    }

    pub fn trace(&self) {
        println!("<{0}, {1}, {2}>", self.x, self.y, self.z);
    }

    //Commentary:
    //I'm not sure offhand how to do fractional powers in Rust
    pub fn mag(&self) -> f32 {
        let magnitude:f32 = f32::sqrt(self.x*self.x+self.y*self.y+self.z*self.z);
        return magnitude;
    }

}

//Todo: add constructor
impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, _rhs: Vector) -> Vector {
        return Vector{x:self.x+_rhs.x, y:self.y+_rhs.y, z:self.z+_rhs.z};
    }
}

//Todo: add constructor
impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, _rhs: Vector) -> Vector {
        return Vector{x:self.x-_rhs.x, y:self.y-_rhs.y, z:self.z-_rhs.z};
    }
}

//Scalar multiplication of form vector * scalar
//Todo: add constructor
impl ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, _rhs: f32) -> Vector {
        return Vector{x:self.x*_rhs, y:self.y*_rhs, z:self.z*_rhs};
    }
}

//Scalar multiplication of form scalar * vector
//Todo: add constructor
impl ops::Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, _rhs: Vector) -> Vector {
        return Vector{x:self*_rhs.x, y:self*_rhs.y, z:self*_rhs.z};
    }
}

#[derive(Copy, Clone)]
pub struct SphereVector {
    pub r: f32,
    pub theta: f32,
    pub phi: f32,
}

impl SphereVector {
    pub fn trace(&self) {
        println!("<{0}, {1}, {2}>", self.r, self.theta, self.phi);
    }

}

//Scalar multiplication of form vector * scalar
//Todo: add constructor
impl ops::Mul<f32> for SphereVector {
    type Output = SphereVector;

    fn mul(self, _rhs: f32) -> SphereVector {
        return SphereVector{r:self.r*_rhs, theta:self.theta, phi:self.phi};
    }
}

//Scalar multiplication of form scalar * vector
//Todo: add constructor
impl ops::Mul<SphereVector> for f32 {
    type Output = SphereVector;

    fn mul(self, _rhs: SphereVector) -> SphereVector {
        return SphereVector{r:self*_rhs.r, theta:_rhs.theta, phi:_rhs.phi};
    }
}

pub fn test() {
    println!("this is the other file");    
}
