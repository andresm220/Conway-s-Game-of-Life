#  Conway's Game of Life – Rust + Raylib

This project is a real-time implementation of **Conway's Game of Life**, developed in **Rust** using the [Raylib](https://www.raylib.com/) graphics library.

##  What is Conway's Game of Life?

Conway's Game of Life is a **cellular automaton** devised by mathematician John Conway. Each cell can be either alive or dead, and evolves according to a set of simple rules based on its neighbors.

Rules (applied every frame):
- A live cell with fewer than 2 live neighbors dies (underpopulation)
- A live cell with 2 or 3 live neighbors survives
- A live cell with more than 3 live neighbors dies (overpopulation)
- A dead cell with exactly 3 live neighbors becomes alive (reproduction)

## Features

- Custom framebuffer built with Raylib
- Real-time visual simulation
- Random generation of classic patterns such as:
  - Gliders
  - Blinkers
  - Toads
- Creative and scattered initial population
- No framebuffer clearing — the game evolves visually over time

##  Preview
<p align="center">
  <img src="demo.gif" alt="Game of Life demo" width="400"/>
</p>





##  Requirements

- [Rust](https://www.rust-lang.org/tools/install)


