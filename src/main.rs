mod bits;
mod cpu;
mod display;
mod font;
mod keyboard;

use clap::{App, Arg};
use cpu::Cpu;
use minifb::{Key, Window, WindowOptions};
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

	let rom_file = matches.value_of("file").unwrap_or("c8games/PONG");

	let rom = fs::read(rom_file).expect("could not read file");

	let mut processor = Cpu::new();

	processor.load_at(&font::FONT, 0);
	processor.load_at(&rom, 0x200);
	let mut window = Window::new("chip8-rs | ESC to exit", 640, 320, WindowOptions::default())
		.unwrap_or_else(|e| {
			panic!("{}", e);
		});

	while window.is_open() && !window.is_key_down(Key::Escape) {
		processor.update_keys(&window);

		window.update_with_buffer(&processor.get_minifb_buffer())
			.unwrap();
		processor.cycle();
	}
}
