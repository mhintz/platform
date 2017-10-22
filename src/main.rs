// -- External crates

#[macro_use]
extern crate error_chain;
extern crate itertools;
extern crate rand;
extern crate threadpool;

extern crate nalgebra;
extern crate nphysics3d;

extern crate sdl2;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_sdl;

// -- App modules

mod platform;
mod errors;
mod standard_pipeline;
mod mesh;

// -- App framework

use platform::*;
use errors::*;

fn main() {
	if let Err(ref e) = run() {
		use std::io::Write;
		use error_chain::ChainedError;

		writeln!(std::io::stderr(), "{}", e.display_chain()).expect("Error writing to stderr");
		::std::process::exit(1);
	}
}

fn run() -> Result<()> {
	let mut app = Platform::init().chain_err(|| "Initialization Error")?;

	loop {
		match app.update().chain_err(|| "Update Error")? {
			AppCmd::Continue => (),
			AppCmd::Exit => break,
		}

		app.draw().chain_err(|| "Draw Error")?;
	}

	app.deinit().chain_err(|| "Deinitialization Error")?;

	Ok(())
}
