use crate::vec3::Point3;
use crate::ray::Ray;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Point3,
    pub t: f32,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32, rec: &mut HitRecord) -> bool;
}
