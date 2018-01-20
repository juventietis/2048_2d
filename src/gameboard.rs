
use rand::{Rng, thread_rng};

/// Size of game board.
pub const SIZE: usize = 4;
const NUMBER_OF_FILLED_CELL_AT_START: usize = 4;
const STARTING_CELL_NUMBER: usize = 2;
const MAX_NUMBER_OF_NEW_CELLS_TO_ADD: usize = 2;
const NUMBER_OF_CELLS: usize = SIZE * SIZE;

type Cells = [[Cell; SIZE]; SIZE];

/// Stores game board information.
pub struct Gameboard {
    /// Stores the content of the cells.
    pub cells: Cells,
    pub has_already_won: bool,
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
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
    pub fn new(set_up_board: bool) -> Gameboard {
        let board = [[Cell::Empty; SIZE]; SIZE];
        let mut gameboard = Gameboard {
            cells: board,
            has_already_won: false,
        };
        if set_up_board{
            gameboard.set_up_board();
        }
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
        let current_num_of_filled = self.number_of_filled_cels();
        let num_of_free = NUMBER_OF_CELLS - current_num_of_filled;
        let max_cells_to_add = thread_rng().gen_range(1, MAX_NUMBER_OF_NEW_CELLS_TO_ADD);
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

    fn has_won(&self) -> bool {
        let mut has_won = false;
        for i in 0..4 {
            for j in 0..4{
                match self.cells[i][j]{
                    Cell::Occupied(2048) => {
                        has_won = true;
                        break;
                    }
                    _ => (),
                }
            }
        }
        return has_won;
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
        if !self.has_already_won && self.has_won(){
            println!("You won!");
            self.has_already_won = true;
            return GameState::Won;
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
                        let mut modifications = Vec::new();
                        match move_direction {
                            MoveDirection::Up => {
                                let mut loop_y = y;
                                while loop_y > 0{
                                    loop_y -= 1;
                                    modifications.push(Position::new(x, loop_y));
                                }
                            }
                            MoveDirection::Right => {
                                let mut loop_x = x;
                                while loop_x < 3{
                                    loop_x += 1;
                                    modifications.push(Position::new(loop_x, y));
                                }
                            }
                            MoveDirection::Down => {
                                let mut loop_y = y;
                                while loop_y < 3{
                                    loop_y += 1;
                                    modifications.push(Position::new(x, loop_y));
                                }
                            }
                            MoveDirection::Left => {
                                let mut loop_x = x;
                                while loop_x > 0{
                                    loop_x -= 1;
                                    modifications.push(Position::new(loop_x, y));
                                }
                            }
                        }
                        executed_move = self.try_apply_modifications(&mut cells, modifications, n, Position::new(x,y));
                    }
                    Cell::Empty => (),
                }
            }
        }
        (executed_move, cells)
    }

    pub fn try_apply_modifications(&self, cells: &mut Cells, modifications: Vec<Position>, current_cell_n: usize, current_cell_position: Position) -> bool{
        let mut modification: Option<(Cell, Position)> = Option::None;
        for Position{x, y} in modifications{
            match cells[x][y] {
                Cell::Empty => {
                    modification = Option::Some((Cell::Occupied(current_cell_n), Position::new(x, y)));
                }
                Cell::Occupied(m) => {
                    if current_cell_n == m {
                        modification = Option::Some((Cell::Occupied(current_cell_n * 2), Position::new(x, y)));
                    }
                    break;
                }
            }
        }
        let executed_move = match modification{
            Option::Some((new_cell, next_position)) => {
                cells[next_position.x][next_position.y] = new_cell;
                cells[current_cell_position.x][current_cell_position.y] = Cell::Empty;
                true
        }
            None => false
        };
        executed_move
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



#[cfg(test)]
mod tests {
    use {Gameboard, Cell, MoveDirection};

    #[test]
    fn simple_addition() {
        let mut gameboard: Gameboard = Gameboard::new(false);
        gameboard.cells[0][0] = Cell::Occupied(2);
        gameboard.cells[0][1] = Cell::Occupied(2);
        gameboard.handle_move(MoveDirection::Up);
        let cell = gameboard.cells[0][0];
        match cell{
            Cell::Occupied(n) => {
                assert_eq!(n, 4);
            }
            _ => {
                panic!("Cell not found!");
            }
        }
    }

    #[test]
    fn numbers_dont_add_up() {
        let mut gameboard: Gameboard = Gameboard::new(false);
        gameboard.cells[0][0] = Cell::Occupied(2);
        gameboard.cells[0][1] = Cell::Occupied(4);
        gameboard.handle_move(MoveDirection::Up);
        let cell = gameboard.cells[0][0];
        match cell{
            Cell::Occupied(n) => {
                assert_eq!(n, 2);
            }
            _ => {
                panic!("Cell not found!");
            }
        }
    }

    #[test]
    fn empty_cell_doesnt_add_up() {
        let mut gameboard: Gameboard = Gameboard::new(false);
        gameboard.cells[0][0] = Cell::Occupied(2);
        gameboard.cells[0][1] = Cell::Empty;
        gameboard.handle_move(MoveDirection::Up);
        let cell = gameboard.cells[0][0];
        match cell{
            Cell::Occupied(n) => {
                assert_eq!(n, 2);
            }
            _ => {
                panic!("Cell not found!");
            }
        }
    }

    #[test]
    fn addition_works_across_multiple_steps() {
        let mut gameboard: Gameboard = Gameboard::new(false);
        gameboard.cells[0][0] = Cell::Occupied(2);
        gameboard.cells[0][3] = Cell::Occupied(2);
        gameboard.handle_move(MoveDirection::Up);
        let cell = gameboard.cells[0][0];
        match cell{
            Cell::Occupied(n) => {
                assert_eq!(n, 4);
            }
            _ => {
                panic!("Cell not found!");
            }
        }
    }

    #[test]
    fn addition_works_for_other_numbers() {
        let mut gameboard: Gameboard = Gameboard::new(false);
        gameboard.cells[0][0] = Cell::Occupied(8);
        gameboard.cells[0][3] = Cell::Occupied(8);
        gameboard.handle_move(MoveDirection::Up);
        let cell = gameboard.cells[0][0];
        match cell{
            Cell::Occupied(n) => {
                assert_eq!(n, 16);
            }
            _ => {
                panic!("Cell not found!");
            }
        }
    }

    #[test]
    fn moving_left_works() {
        let mut gameboard: Gameboard = Gameboard::new(false);
        gameboard.cells[3][0] = Cell::Occupied(2);
        gameboard.handle_move(MoveDirection::Left);
        let cell = gameboard.cells[0][0];
        match cell{
            Cell::Occupied(n) => {
                assert_eq!(n, 2);
            }
            _ => {
                panic!("Cell not found!");
            }
        }
    }

    #[test]
    fn moving_right_works() {
        let mut gameboard: Gameboard = Gameboard::new(false);
        gameboard.cells[0][0] = Cell::Occupied(2);
        gameboard.handle_move(MoveDirection::Right);
        let cell = gameboard.cells[3][0];
        match cell{
            Cell::Occupied(n) => {
                assert_eq!(n, 2);
            }
            _ => {
                panic!("Cell not found!");
            }
        }
    }

    #[test]
    fn moving_down_works() {
        let mut gameboard: Gameboard = Gameboard::new(false);
        gameboard.cells[0][0] = Cell::Occupied(2);
        gameboard.handle_move(MoveDirection::Down);
        let cell = gameboard.cells[0][3];
        match cell{
            Cell::Occupied(n) => {
                assert_eq!(n, 2);
            }
            _ => {
                panic!("Cell not found!");
            }
        }
    }

    #[test]
    fn moving_up_works() {
        let mut gameboard: Gameboard = Gameboard::new(false);
        gameboard.cells[0][3] = Cell::Occupied(2);
        gameboard.handle_move(MoveDirection::Up);
        let cell = gameboard.cells[0][0];
        match cell{
            Cell::Occupied(n) => {
                assert_eq!(n, 2);
            }
            _ => {
                panic!("Cell not found!");
            }
        }
    }
}
