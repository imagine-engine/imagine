pub struct Model {
  pub size: u32,
  pub vertex_buffer: wgpu::Buffer,
  pub index_buffer: wgpu::Buffer
}

pub struct Batch {
  pub buffer: wgpu::Buffer,
  pub binding: wgpu::BindGroup,
  pub models: Vec<Model>
}

pub struct Uniform {
  pub buffer: wgpu::Buffer,
  pub binding: wgpu::BindGroup
}