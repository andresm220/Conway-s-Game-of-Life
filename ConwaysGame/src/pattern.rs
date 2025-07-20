use std::collections::HashSet;
pub type Cell = (i32, i32);

pub fn glider(x: i32, y: i32) -> HashSet<Cell> {
    HashSet::from([
        (x + 1, y),
        (x + 2, y + 1),
        (x, y + 2), (x + 1, y + 2), (x + 2, y + 2),
    ])
}

pub fn blinker(x: i32, y: i32) -> HashSet<Cell> {
    HashSet::from([
        (x, y), (x + 1, y), (x + 2, y),
    ])
}

pub fn toad(x: i32, y: i32) -> HashSet<Cell> {
    HashSet::from([
        (x + 1, y),
        (x + 2, y),
        (x + 3, y),
        (x, y + 1),
        (x + 1, y + 1),
        (x + 2, y + 1),
    ])
}
