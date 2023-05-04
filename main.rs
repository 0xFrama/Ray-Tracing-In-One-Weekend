mod color;
mod vec3;
mod ray;

use std::io::{self, Write};
use crate::vec3::Color;
use crate::color::write_color;
use crate::ray;


Color ray_color(r: &Ray) -> Color {
    Vec3 unit_direction = unit_vector(r.direction());
    let t = 0.5*(unit_direction.y() + 1.0);
    return (1.0-t)*color(1.0, 1.0, 1.0) + t*color(0.5, 0.7, 1.0); 
}


fn main() -> io::Result<()> {
    // Image
    const ASPECT_RATION: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATION) as i32;
    
    // Camera
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = ASPECT_RATION * viewport_height;
    let focal_lenght: f64 = 1.0;

    let origin: Point3 = Point3(0,0,0);
    let horizontal: Vec3 = Vec3::new_with_inputs(viewport_width, 0, 0);
    let vertical: Vec3 = Vec3::new_with_inputs(0,viewport_height, 0);

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);
        io::stdout().flush()?;
        for i in 0..IMAGE_WIDTH {
            let r = f64::from(i) / f64::from(IMAGE_WIDTH - 1);
            let g = f64::from(j) / f64::from(IMAGE_HEIGHT - 1);
            let b = 0.25_f64;
            let pixel_color: Color = Color::new_with_inputs(r, g, b);
            write_color(&mut io::stdout(), pixel_color)?;
        }
    }

    eprintln!("\nDone");
    Ok(())
}