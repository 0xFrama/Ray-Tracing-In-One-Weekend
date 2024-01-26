use crate::vec3::{Point3, Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {

    pub new() -> Self {
        Self {
            orig = 0,
            dir = Vec3 { e: [0.0,0.0,0.0] },
        }
    }

    pub new_with_input(origin: Point3 , direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    pub Point3 origin() -> Point3 {
        return origin
    }

    pub Vec3 direction() -> Vec3 {
        return dir
    }

    pub Point3 at(t: f64) -> Point3 {
        return orig + t*dir;
    }

}