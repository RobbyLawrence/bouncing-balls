use raylib;
use raylib::ffi::Color;
use raylib::prelude::{RaylibDraw, RaylibDrawHandle};

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub color: raylib::ffi::Color,
    pub v_x: f32,
    pub v_y: f32,
}

impl Ball {
    pub fn new(x: f32, y: f32, radius: f32, color: Color, v_x: f32, v_y: f32) -> Self {
        assert!(x > 0.);
        assert!(y > 0.);
        assert!(radius > 0.);
        Ball {
            x,
            y,
            radius,
            color,
            v_x,
            v_y,
        }
    }

    pub fn update(&mut self, screen_w: f32, screen_h: f32) {
        // update position
        self.x += self.v_x;
        self.y += self.v_y;

        // horizontal bouncing
        if self.x - self.radius <= 0.0 || self.x + self.radius >= screen_w {
            self.v_x = -self.v_x;
            // we clamp just in case it skipped a little
            self.x = self.x.clamp(self.radius, screen_w - self.radius);
        }

        if self.y - self.radius <= 0.0 || self.y + self.radius >= screen_h {
            self.v_y = -self.v_y;
            self.y = self.y.clamp(self.radius, screen_h - self.radius);
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle(self.x as i32, self.y as i32, self.radius, self.color);
    }
}
