use crate::vec3::{Point3, dot};
use crate::ray::Ray;
use crate::hittable::{Hitable, HitRecord};

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    fn new(center: Point3, radius: f32) -> Sphere { Sphere { center, radius } }

    fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32, rec: &mut HitRecord) -> bool {
        let mut oc: Point3 = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;
    
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = (discriminant).sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (-half_b + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            } 
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;

        return true;
    }

}