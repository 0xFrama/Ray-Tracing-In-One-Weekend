mod color;
mod vec3;
mod ray;
mod hittable;
mod sphere;

use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::{Vec3, Point3, dot};

fn hit_sphere(center: &Point3, radious: f32, r: &Ray) -> f32 {
    let mut oc: Vec3 = r.origin() - center.clone();
    let a = &r.direction().length_squared();
    let half_b = dot(&oc, &r.direction());
    let c = oc.length_squared() - radious*radious;
    let discriminant = half_b*half_b - a*c;

    if discriminant < 0.0 {
        return -1.0
    } else {
        return (-half_b - (discriminant).sqrt() ) / * a
    }
}

fn ray_color(r: &Ray) -> Color { 
    let t: f32 = hit_sphere(&Point3::new_with_inputs(0., 0., -1.), 0.5, r);
    if t > 0.0 {
        let N: Vec3 = Vec3::unit_vector(r.at(t) - Vec3::new_with_inputs(0., 0., -1.));
        return Color::new_with_inputs(N.x()+1., N.y()+1., N.z()+1.) * 0.5;
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.);
    return Color::new_with_inputs(1., 1., 1.) * (1.-a) + Color::new_with_inputs(0.5, 0.7, 1.) * a;
}

fn main() {

    // Image

    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let mut image_height: u32 = ((image_width as f32) / aspect_ratio) as u32;
    image_height = if image_height < 1 { 1 } else { image_height };

    // Camera
    
    let focal_length: f32 = 1.0;
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = viewport_height * (image_width/image_height) as f32;
    let camera_center: Vec3 = Point3::new_with_inputs(0.,0.,0.);
    
    // Calculate the vectors across the horizontal and down the vertical view port edges.
    let viewport_u = Vec3::new_with_inputs(viewport_width, 0., 0.);
    let viewport_v = Vec3::new_with_inputs(0., -viewport_height, 0.);

    // Calculate the horizontal and vertical delta vectos from pixels to pixel.
    let pixel_delta_u = viewport_u / (image_width as f32);
    let pixel_delta_v = viewport_v / (image_height as f32);

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center - Vec3::new_with_inputs(0., 0., focal_length) - viewport_u/2. - viewport_v/2.;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;


    // Render

    println!("P3\n{} {}\n255", image_width, image_height);
 
    for j in 0..image_height {
        eprintln!("Scanlines remaining: {} ", (image_height-j));
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (pixel_delta_u * (i as f32)) + (pixel_delta_v * (j as f32));
            let ray_direction = pixel_center - camera_center;
            let r: Ray = Ray::ray(camera_center, ray_direction);
            //let pixel_color = Color::new_with_inputs((i as f32/(image_width-1) as f32), (j as f32/(image_height-1) as f32), 0.);
            let pixel_color = ray_color(&r);
            Color::write_color(pixel_color);
        }
    }
}