//! A 2048 game.
#[deny(missing_docs)]
extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow;
use piston::event_loop::{Events, EventSettings, EventLoop};
use opengl_graphics::{OpenGL, GlGraphics, Filter, GlyphCache, TextureSettings};
use piston::input::RenderEvent;

pub use gameboard::Gameboard;
pub use gameboard::{Cell, MoveDirection, SIZE, GameState};
pub use gameboard_controller::GameboardController;
pub use gameboard_view::{GameboardView, GameboardViewSettings};

mod gameboard;
mod gameboard_controller;
mod gameboard_view;

fn main() {
let opengl = OpenGL::V3_2;
	let settings = WindowSettings::new("2048", [512; 2])
        .opengl(opengl)
  	    .exit_on_esc(true);
	let mut window: GlutinWindow = settings.build()
	    .expect("Could not create window");

	let mut events = Events::new(EventSettings::new().lazy(true));

	let mut gl = GlGraphics::new(opengl);

	let gameboard = Gameboard::new(true);
	let mut gameboard_controller = GameboardController::new(gameboard);
	let gameboard_view_settings = GameboardViewSettings::new();
	let gameboard_view = GameboardView::new(gameboard_view_settings);


    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
		.expect("Could not load font");

	while let Some(e) = events.next(&mut window) {
        gameboard_controller.event(&e);
		if let Some(args) = e.render_args() {
			gl.draw(args.viewport(), |c, g| {
				use::graphics::{clear};
				clear([1.0; 4], g);
				gameboard_view.draw(&gameboard_controller, glyphs, &c, g);
			});

    	}
	}
}
