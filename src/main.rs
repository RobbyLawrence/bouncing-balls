use bouncing_balls::Ball;
use rand::Rng;
use raylib::prelude::*;
use std::io;

fn main() {
    print!("How many balls would you like?\n");
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let num_balls = line.trim().parse::<usize>().expect("positive integer");

    let (mut rl, thread) = raylib::init()
        .size(960, 720)
        .title("Bouncing Balls")
        .build();
    let screen_w = 960.0;
    let screen_h = 720.0;
    let radius = 30.;

    let mut rng = rand::rng();
    let mut balls: Vec<Ball> = (0..num_balls)
        .map(|_| {
            let x = rng.random_range(radius..(screen_w - radius));
            let y = rng.random_range(radius..(screen_h - radius));
            let vx = rng.random_range(-0.15..0.15);
            let vy = rng.random_range(-0.15..0.15);
            let c = Color::WHITESMOKE;
            Ball::new(x, y, radius, c, vx, vy)
        })
        .collect();

    while !rl.window_should_close() {
        // collision handling first
        let n = balls.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let (left, right) = balls.split_at_mut(j);
                Ball::collide(&mut left[i], &mut right[0]);
            }
        }
        // then we can move the balls
        for ball in balls.iter_mut() {
            ball.update(screen_w, screen_h);
        }
        // then we update colors
        for ball in balls.clone().iter() {
            ball.update_colors(&mut balls);
        }
        // then we can draw the balls
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        for ball in balls.iter() {
            ball.draw(&mut d);
        }
    }
}
