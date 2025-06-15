use planar_graph::Ball;
use rand::Rng;
use raylib::prelude::*;
use std::io;

fn main() {
    // 1) How many balls?
    print!("How many balls would you like?\n");
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let num_balls = line.trim().parse::<usize>().expect("positive integer");

    // 2) Raylib setup
    let (mut rl, thread) = raylib::init()
        .size(960, 720)
        .title("Bouncing Balls")
        .build();
    let screen_w = 960.0;
    let screen_h = 720.0;
    let radius = 15.;
    // 3) Make random balls
    let mut rng = rand::thread_rng();
    let mut balls: Vec<Ball> = (0..num_balls)
        .map(|_| {
            let x = rng.gen_range(radius..(screen_w - radius));
            let y = rng.gen_range(radius..(screen_h - radius));
            let vx = rng.gen_range(-0.15..0.15);
            let vy = rng.gen_range(-0.15..0.15);
            let c = Color::WHITESMOKE;
            Ball::new(x, y, radius, c, vx, vy)
        })
        .collect();

    // 4) Main loop
    while !rl.window_should_close() {
        // A) **Collide first** (resolve impulses at current positions)
        let n = balls.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let (left, right) = balls.split_at_mut(j);
                Ball::collide(&mut left[i], &mut right[0]);
            }
        }

        // B) **Then step** (move + wall bounce)
        for ball in balls.iter_mut() {
            ball.update(screen_w, screen_h);
        }

        // C) Draw
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        for ball in balls.iter() {
            ball.draw(&mut d);
        }
    }
}
