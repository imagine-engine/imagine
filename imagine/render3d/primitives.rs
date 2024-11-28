use wgpu::util::DeviceExt;
use nalgebra::Matrix4;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Camera3DUniform {
  pub view: [[f32; 4]; 4],
  pub projection: [[f32; 4]; 4]
}

impl Camera3DUniform {
  pub fn default() -> Self {
    Self {
      view: Matrix4::identity().into(),
      projection: Matrix4::identity().into()
    }
  }
}

#[repr(C, align(256))]
#[derive(Copy, Clone, bytemuck::Zeroable)]
pub struct Uniform3D {
  pub transform: [[f32; 4]; 4],
  pub normal_matrix: [[f32; 4]; 4]
}

impl Uniform3D {
  pub fn default() -> Self {
    Self {
      transform: Matrix4::identity().into(),
      normal_matrix: Matrix4::identity().into()
    }
  }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex3D {
  pub position: [f32; 3],
  pub normal: [f32; 3],
  pub uv: [f32; 2]
}

impl Vertex3D {
  pub fn desc() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
      array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &[
        wgpu::VertexAttribute {
          offset: 0,
          shader_location: 0,
          format: wgpu::VertexFormat::Float32x3
        },
        wgpu::VertexAttribute {
          offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
          shader_location: 1,
          format: wgpu::VertexFormat::Float32x3
        },
        wgpu::VertexAttribute {
          offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
          shader_location: 2,
          format: wgpu::VertexFormat::Float32x2
        }
      ]
    }
  }
}