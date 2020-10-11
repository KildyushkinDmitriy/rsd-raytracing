mod raytracing;

extern crate clap;
extern crate image;

use crate::raytracing::*;
use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0", about = "RTX on CPU!")]
struct Opts {
    #[clap(long)]
    image_lib_format: Option<String>,
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.image_lib_format {
        Some(img_format) => {
            println!("writing image in {} format", img_format);
            write_image(&img_format)
        }
        None => {
            print_ppm_image();
        }
    }
}
