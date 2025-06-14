use planar_graph::Ball;
use rand::Rng;
use raylib::prelude::*;
use std::io;
use std::io::*;
use std::thread;

fn main() {
    print!("How many balls would you like? ");
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let num_balls = line.trim().parse::<i64>().expect("unable to convert to integer");

    let (mut rl, thread) = raylib::init().size(960, 720).title("Hello World").build();
    let mut counter = 1;
    let mut balls: Vec<Ball> = Vec::with_capacity(num_balls as usize);

    for _ in 0..

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHEAT);
        d.draw_circle(coordinate, coordinate, 100., Color::BLUEVIOLET);

        if counter % ((1. / velocity) as i64) == 0 {
            coordinate += 1;
        }
        counter += 1;
    }
}
