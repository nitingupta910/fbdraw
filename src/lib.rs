//! # fbdraw
//!
//! Provides a simple interface for creating a "surface" and a single primitive
//! `put_pixel` for drawing on it.
//!
//! The aim is allow playing around with graphics algorithms like curve drawing,
//! without the burden of setting up a window (with an event loop), setting up
//! GPU pipeline, etc.
//!
//! This crate wraps the [minifb](https://docs.rs/minifb/latest/minifb/) library,
//! and provides a simpler `Surface` based interface.
//!
//! The coordinate system has origin at the top-left, with X and Y axis
//! running left-to-right and top-to-bottom, respectively.
//!
//! ## Example
//! ```
//! use fbdraw::{Color, Surface};
//!
//! let mut surface = Surface::new(1920, 1200);
//!
//! surface.begin_draw(my_draw_frame);
//!
//! // Draw a frame on the surface. This callback function is
//! // called at a fixed rate of 60 fps.
//! fn my_draw_frame(surface: &mut Surface) {
//!     let (width, height) = surface.size();
//!     surface.put_pixel(width / 2, height / 2, Color::rgb(255, 0, 0));
//! }
//! ```

use minifb::{Key, Window, WindowOptions};
use std::cmp::min;

pub struct Color {
    val: u32,
}

impl Color {
    /// Returns a color value constructed from the given RGB components.
    /// Individual color component values are clamped to 255.
    pub fn rgb(r: u32, b: u32, g: u32) -> Color {
        Color {
            val: (min(r, 255) << 16) | (min(g, 255) << 8) | min(b, 255),
        }
    }
}

pub struct Surface {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl Surface {
    /// Returns size of the surface as (width, height) tuple
    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Puts a pixel on the surface
    ///
    /// This function is supposed to be called from the draw
    /// callback as passed to the `begin_draw` function.
    ///
    /// Coordinate system: Origin (0, 0) is at top left, with (X, Y)
    /// axis going left-to-right, top-to-bottom.
    ///
    /// # Arguments
    ///
    /// * `x` - X-coordinate
    /// * `y` - Y-coordinate
    /// * `color` - Color of the pixel
    pub fn put_pixel(&mut self, x: usize, y: usize, color: Color) {
        let x_clamp = min(x, self.width - 1);
        let y_clamp = min(y, self.height - 1);
        self.buffer[y_clamp * self.width + x_clamp] = color.val;
    }

    /// Begins drawing on the surface.
    ///
    /// # Arguments
    /// * `draw_frame` - This function is called to draw
    ///    a single frame at a time. The callback is called
    ///    at a fixed rate of 60 fps.
    ///
    /// # Examples
    ///
    /// ```
    /// use fbdraw::{Color, Surface};
    ///
    /// let mut surface = Surface::new(1920, 1200);
    ///
    /// surface.begin_draw(my_draw_frame);
    ///
    /// // Draw a frame on the surface. This function is
    /// // called at a fixed rate of 60 fps.
    /// fn my_draw_frame(surface: &mut Surface) {
    ///     let (width, height) = surface.size();
    ///     surface.put_pixel(width / 2, height / 2, Color::rgb(255, 0, 0));
    /// }
    /// ```
    pub fn begin_draw<F>(&mut self, mut draw_frame: F)
    where
        F: FnMut(&mut Self),
    {
        let mut window = Window::new(
            "fbdraw - ESC to exit",
            self.width,
            self.height,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        while window.is_open() && !window.is_key_down(Key::Escape) {
            draw_frame(self);
            window
                .update_with_buffer(&self.buffer.as_slice(), self.width, self.height)
                .unwrap();
        }
    }

    /// Creates a new surface of the given size
    ///
    /// # Arguments
    /// * `width` - Width of the surface in pixels
    /// * `height` - Height of the surface in pixels
    pub fn new(width: usize, height: usize) -> Surface {
        let buffer = vec![0; width * height];
        Surface {
            width,
            height,
            buffer,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Color, Surface};

    #[test]
    fn create_surface() {
        let _ = Surface::new(1920, 1200);
    }

    #[test]
    fn draw_on_surface() {
        let mut surface = Surface::new(1920, 1200);
        surface.begin_draw(draw_centered_cross);
    }

    fn draw_centered_cross(surface: &mut Surface) {
        let y = surface.height / 2;
        for x in (surface.width / 4)..=(surface.width * 3 / 4) {
            surface.put_pixel(x, y, Color::rgb(255, 0, 0));
        }

        let x = surface.width / 2;
        for y in (surface.height / 4)..=(surface.height * 3 / 4) {
            surface.put_pixel(x, y, Color::rgb(0, 255, 0));
        }
    }
}
