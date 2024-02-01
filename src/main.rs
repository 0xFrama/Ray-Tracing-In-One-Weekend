mod color;
mod vec3;

fn main() {

    // Image

    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    // Render

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
 
    for j in 0..IMAGE_HEIGHT {
        eprintln!("Scanlines remaining: {} ", (IMAGE_HEIGHT-j));
        for i in 0..IMAGE_WIDTH {
            let pixel_color = vec3::Vec3::new_with_inputs((i as f32/(IMAGE_WIDTH-1) as f32), (j as f32/(IMAGE_HEIGHT-1) as f32), 0.);
            color::write_color(pixel_color);
        }
    }
}