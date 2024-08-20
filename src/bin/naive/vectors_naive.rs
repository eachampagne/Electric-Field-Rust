use std::ops;

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
        let x:f32 = self.x + _rhs.x; 
        let y:f32 = self.y + _rhs.y;
        let z:f32 = self.z + _rhs.z;
        return Vector{x:x, y:y, z:z};
    }
}

//Todo: add constructor
impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, _rhs: Vector) -> Vector {
        let x:f32 = self.x-_rhs.x;
        let y:f32 = self.y-_rhs.y;
        let z:f32 = self.z-_rhs.z;
        return Vector{x:x, y:y, z:z};
    }
}

//Scalar multiplication of form vector * scalar
//Todo: add constructor
impl ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, _rhs: f32) -> Vector {
        let x:f32 = self.x*_rhs;
        let y:f32 = self.y*_rhs;
        let z:f32 = self.z*_rhs;
        return Vector{x:x, y:y, z:z};
    }
}

//Scalar multiplication of form scalar * vector
//Todo: add constructor
impl ops::Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, _rhs: Vector) -> Vector {
        let x:f32 = self*_rhs.x;
        let y:f32 = self*_rhs.y;
        let z:f32 = self*_rhs.z;
        return Vector{x:x, y:y, z:z};
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
        let newR:f32 = self.r*_rhs;
        return SphereVector{r:newR, theta:self.theta, phi:self.phi};
    }
}

//Scalar multiplication of form scalar * vector
//Todo: add constructor
impl ops::Mul<SphereVector> for f32 {
    type Output = SphereVector;

    fn mul(self, _rhs: SphereVector) -> SphereVector {
        let newR:f32 = self*_rhs.r;
        return SphereVector{r:newR, theta:_rhs.theta, phi:_rhs.phi};
    }
}

pub fn test() {
    println!("this is the other file");    
}
