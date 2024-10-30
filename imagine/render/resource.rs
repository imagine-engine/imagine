use crate::render::primitives::Texture;
use crate::render::primitives::{Model, ModelMaterial};

pub enum RenderResource {
  NotFound,
  Batch(wgpu::Buffer, wgpu::BindGroup, Vec<Model>),
  Layer(wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, wgpu::BindGroup),
  Buffer(wgpu::Buffer),
  Material(ModelMaterial),
  Texture(Texture),
  Uniform {
    buffer: wgpu::Buffer,
    binding: wgpu::BindGroup
  }
}