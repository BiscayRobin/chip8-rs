pub struct Keyboard {
	keys: [bool; 0x10]
}

impl Keyboard {
	pub fn new() -> Self {
		Self {
			keys: [false;0x10]
		}
	}

	pub fn press_key(&mut self, key: u8) {
		self.keys[key as usize] = true;
	}

	pub fn release_key(&mut self, key: u8) {
		self.keys[key as usize] = false;
	}

	pub fn is_key_down(&self, key: u8) -> bool {
		self.keys[key as usize]
	}
}