use planar_graph::Ball;
use rand::Rng;
use raylib::prelude::*;
use std::io;
use std::io::*;
use std::thread;
use std::thread::current;

fn main() {
    println!("How many balls would you like? ");
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let num_balls = line
        .trim()
        .parse::<i64>()
        .expect("unable to convert to integer");

    let (mut rl, thread) = raylib::init().size(960, 720).title("Balls").build();
    let mut counter = 1;
    let mut balls: Vec<Ball> = Vec::with_capacity(num_balls as usize);

    let mut rng = rand::rng();
    let screen_w = 960.;
    let screen_h = 720.;

    for _ in 0..num_balls {
        let x = rng.random_range((0.0 + 10.)..(screen_w - 10.));
        let y = rng.random_range((0.0 + 10.)..(screen_h - 10.));
        let v_x: f32 = rng.random_range(-0.5..0.5);
        let v_y: f32 = rng.random_range(-0.5..0.5);

        let color = Color::new(
            rng.random_range(0..=255),
            rng.random_range(0..=255),
            rng.random_range(0..=255),
            rng.random_range(0..=255),
        );
        let mut ball = Ball::new(x, y, 10., color.into(), v_x, v_y);
        balls.push(ball);
    }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHEAT);
        for ball in balls.iter_mut() {
            ball.update(screen_w, screen_h);
            ball.draw(&mut d);
        }
    }
}
