pub struct Grid {
    cells: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        let cells = vec![vec![false; height]; width];

        Grid {
            cells,
            width,
            height,
        }
    }

    pub fn toggle_cell(&mut self, x: usize, y: usize) {
        self.cells[x][y] = !self.cells[x][y];
    }

    pub fn get_cell(&self, x: usize, y: usize) -> bool {
        self.cells[x][y]
    }

    pub fn set_cell(&mut self, x: usize, y: usize, alive: bool) {
        self.cells[x][y] = alive;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn evolve(&mut self) {
        let mut next_gen = self.cells.clone();

        for i in 0..self.width {
            for j in 0..self.height {
                let live_neighbors = self.count_live_neighbors(i, j);

                if self.cells[i][j] {
                    if live_neighbors < 2 || live_neighbors > 3 {
                        next_gen[i][j] = false; // mort par ss-population ou surpopulation
                    }
                } else {
                    if live_neighbors == 3 {
                        next_gen[i][j] = true; // la naissance de la nvlle cell
                    }
                }
            }
        }

        self.cells = next_gen;
    }

    fn count_live_neighbors(&self, x: usize, y: usize) -> usize {
        let mut live_count = 0;
        let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

        for (dx, dy) in directions.iter() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                if self.cells[nx as usize][ny as usize] {
                    live_count += 1;
                }
            }
        }

        live_count
    }
}
