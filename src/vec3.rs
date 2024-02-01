use std::ops::{Add, Sub, Mul, Div, AddAssign, MulAssign, Neg, Index, IndexMut};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    e: [f32; 3]
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
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        &mut self.e[i]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.e[0] *= other;
        self.e[1] *= other;
        self.e[2] *= other;
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


impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 {
        Vec3 {
            e: [t * self.e[0], t * self.e[1], t * self.e[2]]
        }
    }
}


impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Vec3 {
        return Vec3 {
            e: [(1_f32/t) * self.e[0], (1_f32/t) * self.e[1], (1_f32/t) * self.e[2]]
        }
    }
}


impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 { e: [0., 0., 0.] }
    }

    pub fn new_with_inputs(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn show(self) {
        println!("[ {}, {}, {} ]", self.e[0], self.e[1], self.e[2]);
    }

    pub fn x(&self) -> f32 {
        return self.e[0];
    }

    pub fn y(&self) -> f32 {
        return self.e[1];
    }

    pub fn z(&self) -> f32 {
        return self.e[2];
    }

    pub fn length(&self) -> f32 {
        return self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        return self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2] 
    }
}


fn dot(u: &Vec3, v: &Vec3) -> f32 {
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