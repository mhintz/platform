#[macro_use]
extern crate error_chain;
extern crate itertools;
extern crate rand;
extern crate threadpool;

extern crate sdl2;
extern crate gfx;
extern crate gfx_window_sdl;
extern crate nalgebra;
extern crate nphysics3d;

mod platform;
use platform::*;

#[allow(dead_code)]
mod errors {
	use gfx_window_sdl;

	error_chain!{
		foreign_links {
			// Error(gfx_window_sdl::InitError);
		}

		errors {
			Init(t: &'static str) {
				description("Initialization Error")
				display("Error during initialization: {}", t)
			}

			Update(t: &'static str) {
				description("Update Error")
				display("Error encountered while updating: {}", t)
			}

			Draw(t: &'static str) {
				description("Draw Error")
				display("Error encountered while drawing: {}", t)
			}
		}
	}
}

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
	let mut app = Platform::new();

	app.init().chain_err(|| "Initialization Error")?;

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
