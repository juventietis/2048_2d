
use rand::{Rng, thread_rng};

/// Size of game board.
const SIZE: usize = 4;
const NUMBER_OF_FILLED_CELL_AT_START: usize = 4;
const STARTING_CELL_NUMBER: usize = 2;

/// Stores game board information.
pub struct Gameboard {
    /// Stores the content of the cells.
    /// `0` is an empty cell.
    pub cells: [[Cell; SIZE]; SIZE],
}

#[derive(Clone, Copy)]
pub enum Cell {
    Occupied(usize),
    Empty,
}

impl Gameboard {
    /// Creates a new game board.
    pub fn new() -> Gameboard {
        let board = [[Cell::Empty; SIZE]; SIZE];
        let mut gameboard = Gameboard {
            cells: board,
        };
        gameboard.set_up_board();
        gameboard
    }

    /// Sets up the initial board.
    pub fn set_up_board(&mut self) {
        for _ in 0..NUMBER_OF_FILLED_CELL_AT_START{
            let cell_x: usize = thread_rng().gen_range(0, SIZE);
            let cell_y: usize = thread_rng().gen_range(0, SIZE);
            self.cells[cell_x][cell_y] = Cell::Occupied(STARTING_CELL_NUMBER);
        }
    }

	/// Gets the character at cell location.
	pub fn cell_val(&self, ind: [usize; 2]) -> Option<char> {
		Some(match self.cells[ind[1]][ind[0]] {
		    Cell::Occupied(2) => '2',
            Cell::Occupied(_) => ' ',
		    Cell::Empty => ' ',
		})
	}

	/// Set cell value.
	pub fn set(&mut self, ind: [usize; 2], val: Cell) {
		self.cells[ind[1]][ind[0]] = val;
	}
}
