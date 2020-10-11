fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    for y in (0..image_height).rev() {
        for x in 0..image_width {
            let r = (x as f64) / ((image_width - 1) as f64);
            let g = (y as f64) / ((image_height - 1) as f64);
            let b = 0.25;

            let multiplier = 255.999;
            let ir = (multiplier * r) as i32;
            let ig = (multiplier * g) as i32;
            let ib = (multiplier * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
