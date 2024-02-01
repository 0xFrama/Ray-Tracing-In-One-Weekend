use crate::vec3::{Point3, Vec3};

pub struct Ray {
    dir: Vec3,
    orig: Point3,
}

impl Ray {
    pub fn ray(origin: Point3, direction: Vec3) -> Self {
        Self { orig: origin, dir: direction }
    }

    pub fn origin(self) -> Point3 {
        return self.orig;
    }

    pub fn direction(self) -> Vec3 {
        return self.dir;
    }

    pub fn at(self, t: f32) -> Point3 {
        return self.orig + t*self.dir;
    }

}