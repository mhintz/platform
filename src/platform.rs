use errors::*;
use sdl2;
use gfx;
use gfx_window_sdl;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::Depth;

pub struct Platform {
    pub sdl_context: sdl2::Sdl,
    pub window: sdl2::video::Window,
    pub gl_context: sdl2::video::GLContext,
    pub device: gfx_window_sdl::Device,
    pub factory: gfx_window_sdl::Factory,
    pub color_view: gfx::handle::RenderTargetView<gfx_window_sdl::Resources, ColorFormat>,
    pub depth_view: gfx::handle::DepthStencilView<gfx_window_sdl::Resources, DepthFormat>,
}

impl Platform {
    pub fn init() -> Result<Platform> {
        let sdl_context = sdl2::init()?;
        let video = sdl_context.video()?;

        let mut builder = video.window("Platform", 800, 600);
        let (mut window, mut gl_context, mut device, mut factory, color_view, depth_view) = match gfx_window_sdl::init::<ColorFormat, DepthFormat>(builder) {
            Err(err) => bail!(ErrorKind::from(err)),
            Ok(x) => x,
        };

        let instance = Platform {
            sdl_context: sdl_context,
            window: window,
            gl_context: gl_context,
            device: device,
            factory: factory,
            color_view: color_view,
            depth_view: depth_view,
        };

        Ok(instance)
    }

    pub fn deinit(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn update(&mut self) -> Result<AppCmd> {
        let mut event_pump = self.sdl_context.event_pump()?;

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => return Ok(AppCmd::Exit),
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => return Ok(AppCmd::Exit),
                _ => (),
            }
        }

        Ok(AppCmd::Continue)
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
