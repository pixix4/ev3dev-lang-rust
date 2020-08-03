#[cfg(feature = "screen")]
use framebuffer::Framebuffer;

#[cfg(feature = "screen")]
use image::{Rgb, RgbImage};

use crate::Ev3Result;

/// Represents the device screen.
#[cfg(feature = "screen")]
#[derive(Debug)]
pub struct Screen {
    /// Direct reference to the framebuffer
    pub buffer: Framebuffer,
    /// Convinience layer to access the framebuffer
    pub image: RgbImage,
}

#[cfg(feature = "screen")]
impl Screen {
    /// Create a reference to the device screen
    pub fn new() -> Ev3Result<Self> {
        let buffer = Framebuffer::new("/dev/fb0")?;

        let image = RgbImage::from_pixel(
            buffer.fix_screen_info.line_length * 8 / buffer.var_screen_info.bits_per_pixel,
            buffer.var_screen_info.yres,
            Rgb([255, 255, 255]),
        );

        Ok(Self { buffer, image })
    }

    /// Horizontal screen resolution
    pub fn xres(&self) -> u32 {
        self.buffer.var_screen_info.xres
    }

    /// Vertical screen resolution
    pub fn yres(&self) -> u32 {
        self.buffer.var_screen_info.yres
    }

    /// Dimensions of the screen.
    pub fn shape(&self) -> (u32, u32) {
        (self.xres(), self.yres())
    }

    /// Clears the screen
    pub fn clear(&mut self) {
        for (_, _, pixel) in self.image.enumerate_pixels_mut() {
            *pixel = Rgb([255, 255, 255]);
        }
    }

    fn update_1bpp(&mut self) {
        panic!("1bpp screens are currently not supported!");
    }

    /// Convert red, green, blue components to a 16-bit 565 RGB value. Components
    /// should be values 0 to 255.
    fn color565(r: u8, g: u8, b: u8) -> (u8, u8) {
        let c = (((r as u16) & 0xF8) << 8) | (((g as u16) & 0xFC) << 3) | ((b as u16) >> 3);
        ((c >> 8) as u8, c as u8)
    }

    fn update_16bpp(&mut self) {
        let mut buffer = vec![0u8; (2 * self.xres() * self.yres()) as usize];

        for (index, (_, _, pixel)) in self.image.enumerate_pixels().enumerate() {
            let i = index * 2;
            let (p1, p2) = Screen::color565(pixel.0[0], pixel.0[1], pixel.0[2]);
            buffer[i] = p1;
            buffer[i + 1] = p2;
        }

        self.buffer.write_frame(&buffer);
    }

    fn update_32bpp(&mut self) {
        let mut buffer = vec![0u8; (4 * self.xres() * self.yres()) as usize];

        for (index, (_, _, pixel)) in self.image.enumerate_pixels().enumerate() {
            let i = index * 4;
            buffer[(i + 1)..(i + 3)].copy_from_slice(&pixel.0[0..2]);
        }

        self.buffer.write_frame(&buffer);
    }

    /// Applies pending changes to the screen.
    /// Nothing will be drawn on the screen until this function is called.
    pub fn update(&mut self) {
        if self.buffer.var_screen_info.bits_per_pixel == 1 {
            self.update_1bpp();
        } else if self.buffer.var_screen_info.bits_per_pixel == 16 {
            self.update_16bpp();
        } else if self.buffer.var_screen_info.bits_per_pixel == 32 {
            self.update_32bpp();
        }
    }
}
