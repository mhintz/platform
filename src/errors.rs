#![allow(dead_code)]

use gfx_window_sdl;
use gfx;

error_chain!{
	foreign_links {
		PipelineStateError(gfx::PipelineStateError<String>);
	}

	errors {
		Init(t: String) {
			description("Initialization Error")
			display("Error during initialization: {}", t)
		}

		Update(t: String) {
			description("Update Error")
			display("Error encountered while updating: {}", t)
		}

		Draw(t: String) {
			description("Draw Error")
			display("Error encountered while drawing: {}", t)
		}
	}
}

impl From<gfx_window_sdl::InitError> for ErrorKind {
	fn from(err: gfx_window_sdl::InitError) -> ErrorKind {
		match err {
			gfx_window_sdl::InitError::PixelFormatUnsupportedError => ErrorKind::Init(format!("Pixel Format Unsupported")),
			gfx_window_sdl::InitError::WindowBuildError(err) => ErrorKind::Init(format!("Unable to Build Window, Error: {}", err)),
			gfx_window_sdl::InitError::SdlError(err_msg) => ErrorKind::Init(format!("SDL Init Error: {}", err_msg)),
		}
	}
}

impl From<gfx_window_sdl::InitError> for Error {
	fn from(err: gfx_window_sdl::InitError) -> Error {
		Error::from(ErrorKind::from(err))
	}
}

// impl From<gfx::PipelineStateError<String>> for ErrorKind {
// 	fn from(err: gfx::PipelineStateError) -> ErrorKind {
// 		ErrorKind::Draw(format!("GFX Pipeline State Error: {:?}"))
// 	}
// }
