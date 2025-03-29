use image::Luma;
use std::io::Write;

fn main() {
    let img = image::open("sample.jpg")
        .expect("Failed read picture data!")
        .to_luma8();
    let (width, height) = img.dimensions();

    let mut file = std::fs::File::create("ascii_image.txt").expect("Failed create File!");

    let aspect_ratio = 2;
    for y in (0..height).step_by(aspect_ratio) {
        for x in 0..width {
            let Luma([luma]) = img.get_pixel(x, y);

            let character = if *luma > 127 { " " } else { "#" };

            write!(file, "{}{}", character, character).expect("Failed write File!");
        }
        write!(file, "\n").expect("Failed write File!");
    }
    println!("Success printe ascii_image file!");
}
