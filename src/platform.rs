use errors::*;
use sdl2;
use gfx;
use gfx_window_sdl;

pub type ColorFormat = gfx::format::Rgba32F;
pub type DepthFormat = gfx::format::Depth;

pub struct Platform {

}

impl Platform {
    pub fn new() -> Platform {
        Platform {}
    }

    pub fn init(&mut self) -> Result<()> {
        let sdl_context = sdl2::init()?;
        let video = sdl_context.video()?;

        let mut builder = video.window("Platform", 800, 600);

        let (mut window, mut gl_context, mut device, mut factory, color_view, depth_view) = match gfx_window_sdl::init::<ColorFormat, DepthFormat>(builder) {
            Err(err) => return Err(ErrorKind::Init("Initialization Error").into()),
            Ok(x) => x,
        };

        Ok(())
    }

    pub fn deinit(&mut self) -> Result<()> {

        Ok(())
    }

    pub fn update(&mut self) -> Result<AppCmd> {
        Ok(AppCmd::Exit)
    }

    pub fn draw(& self) -> Result<AppCmd> {
        Ok(AppCmd::Continue)
    }
}

#[derive(Debug)]
pub enum AppCmd {
    Continue,
    Exit,
}
