use wgpu::util::DeviceExt;

use nalgebra::{Vector2, Vector3, Matrix3, Matrix4};

use crate::render3d::resources::Model;
use crate::render::primitives::Texture;
use crate::render3d::primitives::Vertex3D;

pub struct Transform3DComponent {
  pub scale: Vector3<f32>,
  pub position: Vector3<f32>,
  pub rotation: Vector3<f32>,
  pub transform: Matrix4<f32>
}

pub struct MeshComponent {
  pub vertices: Vec<Vertex3D>,
  pub indices: Vec<u32>
}

pub struct PhongComponent {
  pub opacity: f32,
  pub shininess: f32,
  pub reflectivity: f32,
  pub normal: Texture,
  pub diffuse: Texture,
  pub specular: Texture,
  pub binding: wgpu::BindGroup
}

impl PhongComponent {
  pub fn bind_layout() -> wgpu::BindGroupLayoutDescriptor<'static> {
    wgpu::BindGroupLayoutDescriptor {
      label: None,
      entries: &[
        // normal map
        wgpu::BindGroupLayoutEntry {
          binding: 0,
          count: None,
          visibility: wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Texture {
            multisampled: false,
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: wgpu::TextureViewDimension::D2
          }
        },
        wgpu::BindGroupLayoutEntry {
          binding: 1,
          count: None,
          visibility: wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering)
        },
        // diffuse map
        wgpu::BindGroupLayoutEntry {
          binding: 2,
          count: None,
          visibility: wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Texture {
            multisampled: false,
            view_dimension: wgpu::TextureViewDimension::D2,
            sample_type: wgpu::TextureSampleType::Float { filterable: true }
          }
        },
        wgpu::BindGroupLayoutEntry {
          binding: 3,
          count: None,
          visibility: wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering)
        },
        // specular map
        wgpu::BindGroupLayoutEntry {
          binding: 4,
          count: None,
          visibility: wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Texture {
            multisampled: false,
            view_dimension: wgpu::TextureViewDimension::D2,
            sample_type: wgpu::TextureSampleType::Float { filterable: true }
          }
        },
        wgpu::BindGroupLayoutEntry {
          binding: 5,
          count: None,
          visibility: wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering)
        }
      ]
    }
  }
}

pub struct PBRComponent {
  pub albedo: Texture,
  pub normal: Texture,
  pub specular: Texture,
  pub metallic: Texture,
  pub roughness: Texture,
  pub binding: wgpu::BindGroup
}

pub struct PerspectiveCameraComponent {
  pub aspect: f32,
  pub fov: f32,
  pub znear: f32,
  pub zfar: f32,
  projection: Matrix4<f32>
}

// pub struct OrthoCameraComponent {
//   pub aspect: f32,
//   pub fov: f32,
//   pub znear: f32,
//   pub zfar: f32,
//   pub projection: Matrix4<f32>
// }

impl Default for PerspectiveCameraComponent {
  fn default() -> Self {
    Self {
      aspect: 16.0 / 9.0,
      fov: 45.0,
      znear: 1.0,
      zfar: 1000.0,
      projection: Matrix4::new_perspective(16.0/9.0, 45.0, 1.0, 1000.0)
    }
  }
}

impl Default for Transform3DComponent {
  fn default() -> Self {
    Self {
      scale: Vector3::new(1.0, 1.0, 1.0),
      position: Vector3::new(0.0, 0.0, 0.0),
      rotation: Vector3::new(0.0, 0.0, 0.0),
      transform: Matrix4::identity()
    }
  }
}

impl Transform3DComponent {
  pub fn new(scale: Vector3<f32>, position: Vector3<f32>, rotation: Vector3<f32>) -> Self {
    let mut transform = Self {
      scale,
      position,
      rotation,
      transform: Matrix4::identity()
    };
    transform.sync();

    transform
  }

  pub fn sync(&mut self) {
    let scale = Matrix4::new_nonuniform_scaling(&self.scale);
    let position = Matrix4::new_translation(&self.position);
    let rotation = Matrix4::from_euler_angles(
      self.rotation.x,
      self.rotation.y,
      self.rotation.z
    );

    self.transform = scale * position * rotation;
  }
}

impl MeshComponent {
  pub fn model(&self, device: &wgpu::Device) -> Model {
    Model {
      size: self.indices.len() as u32,
      vertex_buffer: device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
          label: None,
          contents: bytemuck::cast_slice(&self.vertices),
          usage: wgpu::BufferUsages::VERTEX
        }
      ),
      index_buffer: device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
          label: None,
          contents: bytemuck::cast_slice(&self.indices),
          usage: wgpu::BufferUsages::INDEX
        }
      )
    }
  }
}