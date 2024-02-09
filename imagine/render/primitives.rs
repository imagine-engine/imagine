/*******************************************************************************
  primitives.rs
********************************************************************************
  Copyright 2024 Menelik Eyasu

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.
*******************************************************************************/

use wgpu::util::DeviceExt;
use crate::render::RenderContext;
use crate::animation::{Animation, AnimationUpdate};
use nalgebra::{Vector2, Vector3, Matrix3, Matrix4};

pub struct Model {
  pub size: u32,
  pub material: String,
  pub vertex_buffer: wgpu::Buffer,
  pub index_buffer: wgpu::Buffer
}

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

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FillConfigUniform {
  pub clear: [f32; 4],
  pub resolution: [f32; 2],
  pub _padding: [f32; 2],
  pub view: [[f32; 4]; 3]
}

impl FillConfigUniform {
  pub fn default() -> Self {
    Self {
      clear: [0.0, 0.0, 0.0, 1.0],
      resolution: [1920.0, 1080.0],
      _padding: [0.0, 0.0],
      view: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0]
      ]
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

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Zeroable)]
pub struct PathSegment {
  pub start: [f32; 2],
  pub end: [f32; 2]
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GaussianVertex {
  pub position: [f32; 3]
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
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GaussianUniform {
}

impl GaussianUniform {
  pub fn default() -> Self {
    Self {}
  }
}

pub struct Texture {
  pub texture: wgpu::Texture,
  pub view: wgpu::TextureView,
  pub sampler: wgpu::Sampler
}

pub struct Material2D {
  pub opacity: f32,
  pub offset: f32,
  pub color: Texture,
  // pub config: wgpu::Buffer,
  pub binding: wgpu::BindGroup
}

pub struct PhongMaterial {
  pub opacity: f32,
  pub shininess: f32,
  pub reflectivity: f32,
  pub normal: Texture,
  pub diffuse: Texture,
  pub specular: Texture,
  pub binding: wgpu::BindGroup
}

pub struct PBRMaterial {
  pub albedo: Texture,
  pub normal: Texture,
  pub specular: Texture,
  pub metallic: Texture,
  pub roughness: Texture,
  pub binding: wgpu::BindGroup
}

// impl PBRMaterial {
//   pub fn bind_layout() -> wgpu::BindGroupLayoutDescriptor<'static> {
//     wgpu::BindGroupLayoutDescriptor {
//       label: None,
//       entries: &[]
//     }
//   }
// }

impl PhongMaterial {
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

impl Material2D {
  pub fn bind_layout() -> wgpu::BindGroupLayoutDescriptor<'static> {
    wgpu::BindGroupLayoutDescriptor {
      label: None,
      entries: &[
        // config - opacity, frame index, frame size
        wgpu::BindGroupLayoutEntry {
          binding: 0,
          count: None,
          visibility: wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None
          }
        },
        wgpu::BindGroupLayoutEntry {
          binding: 1,
          count: None,
          visibility: wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Texture {
            multisampled: false,
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: wgpu::TextureViewDimension::D2
          }
        },
        wgpu::BindGroupLayoutEntry {
          binding: 2,
          count: None,
          visibility: wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering)
        }
      ]
    }
  }
}

pub enum ModelMaterial {
  Phong(PhongMaterial),
  Physical(PBRMaterial),
  Material2D(Material2D)
}

impl ModelMaterial {
  pub fn binding(&self) -> &wgpu::BindGroup {
    match self {
      ModelMaterial::Phong(phong) => &phong.binding,
      ModelMaterial::Physical(physical) => &physical.binding,
      ModelMaterial::Material2D(material) => &material.binding
    }
  }

  pub fn new_material2d(
    context: &RenderContext,
    opacity: f32,
    offset: f32,
    color: Texture
  ) -> ModelMaterial {
    let buffer = context.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: None,
      contents: bytemuck::cast_slice(&[[0.5, 0.0, 0.0, 0.0]]),
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
    });

    let binding = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
      label: None,
      layout: &context.m2_layout,
      entries: &[
        wgpu::BindGroupEntry {
          binding: 0,
          resource: buffer.as_entire_binding()
        },
        wgpu::BindGroupEntry {
          binding: 1,
          resource: wgpu::BindingResource::TextureView(&color.view),
        },
        wgpu::BindGroupEntry {
          binding: 2,
          resource: wgpu::BindingResource::Sampler(&color.sampler),
        }
      ]
    });

    ModelMaterial::Material2D(Material2D {
      opacity,
      offset,
      color,
      binding
    })
  }

  pub fn new_phong(
    context: &RenderContext,
    opacity: f32,
    shininess: f32,
    reflectivity: f32,
    normal: Texture,
    diffuse: Texture,
    specular: Texture
  ) -> Self {
    let binding = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
      label: None,
      layout: &context.phong_layout,
      entries: &[
        // normal map
        wgpu::BindGroupEntry {
          binding: 0,
          resource: wgpu::BindingResource::TextureView(&normal.view),
        },
        wgpu::BindGroupEntry {
          binding: 1,
          resource: wgpu::BindingResource::Sampler(&normal.sampler),
        },
        // diffuse map
        wgpu::BindGroupEntry {
          binding: 2,
          resource: wgpu::BindingResource::TextureView(&diffuse.view),
        },
        wgpu::BindGroupEntry {
          binding: 3,
          resource: wgpu::BindingResource::Sampler(&diffuse.sampler),
        },
        // specular map
        wgpu::BindGroupEntry {
          binding: 4,
          resource: wgpu::BindingResource::TextureView(&specular.view),
        },
        wgpu::BindGroupEntry {
          binding: 5,
          resource: wgpu::BindingResource::Sampler(&specular.sampler),
        }
      ]
    });

    ModelMaterial::Phong(PhongMaterial {
      opacity,
      shininess,
      reflectivity,
      normal,
      diffuse,
      specular,
      binding
    })
  }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PathUniform {
  pub opacity: f32,
  pub segments: u32,
  pub _padding: [f32; 2],
  pub color: [f32; 4],
  pub bounds: [f32; 4],
  pub transform: [[f32; 4]; 3]
}

pub struct PathConfig {
  pub opacity: f32,
  pub bounds: [f32; 4],
  pub path_segments: usize,
  pub scale: Vector2<f32>,
  pub position: Vector2<f32>,
  pub rotation: f32,
  pub transform: Matrix3<f32>
}

pub struct Object3D {
  pub material: String,
  pub vertices: Vec<Vertex3D>,
  pub indices: Vec<u32>,
  pub scale: Vector3<f32>,
  pub position: Vector3<f32>,
  pub rotation: Vector3<f32>,
  pub transform: Matrix4<f32>
}

impl Object3D {
  pub fn uniform(&self) -> Uniform3D {
    Uniform3D {
      transform: self.transform.into(),
      normal_matrix: Matrix4::identity().into()
    }
  }

  pub fn model(&self, device: &wgpu::Device) -> Model {
    Model {
      material: self.material.clone(),
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

pub enum CameraProjection {
  Perspective(f32, f32, f32),
  Orthographic(f32, f32, f32, f32, f32, f32),
  // Panoramic(???)
}

pub struct Camera3D {
  pub aspect: f32,
  pub scale: Vector3<f32>,
  pub position: Vector3<f32>,
  pub rotation: Vector3<f32>,
  pub view: Matrix4<f32>,
  pub projection: Matrix4<f32>,
  pub config: CameraProjection
}

pub struct Camera2D {
  pub aspect: f32,
  pub rotation: f32,
  pub scale: Vector2<f32>,
  pub position: Vector2<f32>,
  pub view: Matrix3<f32>,
}

impl Camera3D {
  pub fn default() -> Self {
    Self {
      aspect: 16.0/9.0,
      scale: Vector3::new(1.0, 1.0, 1.0),
      position: Vector3::new(0.0, 0.0, 5.0),
      rotation: Vector3::new(0.0, 0.0, 0.0),
      view: Matrix4::new_translation(&Vector3::new(0.0, 0.0, -5.0)),
      projection: Matrix4::new_perspective(16.0/9.0, 45.0, 1.0, 1000.0),
      config: CameraProjection::Perspective(45.0, 1.0, 1000.0)
    }
  }

  pub fn uniform(&self) -> Camera3DUniform {
    Camera3DUniform {
      view: self.view.into(),
      projection: self.projection.into()
    }
  }
}

impl Camera2D {
  pub fn default() -> Self {
    Self {
      aspect: 16.0/9.0,
      rotation: 0.0,
      scale: Vector2::new(100.0, 100.0),
      position: Vector2::new(0.0, 0.0),
      view: Matrix3::new_nonuniform_scaling(&Vector2::new(100.0, 100.0))
    }
  }
}

pub struct WorldLight {
  pub position: Vector3<f32>
}