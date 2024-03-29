use crate::vec3;

pub type Color = vec3::Vec3;

impl Color {
        pub fn write_color(pixel_color: Color) {
                println!("{} {} {}",    (255.999 * pixel_color.x()) as u32, 
                                        (255.999 * pixel_color.y()) as u32, 
                                        (255.999 * pixel_color.z()) as u32)
        }
}