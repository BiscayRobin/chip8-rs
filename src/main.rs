mod cpu;
mod font;

use clap::{App, Arg};
use cpu::Cpu;
use std::fs;

fn main() {
	let matches = App::new("chip8-rs")
		.version("0.0.1")
		.author("Robin Biscay <biscay.rob@gmail.com>")
		.arg(Arg::with_name("file")
			.short("f")
			.long("file")
			.takes_value(true)
			.help("rom to load"))
		.get_matches();

	let rom_file = matches.value_of("file").unwrap_or("c8games/15PUZZLE");

	let rom = fs::read(rom_file).expect("could not read file");

	let mut processor = Cpu::new();

	processor.load_at(&font::FONT, 0);
	processor.load_at(&rom, 0x200);
}
