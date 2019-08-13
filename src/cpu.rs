use crate::display::Display;
use crate::keyboard::Keyboard;
use minifb::Window;
use rand::{rngs::ThreadRng, thread_rng, Rng};

pub struct Cpu {
	sp: usize,
	stack: [usize; 16],
	pc: usize,
	reg_v: [u8; 16],
	reg_i: usize,
	delay_t: u8,
	delay_s: u8,
	memory: [u8; 4096],
	display: Display,
	keyboard: Keyboard,
	rng: ThreadRng,
}

impl Cpu {
	pub fn new() -> Self {
		Self {
			sp: 0,
			stack: [0; 16],
			pc: 0x200, // first 200 are used by interpreter and font
			reg_v: [0; 16],
			reg_i: 0,
			delay_t: 16,
			delay_s: 16,
			memory: [0; 4096],
			display: Display::new(),
			keyboard: Keyboard::new(),
			rng: thread_rng(),
		}
	}

	pub fn load_at(&mut self, bytes: &[u8], index: usize) {
		self.memory[index..index + bytes.len()].copy_from_slice(bytes);
	}

	pub fn cycle(&mut self) {
		let operation = self.read_word(self.pc);
		self.pc += 2;
		self.exec_op_code(operation);
	}

	pub fn update_clock(&mut self) {
		if self.delay_t > 0 {
			self.delay_t -= 1;
		}
		if self.delay_s > 0 {
			self.delay_s -= 1;
		}
	}

	pub fn get_minifb_buffer(&self) -> [u32; 640 * 320] {
		self.display.to_minifb_buffer()
	}

	pub fn update_keys(&mut self, window: &Window) {
		self.keyboard.update_keys(window);
	}

	fn exec_op_code(&mut self, operation: u16) {
		//utils
		let addr = usize::from(operation & 0xFFF);
		let nibble = (operation & 0xF) as usize;
		let byte = (operation & 0xFF) as u8;
		let h = (operation & 0xF000) >> 12;
		let x = ((operation & 0xF00) >> 8) as usize;
		let y = ((operation & 0xF0) >> 4) as usize;
		let l = operation & 0xF;

		match (h, x, y, l) {
			(0x0, 0x0, 0xE, 0x0) => {
				self.display.cls();
			}
			(0x0, 0x0, 0xE, 0xE) => {
				self.sp -= 1;
				self.pc = self.stack[self.sp];
			}
			(0x1, _, _, _) => {
				self.pc = addr;
			}
			(0x2, _, _, _) => {
				self.stack[self.sp] = self.pc;
				self.pc = addr;
				self.sp += 1;
			}
			(0x3, _, _, _) => {
				if byte == self.reg_v[x] {
					self.pc += 2;
				}
			}
			(0x4, _, _, _) => {
				if byte != self.reg_v[x] {
					self.pc += 2;
				}
			}
			(0x5, _, _, 0x0) => {
				if self.reg_v[x] == self.reg_v[y] {
					self.pc += 2
				}
			}
			(0x6, _, _, _) => {
				self.reg_v[x] = byte;
			}
			(0x7, _, _, _) => {
				self.reg_v[x] = self.reg_v[x].wrapping_add(byte);
			}
			(0x8, _, _, 0x0) => {
				self.reg_v[x] = self.reg_v[y];
			}
			(0x8, _, _, 0x1) => {
				self.reg_v[x] |= self.reg_v[y];
			}
			(0x8, _, _, 0x2) => {
				self.reg_v[x] &= self.reg_v[y];
			}
			(0x8, _, _, 0x3) => {
				self.reg_v[x] ^= self.reg_v[y];
			}
			(0x8, _, _, 0x4) => {
				let (result, overflow) =
					self.reg_v[x].overflowing_add(self.reg_v[y]);
				self.reg_v[x] = result;
				self.reg_v[0xF] = if overflow { 1 } else { 0 };
			}
			(0x8, _, _, 0x5) => {
				let (result, borow) = self.reg_v[x].overflowing_sub(self.reg_v[y]);
				self.reg_v[x] = result;
				self.reg_v[0xF] = if borow { 0 } else { 1 };
			}
			(0x8, _, _, 0x6) => {
				self.reg_v[0xF] = if (self.reg_v[x] % 2) == 1 { 1 } else { 0 };
				self.reg_v[x] >>= 1;
			}
			(0x8, _, _, 0x7) => {
				let (result, borow) = self.reg_v[y].overflowing_sub(self.reg_v[x]);
				self.reg_v[x] = result;
				self.reg_v[0xF] = if borow { 0 } else { 1 };
			}
			(0x8, _, _, 0xE) => {
				self.reg_v[0xF] = if (self.reg_v[x] & 0x8) == 0x8 { 1 } else { 0 };
				self.reg_v[x] <<= 1;
			}
			(0x9, _, _, 0) => {
				if self.reg_v[x] != self.reg_v[y] {
					self.pc += 2;
				}
			}
			(0xA, _, _, _) => {
				self.reg_i = addr;
			}
			(0xB, _, _, _) => {
				self.pc = addr + self.reg_v[0x0] as usize;
			}
			(0xC, _, _, _) => {
				self.reg_v[x] = self.rng.gen::<u8>() & byte;
			}
			(0xD, _, _, _) => {
				for i in 0..nibble {
					if self.display.draw_line_at(
						self.memory[self.reg_i + i],
						self.reg_v[x],
						self.reg_v[y] + i as u8,
					) {
						self.reg_v[0xF] = 1;
					}
				}
			}
			(0xE, _, 0x9, 0xE) => {
				if self.keyboard.is_key_down(self.reg_v[x]) {
					self.pc += 2;
				}
			}
			(0xE, _, 0xA, 0x1) => {
				if !self.keyboard.is_key_down(self.reg_v[x]) {
					self.pc += 2;
				}
			}
			(0xF, _, 0x0, 0x7) => {
				self.reg_v[x] = self.delay_t;
			}
			(0xF, _, 0x0, 0xA) => {
				let mut key_pressed = false;
				for key in 0..0x10 {
					if self.keyboard.is_key_down(key) {
						key_pressed = true;
						self.reg_v[x] = key;
					}
				}
				if !key_pressed {
					self.pc -= 2;
				}
			}
			(0xF, _, 0x1, 0x5) => {
				self.delay_t = self.reg_v[x];
			}
			(0xF, _, 0x1, 0x8) => {
				self.delay_s = self.reg_v[x];
			}
			(0xF, _, 0x1, 0xE) => {
				self.reg_i += usize::from(self.reg_v[x]);
			}
			(0xF, _, 0x2, 0x9) => {
				self.reg_i = usize::from(self.reg_v[x]) * 5;
			}
			(0xF, _, 0x3, 0x3) => {
				let val = self.reg_v[x];
				self.memory[self.reg_i] = val / 100;
				self.memory[self.reg_i + 1] = (val / 10) % 10;
				self.memory[self.reg_i + 2] = val % 10;
			}
			(0xF, _, 0x5, 0x5) => {
				self.memory[self.reg_i..=self.reg_i + x as usize]
					.copy_from_slice(&self.reg_v[0..=x as usize]);
			}
			(0xF, _, 0x6, 0x5) => {
				self.reg_v[0..=x as usize].copy_from_slice(
					&self.memory[self.reg_i..=self.reg_i + x as usize],
				);
			}
			_ => unreachable!(),
		};
	}

	fn read_word(&self, index: usize) -> u16 {
		u16::from(self.memory[index]) << 8 | u16::from(self.memory[index + 1])
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new() {
		let proc = Cpu::new();
		assert_eq!(proc.pc, 0x200);
	}

	#[test]
	fn load_at() {
		let mut proc = Cpu::new();
		proc.load_at(&[1, 2, 3, 4], 2);
		assert_eq!(proc.memory[0..6], [0, 0, 1, 2, 3, 4])
	}

	#[test]
	fn cycle() {
		let mut proc = Cpu::new();
		let pc_before = proc.pc;
		proc.memory[0x200] = 0x60; // instruction that don't jump to test pc
		proc.cycle();
		assert_eq!(pc_before + 2, proc.pc);
	}

	#[test]
	fn read_word() {
		let mut proc = Cpu::new();
		proc.memory[proc.pc] = 0xFF;
		proc.memory[proc.pc + 1] = 0xEE;
		assert_eq!(proc.read_word(proc.pc), 0xFFEE);
	}

	#[test]
	fn cls() {
		let mut proc = Cpu::new();
		proc.display.draw_line_at(0xFF, 0, 0);
		let before = proc.display.dump();
		proc.display.cls();
		let after = proc.display.dump();
		assert_ne!(before[..], after[..]);
	}

	#[test]
	fn ret() {
		let mut proc = Cpu::new();
		proc.sp = 1;
		proc.stack[0] = 0x200;
		proc.pc = 0x300;
		let before = proc.pc;
		proc.exec_op_code(0x00EE);
		let after = proc.pc;
		assert_ne!(before, after);
		assert_eq!(proc.sp, 0x0);
	}

	#[test]
	fn jp_addr() {
		let mut proc = Cpu::new();
		proc.exec_op_code(0x1523);
		assert_eq!(proc.pc, 0x523);
	}

	#[test]
	fn call_addr() {
		let mut proc = Cpu::new();
		let before = proc.pc;
		proc.exec_op_code(0x2523);
		assert_eq!(proc.pc, 0x523);
		assert_eq!(proc.stack[proc.sp - 1], before);
	}

	#[test]
	fn se_vx_byte() {
		let mut proc = Cpu::new();
		proc.reg_v[0x0] = 0xF;
		let before = proc.pc;
		proc.exec_op_code(0x300F);
		let after_1 = proc.pc;
		proc.exec_op_code(0x300E);
		let after_2 = proc.pc;
		assert_eq!(before + 2, after_1);
		assert_eq!(after_1, after_2);
	}

	#[test]
	fn sne_vx_byte() {
		let mut proc = Cpu::new();
		proc.reg_v[0x0] = 0xF;
		let before = proc.pc;
		proc.exec_op_code(0x400F);
		let after_1 = proc.pc;
		proc.exec_op_code(0x400E);
		let after_2 = proc.pc;
		assert_eq!(before, after_1);
		assert_eq!(after_1 + 2, after_2);
	}

	#[test]
	fn se_vx_vy() {
		let mut proc = Cpu::new();
		proc.reg_v[0x0] = 0xF;
		proc.reg_v[0x1] = 0xE;
		let before = proc.pc;
		proc.exec_op_code(0x5010);
		let after_1 = proc.pc;
		proc.reg_v[0x1] = 0xF;
		proc.exec_op_code(0x5010);
		let after_2 = proc.pc;
		assert_eq!(before, after_1);
		assert_eq!(after_1 + 2, after_2);
	}

	#[test]
	fn ld_vx_byte() {
		let mut proc = Cpu::new();
		proc.exec_op_code(0x60FF);
		assert_eq!(proc.reg_v[0x0], 0xFF);
	}

	#[test]
	fn add_vx_byte() {
		let mut proc = Cpu::new();
		proc.reg_v[0x0] = 0xE;
		proc.exec_op_code(0x7001);
		assert_eq!(proc.reg_v[0x0], 0xF);
		proc.exec_op_code(0x7002);
		assert_eq!(proc.reg_v[0x0], 0x1);
	}

	//TODO: TEST all operations
}
