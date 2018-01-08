//! Gameboard view.

use graphics::types::Color;
use graphics::{Context, Graphics};
use graphics::character::CharacterCache;

use GameboardController;

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
            background_color: [0.8, 0.8, 1.0, 1.0],
            border_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],
            section_edge_color: [0.0, 0.0, 0.2, 1.0],
            cell_edge_color: [0.0, 0.0, 0.2, 1.0],
            cell_color: [1.0, 1.0, 1.0, 1.0],
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

    /// Draw gameboard.
    pub fn draw<G: Graphics, C>(&self, 
			controller: &GameboardController,
			glyphs: &mut C, 
			c: &Context,
			g: &mut G) where C: CharacterCache<Texture = G::Texture> {
        use graphics::{Line, Rectangle, Transformed, Image};

        let ref settings = self.settings;
        let board_rect = [
            settings.position[0], settings.position[1],
            settings.size, settings.size,
        ];

		      // Draw board background.
        Rectangle::new(settings.background_color)
            .draw(board_rect, &c.draw_state, c.transform, g);

		      // Draw cell borders.
        let cell_edge = Line::new(settings.cell_edge_color, settings.cell_edge_radius);

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
        Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius)
            .draw(board_rect, &c.draw_state, c.transform, g);

		// Draw characters.
		let text_image = Image::new_color(settings.text_color);
		for j in 0..4 {
			for i in 0..4 {
                let cell_rect = [
                    settings.position[0] + i as f64 * cell_size + 10.0,
                    settings.position[1] + j as f64 * cell_size + 10.0,
                    cell_size - 20.0,
                    cell_size - 20.0 ,
                ];
                Rectangle::new(settings.cell_color).draw(cell_rect, &c.draw_state, c.transform, g);
				if let Some(ch) = controller.gameboard.cell_val([i, j]) {
					let pos = [
						settings.position[0] + i as f64 * cell_size + 40.0,
						settings.position[1] + j as f64 * cell_size + 60.0
					];
					if let Ok(character) = glyphs.character(50, ch) {
						let ch_x = pos[0] + character.left();
						let ch_y = pos[1] - character.top();
						text_image.draw(character.texture,
										&c.draw_state,
										c.transform.trans(ch_x, ch_y),
										g);
					}
				}
			}
		}
    }
}
