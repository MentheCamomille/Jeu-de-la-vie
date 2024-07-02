mod interface;
mod grid;

use interface::X11Interface;
use grid::Grid;

fn main() {
    let mut grid = Grid::new(100, 60); 
    let mut interface = X11Interface::new(&mut grid);
    
    interface.run(); 
}
