# chip8-rs

This a basic emulator of chip8 processor. It is part of a project where i would like to make a full environnement for this processor. It will include this emulator, an assembler, a disassembler and finally a higher level language with his compiler.

## Why this project

- learning rust
- deal with low level details
- get feedback
- good documentation about chip 8

## Install

I'm currently working on CI for easier time at installing it but for now you have to follow those steps:

### Install rust:
All information [here](https://www.rust-lang.org/tools/install)

### clone or download the repository
* clone: `git clone git@github.com:BiscayRobin/chip8-rs.git`

### build
 ```BASH
 cd chip8-rs
 cargo build --release
 ```

### run

 ```BASH
 target/release/chip8-rs --help
 target/release/chip8-rs -f <rom_path>
 ```
 roms are available in c8games directory.

 `target/release/chip8-rs -f c8games/MERLIN`

## Usage

### keys
keyboard from machine using chip8:

| 7 		| 8	 	| 9	 	| C	 	|
| :---:		| :---:		| :---:		| :---:		|
| **4** 	| **5** 	| **6** 	| **D** 	|
| **1** 	| **2** 	| **3** 	| **E** 	|
| **A** 	| **0** 	| **B** 	| **F** 	|

keys used by my emulator:

| F1 		| F2	 	| F3	 	| F4	 	|
| :---:		| :---:		| :---:		| :---:		|
| **A** 	| **Z** 	| **E** 	| **R** 	|
| **Q** 	| **S** 	| **D** 	| **F** 	|
| **W** 	| **X** 	| **C** 	| **V** 	|

### games

#### PONG
	F1 -> go up left player
	A -> go down left player
	F4 -> go up right player
	R -> go down right player
#### MERLIN
	A -> upper left pad
	Z -> upper right pad
	Q -> lower left pad
	S -> lower right pad

#### MAZE
	NO INPUT

#### KALEID
from upper left sight:

	F2 -> go up
	A -> go left
	E -> go right
	S -> go down
#### CONNECT4
	A -> move left
	Z -> pop something
	E -> move right

## THANKS TO
[https://www.zophar.net/pdroms/chip8.html](https://www.zophar.net/pdroms/chip8.html)

[http://devernay.free.fr/hacks/chip8/C8TECH10.HTM](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)

[https://en.wikipedia.org/wiki/CHIP-8](https://en.wikipedia.org/wiki/CHIP-8)
