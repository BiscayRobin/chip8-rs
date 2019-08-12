pub struct Keyboard {
	keys: [bool; 0x10]
}

pub enum Key {
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Height,
	Nine,
	A,
	B,
	C,
	D,
	E,
	F,
}

impl Keyboard {
	pub fn new() -> Self {
		Self {
			keys: [false;0x10];
		}
	}

	pub fn press_key(&mut self, key: Key) {
		self.keys[key] = true;
	}

	pub fn release_key(&mut self, key: Key) {
		self.keys[key] = false;
	}

	pub fn is_key_down(&self, key: Key) -> bool {
		self.keys[key]
	}
}