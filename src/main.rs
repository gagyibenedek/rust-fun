//i sierpinki triangle img gen

extern crate image;
extern crate rand;

//use rand::Rng;
use std::fs::File;
use std::path::Path;

use image::GenericImage;
use image::Pixel;

///points used to build triangles
// pub struct Point {
//     x: u32,
//     y: u32,
// }

///main program
pub fn main() {
    //tri(800, 600);
    gray("in.jpg", "out.png");
}

fn gray(file_in: &'static str, file_out: &'static str) {
    let img = image::open(&Path::new(file_in)).unwrap();
    let mut out = image::ImageBuffer::new(img.dimensions().0, img.dimensions().1);

    for (px, py, pixel) in img.pixels() {
        out.put_pixel(px, py, pixel.to_luma());
    }

    let ref mut fout = File::create(&Path::new(file_out)).unwrap();

    // Write the contents of this image to the Writer in PNG format.
    let _ = image::ImageLuma8(out).save(fout, image::PNG);
}
// fn tri(width:u32, height:u32){
//         let mut img = image::ImageBuffer::from_fn(width, height, |x, y|{
//         if x == 0 && y == 0 {
//             image::Luma([0u8])
//         } else {
//             image::Luma([255u8])
//         }
//     });

//     let mut cnt: u32 = 1_000_000;

//     let pts: [Point; 3] = [
//         Point {x: width/2, y: 0},
//         Point {x: 0, y: height},
//         Point {x: width, y: height},
//     ];
//     let mut num: usize;

//     let mut p = Point {x: 350, y:350};
//     let pixel = img[(0, 0)];

//     while cnt > 0 {
//         cnt -= 1;
//         num = rand::thread_rng().gen_range(0, 3);
//         p.x = (p.x + pts[num].x) / 2;
//         p.y = (p.y + pts[num].y) / 2;
//         img.put_pixel(p.x, p.y, pixel);
//     }

//     let ref mut fout = File::create(&Path::new("tri.png")).unwrap();
//     let _ = image::ImageLuma8(img).save(fout, image::PNG);
// }
