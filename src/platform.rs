use errors::*;
use gfx;
use gfx_window_sdl;
use gfx_device_gl;
use sdl2;

use gfx::traits::*;

use mesh::*;
use basic_pipeline::*;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub struct Platform {
    pub sdl_context: sdl2::Sdl,
    pub sdl_video_context: sdl2::VideoSubsystem,
    pub main_window: sdl2::video::Window,
    pub gl_context: sdl2::video::GLContext,
    pub device: gfx_device_gl::Device,
    pub factory: gfx_device_gl::Factory,
    pub color_view: gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>,
    pub depth_view: gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>,
    pub tri_mesh: [Vertex; 3],
}

const CLEAR_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

impl Platform {
    pub fn init() -> Result<Platform> {
        let sdl_context = sdl2::init()?;
        let sdl_video_context = sdl_context.video()?;
        {
            let gl_attr = sdl_video_context.gl_attr();
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        }

        let main_builder = sdl_video_context.window("Platform", 800, 600);
        let (main_window, gl_context, device, factory, color_view, depth_view) = match gfx_window_sdl::init::<ColorFormat, DepthFormat>(main_builder) {
            Err(err) => bail!(ErrorKind::from(err)),
            Ok(w) => w,
        };

        let mut instance = Platform {
            sdl_context: sdl_context,
            sdl_video_context: sdl_video_context,
            main_window: main_window,
            gl_context: gl_context,
            device: device,
            factory: factory,
            color_view: color_view,
            depth_view: depth_view,
            tri_mesh: [
                Vertex::pos_and_color([-0.5, -0.5, 0.0], [1.0, 0.0, 0.0, 1.0]),
                Vertex::pos_and_color([0.5, -0.5, 0.0], [0.0, 1.0, 0.0, 1.0]),
                Vertex::pos_and_color([0.0, -0.5, 0.0], [0.0, 0.0, 1.0, 1.0])
            ],
        };

        Ok(instance)
    }

    pub fn deinit(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn update(&mut self) -> Result<AppCmd> {
        let mut event_pump = self.sdl_context.event_pump()?;

        for event in event_pump.poll_iter() {
            match process_event(event) {
                AppCmd::Exit => return Ok(AppCmd::Exit),
                AppCmd::Continue => ()
            }
        }

        Ok(AppCmd::Continue)
    }

    pub fn draw(&mut self) -> Result<AppCmd> {
        let mut encoder: gfx::Encoder<_, _> = self.factory.create_command_buffer().into();
        let pso = self.factory.create_pipeline_simple(
            include_bytes!("shaders/triangle_v.glsl"),
            include_bytes!("shaders/triangle_f.glsl"),
            basic_pipeline::new()
        ).unwrap();
        let (vertex_buffer, slice) = self.factory.create_vertex_buffer_with_slice(& self.tri_mesh, ());
        let data = basic_pipeline::Data {
            vertex_buffer: vertex_buffer,
            // projection_mat: identity_mat4(),
            // view_mat: identity_mat4(),
            // model_mat: identity_mat4(),
            // normal_mat: identity_mat3(),
            out_color: self.color_view.clone(),
            // out_depth: self.depth_view.clone(),
        };

        encoder.clear(& data.out_color, CLEAR_COLOR);
        encoder.draw(& slice, & pso, & data);
        encoder.flush(&mut self.device);
        self.main_window.gl_swap_window();
        self.device.cleanup();

        Ok(AppCmd::Continue)
    }
}

fn process_event(evt: sdl2::event::Event) -> AppCmd {
    match evt {
        sdl2::event::Event::Quit { .. } => AppCmd::Exit,
        sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => AppCmd::Exit,
        _ => AppCmd::Continue,
    }
}

pub fn identity_mat3() -> [[f32; 3]; 3] {
    [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0]
    ]
}

pub fn identity_mat4() -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ]
}

#[derive(Debug)]
pub enum AppCmd {
    Continue,
    Exit,
}
