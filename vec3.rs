
use std::ops::Neg;
use std::fmt;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Index;
use std::ops::MulAssign;
use std::ops::AddAssign;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    e: [f64; 3]
}


impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.e[0],-self.e[1],-self.e[2]],
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.e[0] *= other;
        self.e[1] *= other;
        self.e[2] *= other;
    }
}


impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}


impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]]
        }
    }
}


impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]]
        }
    }
} 


impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]]
        }
    }
}


impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            e: [t * self.e[0], t * self.e[1], t * self.e[2]]
        }
    }
}


impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        return Vec3 {
            e: [(1_f64/t) * self.e[0], (1_f64/t) * self.e[1], (1_f64/t) * self.e[2]]
        }
    }
}


impl Vec3 {
    fn new() -> Vec3 {
        Vec3 { e: [0.0,0.0,0.0] }
    }

    pub fn new_with_inputs(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    fn from_values(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2]}
    }

    pub fn x(&self) -> f64 {
        return self.e[0];
    }

    pub fn y(&self) -> f64 {
        return self.e[1];
    }

    pub fn z(&self) -> f64 {
        return self.e[2];
    }

    fn length(&self) -> f64 {
        return self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f64 {
        return self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2] 
    }
}


fn dot(u: &Vec3, v: &Vec3) -> f64 {
    return u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}


fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    return Vec3::new_with_inputs(  u.y() * v.z() - u.z() * v.y(),
                            u.z() * v.x() - u.x() * v.z(),
                            u.x() * v.y() - u.y() * v.x())
}


fn unit_vector(v: Vec3) -> Vec3 {
    return v / v.length();
}

pub type Point3 = Vec3;
pub type Color = Vec3;