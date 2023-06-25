extern crate rand;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
struct Cell {
    state: bool,
}

impl Cell {
    fn new() -> Self {
        Self { state: false }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new()
    }
}

struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn new(width: i32, height: i32) -> Grid {
        let cells = vec![vec![Cell::new(); height as usize]; width as usize];
        Grid { cells }
    }

    fn iterate(&mut self) {
        for i in 0..self.cells.len() as i32 {
            for j in 0..self.cells[i as usize].len() as i32 {
                let neighbors = self.get_neighbors(i, j);
                let state = self.cells[i as usize][j as usize].state;
                let new_state = self.get_new_state(state, neighbors);
                self.cells[i as usize][j as usize].state = new_state;
            }
        }
    }

    fn get_neighbors(&self, i: i32, j: i32) -> Vec<Cell> {
        let mut neighbors = vec![];
        for di in -1..2 {
            for dj in -1..2 {
                let ni = i + di;
                let nj = j + dj;
                if ni >= 0i32
                    && ni < self.cells.len() as i32
                    && nj >= 0i32
                    && nj < self.cells[ni as usize].len() as i32
                {
                    neighbors.push(self.cells[ni as usize][nj as usize]);
                }
            }
        }
        neighbors
    }

    fn get_new_state(&self, state: bool, neighbors: Vec<Cell>) -> bool {
        let mut count = 0;
        for neighbor in neighbors {
            if neighbor.state {
                count += 1;
            }
        }
        if state {
            count == 2 || count == 3
        } else {
            count == 3
        }
    }

    fn place_room_instance(&mut self, _width: usize, _height: usize) {
        let mut found_position = false;
        while !found_position {
            let i = rand::thread_rng().gen_range(0..self.cells.len());
            let j = rand::thread_rng().gen_range(0..self.cells[i].len());
            if !self.cells[i][j].state {
                self.cells[i][j].state = true;
                found_position = true;
            }
        }
    }
}

fn main() {
    let mut grid = Grid::new(100, 100);
    for _a in 0..10 {
        grid.place_room_instance(10, 10);
    }

    // Print the grid.
    for row in &grid.cells {
        for cell in row {
            if cell.state {
                print!("1 ");
            } else {
                print!("0 ");
            }
        }
        println!();
    }
}
