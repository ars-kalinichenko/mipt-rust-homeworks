# Conway's Game of Life

## Task

Implement the [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

In this game, the board is made up of a grid of cells, where each cell has an initial state: alive or dead. Each cell interacts with its eight neighbors (horizontal, vertical, diagonal) using the following four rules (taken from the above Wikipedia article):

1. Any live cell with fewer than two live neighbors dies as if caused by under-population.
2. Any live cell with two or three live neighbors lives on to the next generation.
3. Any live cell with more than three live neighbors dies, as if by over-population.
4. Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.

The next state is created by applying the above rules simultaneously to every cell in the current state, where births and deaths occur simultaneously.

## Questions

- When should we use `enum`? How can `match` help us to avoid nasty `if`'s?
- Why shouldn't we use `Vec<Vec<Cell>>` instead of `Vec<Cell>` in the definition of the grid?

## Advanced level

Change `Grid::neighbours` so that it will return some iterator instead of `Vec`, i.e. make no allocation solution.
