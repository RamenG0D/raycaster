use nalgebra::{vector, Vector3};
use raylib::{color::Color, ffi::KeyboardKey, prelude::RaylibDraw};

pub struct Player {
	position: Vector3<f32>,
	rotation: f32,
}

impl Player {
	pub const SIZE: i32 = 50;
	pub const FOV: i32 = 60;

	pub fn new(x: f32, y: f32, z: f32) -> Self {
		Self {
			position: vector![x, y, z],
			rotation: 0.,
		}
	}
}

static MAP: [[i32; 10]; 10] = [
	[1,1,1,1,1,1,1,1,1,1],
	[1,0,0,0,0,0,0,0,0,1],
	[1,0,0,0,0,0,0,0,0,1],
	[1,0,0,0,0,0,0,0,0,1],
	[1,0,0,0,0,0,0,0,0,1],
	[1,0,0,0,0,0,0,0,0,1],
	[1,0,0,0,0,0,0,0,0,1],
	[1,0,0,0,0,0,0,0,0,1],
	[1,0,0,0,0,0,0,0,0,1],
	[1,1,1,1,1,1,1,1,1,1],
];

pub struct Ray(f32, f32);

pub const WALL_HEIGHT: i32 = 300;

impl Ray {
	pub fn new(x: f32, y: f32) -> Self {
		Self(x, y)
	}

	pub fn move_x(&mut self, xc: f32) {
		let Ray(x, _) = self;
		*x += xc;
	}

	pub fn move_y(&mut self, yc: f32) {
		let Ray(_, y) = self;
		*y += yc;
	}

	pub fn move_(&mut self, xc: f32, yc: f32) {
		let Ray(x, y) = self;
		*x += xc;
		*y += yc;
	}

	pub fn get_x(&self) -> f32 {
		let Ray(x, _) = self;
		*x
	}

	pub fn get_y(&self) -> f32 {
		let Ray(_, y) = self;
		*y
	}
}

fn main() {
	let (mut rl, rt) = raylib::init()
		.title("Raycaster")
		.size(600, 600)
		.vsync()
		// .resizable()
		.build();

	rl.set_target_fps(60);

	let mut p = Player::new(4., 4., 0.);

	while !rl.window_should_close() {
		if rl.is_key_down(KeyboardKey::KEY_W) {
			p.position.y -= 1.;
		}
		if rl.is_key_down(KeyboardKey::KEY_S) {
			p.position.y += 1.;
		}
		if rl.is_key_down(KeyboardKey::KEY_A) {
			p.position.x -= 1.;
		}
		if rl.is_key_down(KeyboardKey::KEY_D) {
			p.position.x += 1.;
		}

		if rl.is_key_down(KeyboardKey::KEY_LEFT) {
			p.rotation -= 0.1;
		}
		if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
			p.rotation += 0.1;
		}

		let mut rd = rl.begin_drawing(&rt);

		rd.clear_background(Color::RAYWHITE);

		let (px, py) = (
			p.position.x as i32,
			p.position.y as i32,
		);

		// now we want to draw a line which start in the middle of the player (px-(size/2)) and make sure it points in the direction they are looking
		let (dx, dy) = (
			p.rotation.cos() * 10.,
			p.rotation.sin() * 10.,
		);
		let (pmx, pmy) = (
			(px + Player::SIZE / 2) as f32,
			(py + Player::SIZE / 2) as f32,
		);

		rd.draw_line(
			pmx as i32,
			pmy as i32,
			(pmx + (dx * 6.)) as i32,
			(pmy + (dy * 6.)) as i32,
			Color::RED
		);

		rd.draw_rectangle(px, py, Player::SIZE, Player::SIZE, Color::BLACK);
	}
}
