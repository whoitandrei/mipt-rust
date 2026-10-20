#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

use crate::Cell::{Alive, Dead};

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Grid { rows, cols, grid: vec![T::default(); rows * cols] }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        assert!(grid.len() == rows * cols);
        Grid { rows, cols, grid: Vec::from(grid) }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        assert!(row < self.rows && col < self.cols);
        &self.grid[row * self.cols + col]
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        assert!(row < self.rows && col < self.cols);
        self.grid[row * self.cols + col] = value;
    }

    pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        for d_row in -1isize..=1 {
            for d_col in -1isize..=1 {
                if d_row == 0 && d_col == 0 {
                    continue;
                }
                let r = row as isize + d_row;
                let c = col as isize + d_col;
                if r >= 0 && r < self.rows as isize && c >= 0 && c < self.cols as isize {
                    res.push((r as usize, c as usize));
                }
            }
        }

        res
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dead
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        GameOfLife { grid }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        &self.grid
    }

    pub fn step(&mut self) {
        let (rows, cols) = self.grid.size();
        let mut new_grid = Grid::<Cell>::new(rows, cols);

        for row in 0..rows {
            for col in 0..cols {
                let mut alive = 0;
                for (r ,c) in self.grid.neighbours(row, col) {
                    if *self.grid.get(r, c) == Alive {
                        alive = alive + 1;
                    }
                }

                let next = match(self.grid.get(row, col), alive) {
                    (Alive, 2) | (Alive, 3) => Alive, 
                    (Dead, 3) => Alive, 
                    _ => Dead,
                };
                new_grid.set(next, row, col);
            }
        }

        self.grid = new_grid;
    }
}
