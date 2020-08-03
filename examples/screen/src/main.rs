extern crate ev3dev_lang_rust;
extern crate framebuffer;
extern crate image;

use image::Rgb;

use ev3dev_lang_rust::Screen;

fn main() {
    let mut screen = Screen::new().unwrap();

    for x in 10..20 {
        for y in 10..20 {
            screen.image.put_pixel(x, y, Rgb([0, 0, 0]));
        }
    }
    for x in 30..40 {
        for y in 10..20 {
            screen.image.put_pixel(x, y, Rgb([255, 0, 0]));
        }
    }

    for x in 10..20 {
        for y in 30..40 {
            screen.image.put_pixel(x, y, Rgb([0, 255, 0]));
        }
    }
    for x in 30..40 {
        for y in 30..40 {
            screen.image.put_pixel(x, y, Rgb([0, 0, 255]));
        }
    }

    screen.update();
}
