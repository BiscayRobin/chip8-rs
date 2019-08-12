use crate::bits::GetBits;

pub struct Display {
	screen: [bool; 64 * 32],
}

impl GetBits for u8 {
	fn get_bit(&self, index: usize) -> bool {
		(self >> index) % 2 == 1
	}
}

fn coord_to_index(x: usize, y: usize, width: usize) -> usize {
	(y * width) + x
}

impl Display {
	pub fn new() -> Self {
		Self {
			screen: [false; 64 * 32],
		}
	}
	pub fn cls(&mut self) {
		self.screen = [false; 64 * 32];
	}

	pub fn draw_line_at(&mut self, byte: u8, idx: u8, idy: u8) {
		for i in 0..8 {
			self.screen[coord_to_index(
				(idx as usize + i) % 64,
				(idy as usize) % 32,
				64,
			)] ^= byte.get_bit(7 - i);
		}
	}

	pub fn to_minifb_buffer(&self) -> [u32; 640 * 320] {
		let mut buf = [0; 640 * 320];
		for x in 0..640 {
			for y in 0..320 {
				let x_screen = x / 10;
				let y_screen = y / 10;
				//println!("(x:{}, y:{}) -> (x:{}, y:{})", x, y, x_screen, y_screen);
				if self.screen[coord_to_index(x_screen, y_screen, 64)] {
					buf[coord_to_index(x, y, 640)] = 0xFFFF_FFFF;
				}
			}
		}
		buf
	}
}
