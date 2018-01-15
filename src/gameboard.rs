
use rand::{Rng, thread_rng};

/// Size of game board.
pub const SIZE: usize = 4;
const NUMBER_OF_FILLED_CELL_AT_START: usize = 4;
const STARTING_CELL_NUMBER: usize = 2;
const CHANCE_OF_ADDING_CELLS: usize = 4;
const MAX_NUMBER_OF_NEW_CELLS_TO_ADD: usize = 4;
const NUMBER_OF_CELLS: usize = SIZE * SIZE;

type Cells = [[Cell; SIZE]; SIZE];

/// Stores game board information.
pub struct Gameboard {
    /// Stores the content of the cells.
    pub cells: Cells,
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

#[derive(Clone, Copy)]
pub enum GameState {
    Won,
    Lost,
    Playing,
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

    pub fn maybe_add_new_cells(&mut self){
        let chance = thread_rng().gen_range(0, CHANCE_OF_ADDING_CELLS);
        if chance == 0 {
            let current_num_of_filled = self.number_of_filled_cels();
            let num_of_free = NUMBER_OF_CELLS - current_num_of_filled;
            let max_cells_to_add = thread_rng().gen_range(2, MAX_NUMBER_OF_NEW_CELLS_TO_ADD);
            if num_of_free != 0{
                let cells_to_add = ::std::cmp::min(num_of_free, max_cells_to_add);
                for _ in 0..cells_to_add{
                    loop{
                        let cell_x: usize = thread_rng().gen_range(0, SIZE);
                        let cell_y: usize = thread_rng().gen_range(0, SIZE);
                        match self.cell([cell_x, cell_y]) {
                            Cell::Empty => {
                                self.set([cell_x, cell_y], Cell::Occupied(STARTING_CELL_NUMBER));
                                break;
                            }
                            Cell::Occupied(_) => ()
                        }
                    }
                }
            }
        }
    }

    fn number_of_filled_cels(&self) -> usize {
        let mut count = 0;
        for i in 0..4{
            for j in 0..4{
                match self.cell([i,j]) {
                    Cell::Occupied(_) => count += 1,
                    Cell::Empty => (),
                }
            }
        }
        count
    }

    fn can_move(&self) -> bool {
        let mut can_move = false;
        for movement in [MoveDirection::Up, MoveDirection::Right, MoveDirection::Down, MoveDirection::Left].iter(){
            let board = self.cells.clone();
            let (moved, _) = self.move_command(*movement, board);
            if moved{
                 can_move = true;
                 break;
            }
        }
        return can_move;
    }

	/// Gets the character at cell location.
	pub fn cell_val(&self, ind: [usize; 2]) -> Option<char> {
		Some(match self.cells[ind[1]][ind[0]] {
		    Cell::Occupied(2) => '2',
            Cell::Occupied(_) => ' ',
		    Cell::Empty => ' ',
		})
	}

    pub fn handle_move(&mut self, move_direction: MoveDirection) -> GameState{
        let board = self.cells.clone();
        let (moved, board_after_move) = self.move_command(move_direction, board);
        self.cells = board_after_move;
        if moved{
            self.maybe_add_new_cells();
        }
        let can_still_move = self.can_move();
        if !can_still_move{
            println!("You lost!");
            return GameState::Lost;
        }
        return GameState::Playing;
    }

    pub fn move_command(&self, move_direction: MoveDirection, mut cells: Cells) -> (bool, Cells) {
        let mut executed_move = false;
        let mut iter_order_x: Vec<usize> = (0..4).collect();
        let mut iter_order_y: Vec<usize> = (0..4).collect();
        match move_direction {
            MoveDirection::Right => iter_order_x.reverse(),
            MoveDirection::Down => iter_order_y.reverse(),
            _ => (),
        };
        for x in iter_order_x{
            for y in iter_order_y.clone(){
                let cell = cells[x][y];
                match cell {
                    Cell::Occupied(n) => {
                        match move_direction {
                            MoveDirection::Up => {
                                if y != 0{
                                    let mut next_position: Position = Position::new(x,y);
                                    let mut new_cell = cell;
                                    let mut moved = false;
                                    let mut next_y = y - 1;
                                    loop{
                                        match cells[x][next_y] {
                                            Cell::Empty => {next_position = Position::new(x, next_y); moved = true;}
                                            Cell::Occupied(m) => {
                                                if n == m {
                                                    next_position = Position::new(x, next_y);
                                                    new_cell = Cell::Occupied(n*2);
                                                    moved = true;
                                                }
                                                break;
                                            }
                                        }
                                        if next_y == 0{
                                            break;
                                        }
                                        next_y -= 1;
                                    };
                                    if moved{
                                        cells[next_position.x][next_position.y] = new_cell;
                                        cells[x][y] = Cell::Empty;
                                        executed_move = true;
                                    }
                                }
                            }
                            MoveDirection::Right => {
                                if x != 3{
                                    let mut next_position: Position = Position::new(x,y);
                                    let mut new_cell = cell;
                                    let mut moved = false;
                                    let mut next_x = x + 1;
                                    loop{
                                        match cells[next_x][y] {
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
                                        cells[next_position.x][next_position.y] = new_cell;
                                        cells[x][y] = Cell::Empty;
                                        executed_move = true;
                                    }

                                }
                            }
                            MoveDirection::Down => {
                                if y != 3{
                                    let mut next_position: Position = Position::new(x,y);
                                    let mut new_cell = cell;
                                    let mut moved = false;
                                    let mut next_y = y + 1;
                                    loop{
                                        match cells[x][next_y] {
                                            Cell::Empty => {next_position = Position::new(x, next_y); moved = true;}
                                            Cell::Occupied(m) => {
                                                if n == m {
                                                    next_position = Position::new(x, next_y);
                                                    new_cell = Cell::Occupied(n*2);
                                                    moved = true;
                                                }
                                                break;
                                            }
                                        }
                                        if next_y == 3{
                                            break;
                                        }
                                        next_y += 1;
                                    };
                                    if moved{
                                        cells[next_position.x][next_position.y] = new_cell;
                                        cells[x][y] = Cell::Empty;
                                        executed_move = true;
                                    }
                                }
                            }
                            MoveDirection::Left => {
                                if x != 0{
                                    let mut next_position: Position = Position::new(x,y);
                                    let mut new_cell = cell;
                                    let mut moved = false;
                                    let mut next_x = x - 1;
                                    loop{
                                        match cells[next_x][y] {
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
                                        cells[next_position.x][next_position.y] = new_cell;
                                        cells[x][y] = Cell::Empty;
                                        executed_move = true;
                                    }

                                }
                            }
                        }
                    }
                    Cell::Empty => (),
                }
            }
        }
        (executed_move, cells)
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
