
use rand::{Rng, thread_rng};

/// Size of game board.
pub const SIZE: usize = 4;
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

#[derive(Clone, Copy)]
pub enum MoveDirection {
    Up,
    Right,
    Down,
    Left,
}

pub struct Position {
    x: usize,
    y: usize,
}
impl Position{
    pub fn new(x: usize, y: usize) -> Position {
        Position{
            x: x,
            y: y,
        }
    }
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

    pub fn handle_move(&mut self, move_direction: MoveDirection){
        let moved = self.move_command(move_direction);
    }

    pub fn move_command(&mut self, move_direction: MoveDirection) -> bool{
        let mut modifications: Vec<(Position, Position, Cell)> = vec![];
        let mut iter_order_x: Vec<usize> = (0..4).collect();
        let mut iter_order_y: Vec<usize> = (0..4).collect();
        match move_direction {
            MoveDirection::Right => iter_order_x.reverse(),
            MoveDirection::Down => iter_order_y.reverse(),
            _ => (),
        };
        for x in iter_order_x{
            for y in iter_order_y.clone(){
                println!("X: {}, Y: {}", x, y);
                let cell = self.cell([x, y]);
                match cell {
                    Cell::Occupied(n) => {
                        match move_direction {
                            MoveDirection::Up => {
                                if y != 0{
                                    print!("Moved up");
                                    let mut next_position: Position = Position::new(x,y);
                                    let mut new_cell = cell;
                                    let mut moved = false;
                                    let mut next_y = y - 1;
                                    loop{
                                        println!("Next_y {}", next_y);
                                        match self.cell([x, next_y]) {
                                            Cell::Empty => {next_position = Position::new(x, next_y); moved = true;}
                                            Cell::Occupied(m) => {
                                                if n == m {
                                                    next_position = Position::new(x, next_y);
                                                    new_cell = Cell::Occupied(n*2);
                                                    moved = true;
                                                    break;
                                                }
                                            }
                                        }
                                        if next_y == 0{
                                            break;
                                        }
                                        next_y -= 1;
                                    };
                                    if moved{
                                        println!("Move happened");
                                        self.set([next_position.x, next_position.y], new_cell);
                                        self.set([x, y], Cell::Empty);
                                    }
                                }
                            }
                            MoveDirection::Right => {
                                if x != 3{
                                    print!("Moved up");
                                    let mut next_position: Position = Position::new(x,y);
                                    let mut new_cell = cell;
                                    let mut moved = false;
                                    let mut next_x = x + 1;
                                    loop{
                                        println!("Next_x {}", next_x);
                                        match self.cell([next_x, y]) {
                                            Cell::Empty => {next_position = Position::new(next_x, y); moved = true;}
                                            Cell::Occupied(m) => {
                                                if n == m {
                                                    next_position = Position::new(next_x, y);
                                                    new_cell = Cell::Occupied(n*2);
                                                    moved = true;
                                                }
                                                break;
                                            }
                                        }
                                        if next_x == 3{
                                            break;
                                        }
                                        next_x += 1;
                                    };
                                    if moved{
                                        println!("Move happened");
                                        self.set([next_position.x, next_position.y], new_cell);
                                        self.set([x, y], Cell::Empty);
                                    }

                                }
                            }
                            MoveDirection::Down => {
                                if y != 3{
                                    print!("Moved down");
                                    let mut next_position: Position = Position::new(x,y);
                                    let mut new_cell = cell;
                                    let mut moved = false;
                                    let mut next_y = y + 1;
                                    loop{
                                        println!("Next_y {}", next_y);
                                        match self.cell([x, next_y]) {
                                            Cell::Empty => {next_position = Position::new(x, next_y); moved = true;}
                                            Cell::Occupied(m) => {
                                                if n == m {
                                                    next_position = Position::new(x, next_y);
                                                    new_cell = Cell::Occupied(n*2);
                                                    moved = true;
                                                    break;
                                                }
                                            }
                                        }
                                        if next_y == 3{
                                            break;
                                        }
                                        next_y += 1;
                                    };
                                    if moved{
                                        println!("Move happened");
                                        self.set([next_position.x, next_position.y], new_cell);
                                        self.set([x, y], Cell::Empty);
                                    }
                                }
                            }
                            MoveDirection::Left => {
                                if x != 0{
                                    print!("Moved left");
                                    let mut next_position: Position = Position::new(x,y);
                                    let mut new_cell = cell;
                                    let mut moved = false;
                                    let mut next_x = x - 1;
                                    loop{
                                        println!("Next_x {}", next_x);
                                        match self.cell([next_x, y]) {
                                            Cell::Empty => {next_position = Position::new(next_x, y); moved = true;}
                                            Cell::Occupied(m) => {
                                                if n == m {
                                                    next_position = Position::new(next_x, y);
                                                    new_cell = Cell::Occupied(n*2);
                                                    moved = true;
                                                }
                                                break;
                                            }
                                        }
                                        if next_x == 0{
                                            break;
                                        }
                                        next_x -= 1;
                                    };
                                    if moved{
                                        println!("Move happened");
                                        self.set([next_position.x, next_position.y], new_cell);
                                        self.set([x, y], Cell::Empty);
                                    }

                                }
                            }
                            _ => (),
                        }
                    }
                    Cell::Empty => (),
                }
            }
        }
        true
    }

	/// Gets the character at cell location.
	pub fn cell(&self, ind: [usize; 2]) -> Cell {
		self.cells[ind[1]][ind[0]]
	}

	/// Set cell value.
	pub fn set(&mut self, ind: [usize; 2], val: Cell) {
		self.cells[ind[1]][ind[0]] = val;
	}
}
