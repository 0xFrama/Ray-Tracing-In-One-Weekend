fn main() {

    // Image

    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    // Render

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
 
    for j in 0..IMAGE_HEIGHT {
        eprintln!("Scanlines remaining: {} ", (IMAGE_HEIGHT-j));
        for i in 0..IMAGE_WIDTH {
            let r = (i as f32) / ((IMAGE_WIDTH-1) as f32);
            let g = (j as f32) / ((IMAGE_HEIGHT-1) as f32);
            let b = 0 as f32;

            let ir: u32 = (255.999 * r) as u32;
            let ig: u32 = (255.999 * g) as u32;
            let ib: u32 = (255.999 * b) as u32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}