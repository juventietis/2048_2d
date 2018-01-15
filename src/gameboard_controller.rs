//! Gameboard controller.

use piston::input::GenericEvent;

use {Gameboard, MoveDirection, GameState};

/// Handles events for Sudoku game.
pub struct GameboardController {
    /// Stores the gameboard state.
    pub gameboard: Gameboard,
    pub game_state: GameState,

}

impl GameboardController {
    /// Creates a new gameboard controller.
    pub fn new(gameboard: Gameboard) -> GameboardController {
        GameboardController {
            gameboard: gameboard,
            game_state: GameState::Playing,
        }
    }

    fn move_command(&mut self, move_direction: MoveDirection){
        match self.game_state {
            GameState::Playing => {
                let new_game_state = self.gameboard.handle_move(move_direction);
                self.game_state = new_game_state;
            }
            GameState::Won => {
                self.game_state = GameState::Playing;
            }
            _ =>(),
        }
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
