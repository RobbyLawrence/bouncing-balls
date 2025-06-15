use raylib::prelude::*;

#[derive(Debug)]
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub color: Color,
    pub v_x: f32,
    pub v_y: f32,
}

impl Ball {
    /// Create a new ball.
    pub fn new(x: f32, y: f32, radius: f32, color: Color, v_x: f32, v_y: f32) -> Self {
        Self {
            x,
            y,
            radius,
            color,
            v_x,
            v_y,
        }
    }

    /// Handle an elastic collision between two equal-mass balls.
    ///
    /// This only modifies velocities, then nudges apart.
    pub fn collide(a: &mut Ball, b: &mut Ball) {
        let dx = b.x - a.x;
        let dy = b.y - a.y;
        let dist_sq = dx * dx + dy * dy;
        let min_dist = a.radius + b.radius;

        if dist_sq < min_dist * min_dist {
            let dist = dist_sq.sqrt();
            let nx = dx / dist;
            let ny = dy / dist;

            // relative velocity along normal
            let rvx = a.v_x - b.v_x;
            let rvy = a.v_y - b.v_y;
            let vel_along_norm = rvx * nx + rvy * ny;

            // **Apply impulse BEFORE movement**, whenever they're overlapping
            // (we no longer skip when vel_along_norm > 0)
            let j = -vel_along_norm;
            a.v_x += j * nx;
            a.v_y += j * ny;
            b.v_x -= j * nx;
            b.v_y -= j * ny;

            // Now separate them so they’re exactly just touching
            let overlap = min_dist - dist;
            let shift_x = (overlap / 2.0) * nx;
            let shift_y = (overlap / 2.0) * ny;
            a.x -= shift_x;
            a.y -= shift_y;
            b.x += shift_x;
            b.y += shift_y;
        }
    }

    /// Move the ball by its velocity and bounce off walls.
    pub fn update(&mut self, screen_w: f32, screen_h: f32) {
        // Move
        self.x += self.v_x;
        self.y += self.v_y;

        // Walls
        if self.x - self.radius <= 0.0 || self.x + self.radius >= screen_w {
            self.v_x = -self.v_x;
            self.x = self.x.clamp(self.radius, screen_w - self.radius);
        }
        if self.y - self.radius <= 0.0 || self.y + self.radius >= screen_h {
            self.v_y = -self.v_y;
            self.y = self.y.clamp(self.radius, screen_h - self.radius);
        }
    }

    /// Draw the ball.
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle(self.x as i32, self.y as i32, self.radius, self.color);
    }
}
