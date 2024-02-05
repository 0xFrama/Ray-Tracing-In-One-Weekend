mod color;
mod vec3;
mod ray;

use color::Color;
use ray::Ray;
use vec3::{Vec3, Point3, dot};

fn hit_sphere(center: &Point3, radious: f32, r: &Ray) -> bool {
    let oc: Vec3 = r.origin() - center.clone();
    let a = dot(&r.direction(), &r.direction());
    let b = dot(&oc, &r.direction()) * 2.;
    let c = dot(&oc, &oc) - radious*radious;
    let discriminant = b*b - 4_f32*a*c;
    return (discriminant >= 0.);
}

fn ray_color(r: &Ray) -> Color { 
    if (hit_sphere(&Point3::new_with_inputs(0., 0., 1.), 0.5, r)) {
        return Color::new_with_inputs(1., 0., 0.);
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
    let viewport_width= viewport_height * (image_width/image_height) as f32;
    let camera_center = Point3::new_with_inputs(0.,0.,0.);
    
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
            let r = Ray::ray(camera_center, ray_direction);
            //let pixel_color = Color::new_with_inputs((i as f32/(image_width-1) as f32), (j as f32/(image_height-1) as f32), 0.);
            let pixel_color = ray_color(&r);
            Color::write_color(pixel_color);
        }
    }
}