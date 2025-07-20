use crate::framebuffer::Framebuffer;
use raylib::prelude::*;

pub type Grid = Vec<Vec<bool>>;

pub fn create_empty_grid(width: usize, height: usize) -> Grid {
    vec![vec![false; width]; height]
}

pub fn count_neighbors(grid: &Grid, x: usize, y: usize) -> u8 {
    let width = grid[0].len();
    let height = grid.len();
    let mut count = 0;

    for dy in [-1isize, 0, 1] {
        for dx in [-1isize, 0, 1] {
            if dx == 0 && dy == 0 { continue; }
            let nx = (x as isize + dx + width as isize) % width as isize;
            let ny = (y as isize + dy + height as isize) % height as isize;
            if grid[ny as usize][nx as usize] {
                count += 1;
            }
        }
    }
    count
}

pub fn render_life(current: &Grid, next: &mut Grid, fb: &mut Framebuffer) {
    for y in 0..current.len() {
        for x in 0..current[0].len() {
            let alive = current[y][x];
            let neighbors = count_neighbors(current, x, y);

            let new_state = match (alive, neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };

            next[y][x] = new_state;

            fb.set_current_color(if new_state { Color::WHITE } else { Color::BLACK });
            fb.set_pixel(x as u32, y as u32);
        }
    }
}
