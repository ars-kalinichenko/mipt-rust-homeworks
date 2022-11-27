use conway::{Cell, GameOfLife, Grid};

fn get_grid(grid: Vec<Vec<u8>>) -> Grid<Cell> {
    let rows = grid.len();
    let cols = grid[0].len();
    let grid: Vec<Cell> = grid
        .into_iter()
        .flatten()
        .map(|value| if value == 0 { Cell::Dead } else { Cell::Alive })
        .collect();
    assert_eq!(grid.len(), rows * cols);
    Grid::from_slice(grid.as_slice(), rows, cols)
}

#[test]
fn grid_neighbours() {
    assert_eq!(
        Grid::<i32>::new(3, 3)
            .neighbours(2, 2)
            .into_iter()
            .collect::<Vec<_>>(),
        vec![(1, 1), (1, 2), (2, 1)]
    );
    assert_eq!(
        Grid::<i32>::new(1, 1)
            .neighbours(0, 0)
            .into_iter()
            .collect::<Vec<_>>(),
        vec![]
    );
    assert_eq!(
        Grid::<i32>::new(3, 4)
            .neighbours(1, 1)
            .into_iter()
            .collect::<Vec<_>>(),
        vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2)
        ]
    );
}

#[test]
fn first_rule() {
    #[rustfmt::skip]
    let grid = get_grid(vec![
        vec![1, 0, 0],
        vec![0, 1, 0],
        vec![0, 0, 0]
    ]);
    let final_grid = get_grid(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]);
    let mut game = GameOfLife::from_grid(grid.clone());
    game.step();
    assert!(game.get_grid() == &final_grid);
}

#[test]
fn second_rule() {
    #[rustfmt::skip]
    let grid = get_grid(vec![
        vec![1, 0, 0],
        vec![0, 1, 0],
        vec![0, 0, 1]
    ]);
    #[rustfmt::skip]
    let final_grid = get_grid(vec![
        vec![0, 0, 0],
        vec![0, 1, 0],
        vec![0, 0, 0]
    ]);
    let mut game = GameOfLife::from_grid(grid.clone());
    game.step();
    assert!(game.get_grid() == &final_grid);
}

#[test]
fn third_rule() {
    #[rustfmt::skip]
    let grid = get_grid(vec![
        vec![0, 1, 0],
        vec![1, 1, 1],
        vec![0, 1, 0]
    ]);
    let final_grid = get_grid(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]);
    let mut game = GameOfLife::from_grid(grid.clone());
    game.step();
    assert!(game.get_grid() == &final_grid);
}

#[test]
fn fourth_rule() {
    #[rustfmt::skip]
    let grid = get_grid(vec![
        vec![0, 0, 0],
        vec![0, 1, 0],
        vec![1, 0, 1]
    ]);
    #[rustfmt::skip]
    let final_grid = get_grid(vec![
        vec![0, 0, 0],
        vec![0, 1, 0],
        vec![0, 1, 0]
    ]);
    let mut game = GameOfLife::from_grid(grid.clone());
    game.step();
    assert!(game.get_grid() == &final_grid);
}

#[test]
fn glider() {
    let grid1 = get_grid(vec![
        vec![0, 1, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0],
        vec![1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 1],
        vec![0, 0, 0, 0, 1, 1],
    ]);
    let grid2 = get_grid(vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 0, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 1],
        vec![0, 0, 0, 0, 1, 1],
    ]);
    let grid3 = get_grid(vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0],
        vec![1, 0, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 1],
        vec![0, 0, 0, 0, 1, 1],
    ]);
    let grid4 = get_grid(vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 0, 0],
        vec![0, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 1, 1, 1],
        vec![0, 0, 0, 0, 1, 1],
    ]);
    let grid5 = get_grid(vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0],
        vec![0, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 1, 0, 1],
    ]);
    let grid6 = get_grid(vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 0],
        vec![0, 0, 0, 0, 1, 0],
    ]);
    let grid7 = get_grid(vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
    ]);

    let mut game = GameOfLife::from_grid(grid1.clone());
    assert!(game.get_grid() == &grid1);
    game.step();
    assert!(game.get_grid() == &grid2);
    game.step();
    assert!(game.get_grid() == &grid3);
    game.step();
    assert!(game.get_grid() == &grid4);
    game.step();
    assert!(game.get_grid() == &grid5);
    game.step();
    assert!(game.get_grid() == &grid6);
    game.step();
    assert!(game.get_grid() == &grid7);
    game.step();
    assert!(game.get_grid() == &grid7);
}
