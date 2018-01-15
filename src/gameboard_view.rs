//! Gameboard view.

use graphics::types::Color;
use graphics::{Context, Graphics};
use graphics::character::CharacterCache;
use graphics::Text;
use graphics::{Line, Rectangle, Transformed};

use GameboardController;
use Cell;
use GameState;

/// Stores gameboard view settings.
pub struct GameboardViewSettings {
    /// Position from left-top corner.
    pub position: [f64; 2],
    /// Size of gameboard along horizontal and vertical edge.
    pub size: f64,
    /// Background color.
    pub background_color: Color,
    /// Border color.
    pub border_color: Color,
    /// Edge color around the whole board.
    pub board_edge_color: Color,
    /// Edge color between the 3x3 sections.
    pub section_edge_color: Color,
    /// Edge color between cells.
    pub cell_edge_color: Color,
    /// Cell Color
    pub cell_color: Color,
    /// Edge radius around the whole board.
    pub board_edge_radius: f64,
    /// Edge radius between the 3x3 sections.
    pub section_edge_radius: f64,
    /// Edge radius between cells.
    pub cell_edge_radius: f64,
	/// Text color
	pub text_color: Color,
}

impl GameboardViewSettings {
    /// Creates new gameboard view settings.
    pub fn new() -> GameboardViewSettings {
        GameboardViewSettings {
            position: [10.0; 2],
            size: 400.0,
            background_color: [1.0, 0.89, 0.8, 1.0],
            border_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],
            section_edge_color: [0.0, 0.0, 0.2, 1.0],
            cell_edge_color: [0.0, 0.0, 0.2, 1.0],
            cell_color: [1.0, 1.0, 1.0, 0.70],
            board_edge_radius: 3.0,
            section_edge_radius: 2.0,
            cell_edge_radius: 1.0,
			text_color: [0.0, 0.0, 0.1, 1.0],
        }
    }
}

/// Stores visual information about a gameboard.
pub struct GameboardView {
    /// Stores gameboard view settings.
    pub settings: GameboardViewSettings,
}

impl GameboardView {
    /// Creates a new gameboard view.
    pub fn new(settings: GameboardViewSettings) -> GameboardView {
        GameboardView {
            settings: settings,
        }
    }

    fn get_cell_color(&self, cell: Cell) -> Color {
        let color: Color = [1.0, 0.8, 0.6, 1.0];
        match cell {
            Cell::Occupied(2) => [1.0, 0.8, 0.6, 1.0],
            Cell::Occupied(4) => [1.0, 0.6, 0.207, 1.0],
            Cell::Occupied(8) => [1.0, 0.5, 0.0, 1.0],
            Cell::Occupied(16) => [1.0, 0.4, 0.0, 1.0],
            Cell::Occupied(32) => [1.0, 0.3, 0.0, 1.0],
            Cell::Occupied(64) => [1.0, 0.2, 0.0, 1.0],
            Cell::Occupied(128) => [0.8, 0.2, 0.2, 1.0],
            Cell::Occupied(256) => [1.0, 1.0, 0.6, 1.0],
            Cell::Occupied(512) => [1.0, 0.7560, 0.4, 1.0],
            Cell::Occupied(1024) => [1.0, 0.6, 0.0, 1.0],
            Cell::Occupied(2048) => [0.84, 0.48, 0.149, 1.0],
            _ => color,
        }
    }

    fn draw_dialog<G: Graphics, C>(&self, 
			glyphs: &mut C, 
            text: &str,
			c: &Context,
			g: &mut G) where C: CharacterCache<Texture = G::Texture> {
        let ref settings = self.settings;
        let notif_rect = [
            settings.position[0] + 40.0, settings.position[1] + 40.0,
            settings.size - 80.0, settings.size - 80.0,
        ];
        // Draw board background.
        Rectangle::new_round(settings.cell_color, 10.0)
            .draw(notif_rect, &c.draw_state, c.transform, g);
        let text_pos = [
            settings.position[0] + 80.0,
            settings.position[1] + 160.0 + 60.0
        ];
        Text::new_color(settings.text_color, 60).draw(text,
                                            glyphs,
                                            &c.draw_state,
                                            c.transform.trans(text_pos[0], text_pos[1]),
                                            g);

    }

    /// Draw gameboard.
    pub fn draw<G: Graphics, C>(&self, 
			controller: &GameboardController,
			glyphs: &mut C, 
			c: &Context,
			g: &mut G) where C: CharacterCache<Texture = G::Texture> {

        let ref settings = self.settings;
        let board_rect = [
            settings.position[0], settings.position[1],
            settings.size, settings.size,
        ];

        // Draw board background.
        Rectangle::new(settings.background_color)
            .draw(board_rect, &c.draw_state, c.transform, g);

        // Draw section borders.
        let section_edge = Line::new(settings.section_edge_color, settings.section_edge_radius);
		let cell_size = settings.size / 4.0;
        for i in 0..4 {
            // Set up coordinates.
            let x = settings.position[0] + i as f64 / 4.0 * settings.size;
            let y = settings.position[1] + i as f64 / 4.0 * settings.size;
            let x2 = settings.position[0] + settings.size;
            let y2 = settings.position[1] + settings.size;

            let vline = [x, settings.position[1], x, y2];
            section_edge.draw(vline, &c.draw_state, c.transform, g);

            let hline = [settings.position[0], y, x2, y];
            section_edge.draw(hline, &c.draw_state, c.transform, g);
        }

        // Draw board edge.
        Rectangle::new_round_border(settings.board_edge_color, 5.0, settings.board_edge_radius)
            .draw(board_rect, &c.draw_state, c.transform, g);

		// Draw characters.
		for i in 0..4 {
			for j in 0..4 {
                let cell = controller.gameboard.cell([i, j]);
                match cell {
                    Cell::Occupied(n) => {
                        let cell_color = self.get_cell_color(cell);
                        let cell_rect = [
                            settings.position[0] + j as f64 * cell_size + 10.0,
                            settings.position[1] + i as f64 * cell_size + 10.0,
                            cell_size - 20.0,
                            cell_size - 20.0 ,
                        ];
                        Rectangle::new_round(cell_color, 5.0).draw(cell_rect, &c.draw_state, c.transform, g);
                        let cell_val_str: String = n.to_string();
                        let n_char = cell_val_str.chars().count();
                        let pad_x = match n_char {
                            1 => 25.0,
                            2 => 15.0,
                            3 => 2.5,
                            _ => 0.0,
                        };
                        let font_size = if n_char <= 3{
                            50
                        } else {
                            40
                        };
                        let pos = [
                            settings.position[0] + j as f64 * cell_size + 10.0 + pad_x,
                            settings.position[1] + i as f64 * cell_size + 60.0
                        ];
                        Text::new_color(settings.text_color, font_size).draw(&n.to_string(),
                                                            glyphs,
                                                            &c.draw_state,
                                                            c.transform.trans(pos[0], pos[1]),
                                                            g);

                    }
                    Cell::Empty => (),
                }
			}

            match controller.game_state{
                GameState::Lost => {
                    self.draw_dialog(glyphs, "You lost!", &c, g);
                }
                GameState::Won => {
                    self.draw_dialog(glyphs, "You won!", &c, g);
                }
                _ => (),
            }
		}
    }
}
