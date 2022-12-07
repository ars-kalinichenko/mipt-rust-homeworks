#![forbid(unsafe_code)]

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Grid {
            rows,
            cols,
            grid: Vec::new(),
        }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        Grid {
            rows,
            cols,
            grid: Vec::from(grid),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows as usize, self.cols as usize)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        let index = row * self.size().1 + col;
        self.grid.get(index).unwrap()
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        let index = row * self.size().1 + col;
        self.grid[index] = value;
    }

    fn is_ok_indexes(&self, i: i32, j: i32, row: i32, col: i32) -> bool {
        i >= 0 && j >= 0 && i < self.rows as i32 && j < self.cols as i32 && (i != row || j != col)
    }

    pub fn neighbours(&self, row: i32, col: i32) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for i in row - 1..=row + 1 {
            for j in col - 1..=col + 1 {
                if self.is_ok_indexes(i, j, row, col) {
                    result.push((i as usize, j as usize));
                }
            }
        }
        result
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
        Self {
            grid
        }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        &self.grid
    }

    fn sum_of_neighbours(&self, row: usize, col: usize) -> i32 {
        let mut sum = 0;
        let neighbours = self.grid.neighbours(row as i32, col as i32);
        for (row, col) in neighbours {
            match self.grid.get(row, col) {
                Cell::Dead => (),
                Cell::Alive => { sum += 1; }
            }
        }
        return sum;
    }

    pub fn step(&mut self) {
        let mut future_grid = self.grid.clone();
        for row in 0..self.grid.rows {
            for col in 0..self.grid.cols {
                let element = self.grid.get(row, col);
                let sum = self.sum_of_neighbours(row, col);
                match element {
                    Cell::Dead if sum == 3 => { future_grid.set(Cell::Alive, row, col); }
                    Cell::Alive if sum < 2 => { future_grid.set(Cell::Dead, row, col); }
                    Cell::Alive if sum > 3 => { future_grid.set(Cell::Dead, row, col); }
                    Cell::Alive => (),
                    _ => {}
                }
            }
        }
        self.grid = future_grid;
    }
}
