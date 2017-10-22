#![allow(unused)]

use gfx;
use platform::{ColorFormat, DepthFormat};

gfx_defines! {
    // #[derive(Default)]
    vertex Vertex {
        pos: [f32; 3] = "a_position",
        color: [f32; 4] = "a_color",
        normal: [f32; 3] = "a_normal",
        tex_coord0: [f32; 2] = "a_tex_coord0",
    }

    pipeline standard_pipeline {
        vertex_buffer: gfx::VertexBuffer<Vertex> = (),
        // projection_mat: gfx::Global<[[f32; 4]; 4]> = "uProjectionMatrix",
        // view_mat: gfx::Global<[[f32; 4]; 4]> = "uViewMatrix",
        // model_mat: gfx::Global<[[f32; 4]; 4]> = "uModelMatrix",
        // normal_mat: gfx::Global<[[f32; 3]; 3]> = "uNormalMatrix",
        out_color: gfx::RenderTarget<ColorFormat> = "Target0",
        // out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl Vertex {
  pub fn new(pos: & [f32; 3], norm: & [f32; 3], color: & [f32; 4], tex: & [f32; 2]) -> Vertex {
    Vertex {
      color: *color,
      pos: *pos,
      normal: *norm,
      tex_coord0: *tex,
    }
  }

  pub fn pos_only(pos: & [f32; 3]) -> Vertex {
    Vertex {
      color: [0.0; 4],
      pos: *pos,
      normal: [0.0; 3],
      tex_coord0: [0.0; 2],
    }
  }

  pub fn pos_and_normal(pos: & [f32; 3], normal: & [f32; 3]) -> Vertex {
      Vertex {
          color: [0.0; 4],
          pos: *pos,
          normal: *normal,
          tex_coord0: [0.0; 2],
      }
  }

  pub fn pos_and_color(pos: [f32; 3], color: [f32; 4]) -> Vertex {
    Vertex {
      color: color,
      pos: pos,
      normal: [0.0; 3],
      tex_coord0: [0.0; 2],
    }
  }

  pub fn pos_and_tex(pos: & [f32; 3], tex: & [f32; 2]) -> Vertex {
    Vertex {
      color: [0.0; 4],
      pos: *pos,
      normal: [0.0; 3],
      tex_coord0: *tex,
    }
  }

  // pub fn from(pos: Point3<f32>, norm: Vector3<f32>, color: Vector4<f32>, tex: Vector2<f32>) -> Vertex {
  //   Vertex {
  //     color: color.into(),
  //     pos: pos.into(),
  //     normal: norm.into(),
  //     tex_coord0: tex.into(),
  //   }
  // }

  // pub fn from_pos(pos: Point3<f32>) -> Vertex {
  //   Vertex {
  //     color: [0.0; 4],
  //     pos: pos.into(),
  //     normal: [0.0; 3],
  //     tex_coord0: [0.0; 2],
  //   }
  // }
  //
  // pub fn from_pos_and_color(pos: Point3<f32>, color: Vector4<f32>) -> Vertex {
  //   Vertex {
  //     color: color.into(),
  //     pos: pos.into(),
  //     normal: [0.0; 3],
  //     tex_coord0: [0.0; 2],
  //   }
  // }
  //
  // pub fn from_pos_and_tex(pos: Point3<f32>, tex: Vector2<f32>) -> Vertex {
  //   Vertex {
  //     color: [0.0; 4],
  //     pos: pos.into(),
  //     normal: [0.0; 3],
  //     tex_coord0: tex.into(),
  //   }
  // }

  // pub fn pos(&self) -> Point3<f32> { Point3::from(self.pos) }

  // pub fn color(&self) -> Vector4<f32> { Vector4::from(self.color) }

  // pub fn normal(&self) -> Vector3<f32> { Vector3::from(self.normal) }

  // pub fn tex(&self) -> Vector2<f32> { Vector2::from(self.tex_coord0) }

  // pub fn set_pos(&mut self, pos: Point3<f32>) { self.pos = pos.into() }

  // pub fn set_color(&mut self, color: Vector4<f32>) { self.color = color.into() }

  // pub fn set_normal(&mut self, norm: Vector3<f32>) { self.normal = norm.into() }

  // pub fn normalize_normal(&mut self) { self.normal = Vector3::from(self.normal).normalize().into() }

  // pub fn set_tex(&mut self, tex: Vector2<f32>) { self.tex_coord0 = tex.into() }
}
