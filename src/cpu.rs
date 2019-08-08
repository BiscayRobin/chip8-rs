use rand::{rngs::ThreadRng, thread_rng, Rng};

pub struct Cpu {
	sp: usize,
	stack: [usize; 16],
	pc: usize,
	reg_v: [u8; 16],
	reg_i: u16,
	delay_t: u8,
	delay_s: u8,
	memory: [u8; 4096],
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
			rng: thread_rng(),
		}
	}

	pub fn load_at(&mut self, bytes: &[u8], index: usize) {
		self.memory[index..index + bytes.len()].copy_from_slice(bytes);
	}

	pub fn cycle(&mut self) {
		self.pc += 2;
	}

	pub fn dump_memory(&self) {
		println!("{:?}", &self.memory[..]);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new() {
		let proc = Cpu::new();
		assert_eq!(proc.pc, 0x200);
	}

	#[test]
	fn test_load_at() {
		let mut proc = Cpu::new();
		proc.load_at(&[1, 2, 3, 4], 2);
		assert_eq!(proc.memory[0..6], [0, 0, 1, 2, 3, 4])
	}

	#[test]
	fn test_cycle() {
		let mut proc = Cpu::new();
		let pc_before = proc.pc;
		proc.cycle();
		assert_eq!(pc_before + 2, proc.pc);
	}
}
