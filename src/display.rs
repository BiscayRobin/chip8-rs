use sbitty::GetBit;

pub struct Display {
	screen: [bool; 64 * 32],
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

	#[cfg(test)]
	pub fn dump(&self) -> [bool; 64 * 32] {
		self.screen
	}

	pub fn cls(&mut self) {
		self.screen = [false; 64 * 32];
	}

	pub fn draw_line_at(&mut self, byte: u8, idx: u8, idy: u8) -> bool {
		let mut collision = false;
		for i in 0..8 {
			let to_print = byte.get_bit(7 - i).unwrap_or(false);
			if self.screen
				[coord_to_index((idx as usize + i) % 64, (idy as usize) % 32, 64)]
				&& to_print
			{
				collision = true;
			}
			self.screen[coord_to_index(
				(idx as usize + i) % 64,
				(idy as usize) % 32,
				64,
			)] ^= to_print;
		}
		collision
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
