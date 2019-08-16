use minifb::{Key, KeyRepeat, Window};

pub struct Keyboard {
	keys: [bool; 0x10],
}

impl Keyboard {
	pub fn new() -> Self {
		Self {
			keys: [false; 0x10],
		}
	}

	pub fn press_key(&mut self, key: u8) {
		self.keys[key as usize] = true;
	}

	pub fn is_key_down(&self, key: u8) -> bool {
		self.keys[key as usize]
	}

	pub fn update_keys(&mut self, window: &Window) {
		self.keys = [false; 0x10];
		let x = (*window).get_keys_pressed(KeyRepeat::Yes);
		if let Some(keys) = x {
			for k in keys {
				let id = match k {
					Key::F1 => 0x1,
					Key::F2 => 0x2,
					Key::F3 => 0x3,
					Key::F4 => 0xC,
					Key::A => 0x4,
					Key::Z => 0x5,
					Key::E => 0x6,
					Key::R => 0xD,
					Key::Q => 0x7,
					Key::S => 0x8,
					Key::D => 0x9,
					Key::F => 0xE,
					Key::W => 0xA,
					Key::X => 0x0,
					Key::C => 0xB,
					Key::V => 0xF,
					_ => 0x10,
				};
				if id != 0x10 {
					self.press_key(id);
				}
			}
		};
	}
}
