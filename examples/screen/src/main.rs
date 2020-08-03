extern crate ev3dev_lang_rust;
extern crate image;
extern crate imageproc;

use image::Rgb;

use ev3dev_lang_rust::Screen;

fn interpolate(a: Rgb<u8>, b: Rgb<u8>, progress: f32) -> Rgb<u8> {
    Rgb([
        ((a.0[0] as f32) * (1.0 - progress) + (b.0[0] as f32) * progress) as u8,
        ((a.0[1] as f32) * (1.0 - progress) + (b.0[1] as f32) * progress) as u8,
        ((a.0[2] as f32) * (1.0 - progress) + (b.0[2] as f32) * progress) as u8,
    ])
}

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

    imageproc::drawing::draw_antialiased_line_segment_mut(
        &mut screen.image,
        (10, 50),
        (50, 90),
        Rgb([0, 0, 0]),
        interpolate,
    );

    imageproc::drawing::draw_filled_circle_mut(&mut screen.image, (100, 50), 40, Rgb([0, 0, 255]));

    screen.update();
}
