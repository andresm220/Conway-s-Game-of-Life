use raylib::prelude::*;
use std::collections::{HashMap, HashSet};

mod framebuffer;
mod pattern;
mod population;

use framebuffer::Framebuffer;
use population::create_random_population;

type Cell = (i32, i32);
const WIDTH: i32 = 100;
const HEIGHT: i32 = 100;

fn neighbors(x: i32, y: i32) -> Vec<Cell> {
    vec![
        (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
        (x - 1, y),               (x + 1, y),
        (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
    ]
}

fn step(alive: &HashSet<Cell>) -> HashSet<Cell> {
    let mut neighbor_count: HashMap<Cell, u8> = HashMap::new();

    for &(x, y) in alive.iter() {
        for n in neighbors(x, y) {
            *neighbor_count.entry(n).or_insert(0) += 1;
        }
    }

    let mut new_alive = HashSet::new();
    for (cell, count) in neighbor_count {
        if count == 3 || (count == 2 && alive.contains(&cell)) {
            new_alive.insert(cell);
        }
    }

    new_alive
}

fn render_life(cells: &HashSet<Cell>, fb: &mut Framebuffer) {
    fb.set_current_color(Color::BLACK);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            fb.set_pixel(x as u32, y as u32);
        }
    }

    for &(x, y) in cells {
        if x >= 0 && y >= 0 && x < WIDTH && y < HEIGHT {
            fb.set_current_color(Color::YELLOW);
            fb.set_pixel(x as u32, y as u32);
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Conway's Game of Life")
        .build();

    let mut fb = Framebuffer::new(WIDTH as u32, HEIGHT as u32);
    let mut alive = create_random_population(WIDTH, HEIGHT);

    while !rl.window_should_close() {
        render_life(&alive, &mut fb);
        fb.swap_buffers(&mut rl, &thread);

        alive = step(&alive);

        std::thread::sleep(std::time::Duration::from_millis(120));
    }
}
