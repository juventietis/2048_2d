//! Gameboard controller.

use piston::input::GenericEvent;

use {Gameboard, MoveDirection};

/// Handles events for Sudoku game.
pub struct GameboardController {
    /// Stores the gameboard state.
    pub gameboard: Gameboard,

}

impl GameboardController {
    /// Creates a new gameboard controller.
    pub fn new(gameboard: Gameboard) -> GameboardController {
        GameboardController {
            gameboard: gameboard,
        }
    }

    fn move_command(&mut self, move_direction: MoveDirection){
        self.gameboard.handle_move(move_direction);
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        use piston::input::{Button, Key};

        if let Some(Button::Keyboard(key)) = e.press_args(){
            match key {
                Key::Up | Key::W => self.move_command(MoveDirection::Up),
                Key::Right | Key::D => self.move_command(MoveDirection::Right),
                Key::Down | Key::S => self.move_command(MoveDirection::Down),
                Key::Left | Key::A => self.move_command(MoveDirection::Left),
                _ => (),
            }
        }

    }
}
