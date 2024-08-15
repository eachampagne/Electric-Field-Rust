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

pub fn test() {
    println!("this is the other file");    
}
