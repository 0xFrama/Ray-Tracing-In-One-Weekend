use crate::vec3::Point3;

#[derive(Copy, Clone)]
pub struct Ray {
    dir: Point3,
    orig: Point3,
}

impl Ray {
    pub fn ray(orig: Point3, dir: Point3) -> Ray { Ray { orig, dir } }

    pub fn origin(&self) -> Point3 { self.orig }

    pub fn direction(&self) -> Point3 { self.dir }

    pub fn at(&self, t: f32) -> Point3 { self.orig + self.dir*t }

}