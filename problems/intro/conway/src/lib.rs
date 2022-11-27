#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn size(&self) -> (usize, usize) {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        // TODO: your code goes here.
        unimplemented!()
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
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn step(&mut self) {
        // TODO: your code goes here.
        unimplemented!()
    }
}
