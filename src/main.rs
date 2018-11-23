extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use opengl_graphics::{OpenGL, GlGraphics};
use glutin_window::GlutinWindow;
use piston::event_loop::{Events, EventSettings, EventLoop};
use piston::input::RenderEvent;
use piston::input::{Button, Key};
use piston::input::PressEvent;
use graphics::clear;
use game::Game;
use solver::move_n_disks;
use std::env;

mod game;
mod solver;

fn main() {
	let opengl = OpenGL::V3_2;
	let settings = WindowSettings::new("Tower of hanoi", [1024, 640])
		.opengl(opengl)
		.exit_on_esc(true);
	let mut window: GlutinWindow = settings.build()
		.expect("Failed to create window");
	let mut events = Events::new(EventSettings::new());
	let mut gl = GlGraphics::new(opengl);

	let args: Vec<_> = env::args().collect();
	let n_disks = if let Some (arg) = args.get(1) {
		arg.parse::<u32>().unwrap()
	} else {
		5
	};

	let mut game = Game::new(n_disks);
	let solution = move_n_disks(n_disks,0,2);
	let mut solution = solution.iter();

	while let Some(e) = events.next(&mut window) {
		if let Some(args) = e.render_args() {
			gl.draw(args.viewport(), |c, gl| {
				clear([0.0,0.0,0.0,1.0], gl);
				game.render(c, gl);
			})
		}
		if let Some(Button::Keyboard(Key::Return)) = e.press_args() {
			if let Some((src, dst)) = solution.next() {
				game.move_disk(*src, *dst);
			} else {
				break;
			}
		}
	}		
}