use raylib;
use raylib::ffi::Color;
use raylib::prelude::*;

pub struct Ball {
    x: f32,
    y: f32,
    radius: f32,
    color: Color,
    v_x: i64,
    v_y: i64,
}

impl Ball {
    pub fn new(x: f32, y: f32, radius: f32, color: Color, v_x: i64, v_y: i64) -> Self {
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
}
