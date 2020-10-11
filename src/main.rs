extern crate clap;
extern crate image;

use image::{ImageBuffer, ImageResult, RgbImage};
use std::io::Write;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() -> ImageResult<()> {
    let args = clap::App::new("Rust saber div Raytracing")
        .version("1.0")
        .about("RTX on CPU!")
        .arg("--image-lib-format=[FORMAT] 'use image lib with chosen image format'")
        .get_matches();

    match args.value_of("image-lib-format") {
        Some(img_format) => {
            println!("writing image in {} format", img_format);
            write_image(img_format)
        }
        None => {
            print_ppm_image();
            Ok(())
        }
    }
}

fn print_ppm_image() {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for y in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {}", y);

        for x in 0..IMAGE_WIDTH {
            let r = (x as f64) / ((IMAGE_WIDTH - 1) as f64);
            let g = (y as f64) / ((IMAGE_HEIGHT - 1) as f64);
            let b = 0.25;

            let multiplier = 255.999;
            let ir = (multiplier * r) as i32;
            let ig = (multiplier * g) as i32;
            let ib = (multiplier * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("\rDone");
}

fn write_image(img_extension: &str) -> ImageResult<()> {
    let img: RgbImage = ImageBuffer::from_fn(IMAGE_WIDTH, IMAGE_HEIGHT, |x, y| {
        let r = (x as f64) / ((IMAGE_WIDTH - 1) as f64);
        let g = ((IMAGE_HEIGHT - y) as f64) / ((IMAGE_HEIGHT - 1) as f64);
        let b = 0.25;

        let multiplier = 255.999;
        let ir = (multiplier * r) as u8;
        let ig = (multiplier * g) as u8;
        let ib = (multiplier * b) as u8;

        image::Rgb([ir, ig, ib])
    });

    img.save(format!("img.{}", img_extension))
}
