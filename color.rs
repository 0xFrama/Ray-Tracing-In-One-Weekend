

use std::io::Write;
use crate::vec3::Vec3;

pub fn write_color<T: Write>(mut out: T, pixel_color: Vec3) -> std::io::Result<()> {
    writeln!(
        out,
        "{} {} {}",
        (255.999 * pixel_color.x()) as i32,
        (255.999 * pixel_color.y()) as i32,
        (255.999 * pixel_color.z()) as i32,
    )?;
    Ok(())
}