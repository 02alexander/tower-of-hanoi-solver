
use opengl_graphics::GlGraphics;
use graphics::context::Context;
use graphics::rectangle::Rectangle;
use graphics::rectangle::Border;
use graphics::draw_state::DrawState;

pub struct Game {
	towers: [Vec<u32>; 3],
	n_disks: u32,
}

impl Game {
	pub fn new(height: u32) -> Self {
		let mut twrs = [Vec::new(), Vec::new(), Vec::new()];
		for i in (0..height).rev() {
			twrs[0].push(i+1);
		}
		Game {
			towers:twrs,
			n_disks:height,
		}
	}
	pub fn render(&mut self, c: Context, gl: &mut GlGraphics) {
		use graphics::*;

		const STICK_COLOR: [f32; 4] = [0.35, 0.2, 0.08, 1.0];
		const GRID_COLOR: [f32; 4] = [0.67, 0.67, 0.67, 1.0];
		const DISK_COLOR: [f32; 4] = [0.64, 0.03, 0.14, 1.0];
		let viewport = if let Some(v) = c.viewport {
			v
		} else {
			return;
		};
		let (screen_width, screen_height) = (viewport.rect[2], viewport.rect[3]);

		#[cfg(debug_assertions)] {
			let vertical_line = [0.0, 0.0, 1.0, screen_height as f64];
			let horizontal_line = [0.0, 0.0, screen_width as f64, 1.0];

			for i in 1..13 {
				let transform = c.transform.trans((i as f64 /13.0)*screen_width as f64, 0.0);
				rectangle(GRID_COLOR, vertical_line, transform, gl);
			}
			for i in 1..7 {
				let transform = c.transform.trans(0.0, (i as f64 / 7.0)*screen_height as f64);
				rectangle(GRID_COLOR, horizontal_line, transform, gl);
			}
		}

		for j in 1..4 {
			let i = 4.0*j as f64 - 3.0; // results in the sequence 1,5,9...
			let width = 3.0; // the width of everything that has to to rendered for 1 stick
			let height = 3.0; // the height of everything that has to rendered for 1 stick, including the stick
			let disk_space_height = 2.5; // space made for disks on every stick
			let stick_width = 0.5;
			let smallest_stick = 1.0;

			let width_denom = 13.0;
			let height_denom = 7.0;
			let transform = c.transform.trans((i/width_denom)*screen_width as f64, 
											  (3.0/height_denom)*screen_height as f64);
			let stick = [
				(3.0/width_denom)*screen_width as f64 / 2.0 - (stick_width/width_denom)*screen_width  as f64 / 2.0,
				0.0,
				(stick_width/width_denom)*screen_width as f64,
				(height/height_denom)*screen_height as f64
			];
			rectangle(STICK_COLOR, stick, transform, gl);

			let disk_height = if self.n_disks < 5 {
				disk_space_height / 5.0
			} else {
				disk_space_height / self.n_disks as f64
			};
			let mut transform = transform.trans(0.0,(height/height_denom)*screen_height as f64);
			for disk_size in &self.towers[j-1] {
				transform = transform.trans(0.0,-(disk_height/height_denom)*screen_height as f64);
				let disk_width = (width-smallest_stick)/self.n_disks as f64 * *disk_size as f64 + 1.0;
				let disk: [f64; 4] = [
					(width/width_denom)*screen_width as f64 / 2.0 - (disk_width/width_denom)*screen_width as f64 / 2.0,
					0.0, 
					(disk_width/width_denom)*screen_width as f64,
					(disk_height/height_denom)*screen_height as f64
				];
				
				Rectangle::new_border([0.2,0.2,0.2,1.0],1.0).color(DISK_COLOR)
															.draw(disk, &DrawState::new_alpha(), transform, gl);
			}
		}
	}
	pub fn move_disk(&mut self, src: usize, dst: usize) {
		let d = self.towers[src].pop().unwrap();
		self.towers[dst].push(d);
	}
}