// use wgpu::util::DeviceExt;
use std::any::Any;
use image::GenericImageView;
use std::collections::HashMap;
use crate::render::primitives::Texture;

pub struct RenderContext {
  pub queue: wgpu::Queue,
  pub device: wgpu::Device,
  pub size: wgpu::Extent3d,
  pub max_vertices: u64,
  pub max_models: u64,
  // pub max_paths: u64,
  pub batch_size_2d: usize,
  pub resources: HashMap<String, Box<dyn Any + Send>>
}

impl RenderContext {
  pub async fn new() -> Self {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::all(),
      dx12_shader_compiler: Default::default()
    });

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
          power_preference: wgpu::PowerPreference::default(),
          compatible_surface: None,
          force_fallback_adapter: false
        })
        .await
        .unwrap();

    let (device, queue) = adapter.request_device(&Default::default(), None)
        .await
        .unwrap();

    Self {
      device,
      queue,
      max_vertices: 1000,
      max_models: 10_000,
      batch_size_2d: 1_000_000,
      // batch_size_2d: 10_000,
      // max_paths: 10000,
      size: wgpu::Extent3d {
        width: 1920,
        height: 1080,
        depth_or_array_layers: 1
      },
      resources: HashMap::new()
    }
  }

  pub fn add_resource<T: Any + Send>(&mut self, name: &str, resource: T) {
    if !self.resources.contains_key(name) {
      self.resources.insert(String::from(name), Box::new(resource));
    }
  }

  pub fn get<T: Any>(&self, name: &str) -> Option<&T> {
    if let Some(resource) = self.resources.get(name) {
      return resource.downcast_ref();
    }

    None
  }

  pub fn get_mut<T: Any>(&mut self, name: &str) -> Option<&mut T> {
    if let Some(resource) = self.resources.get_mut(name) {
      return resource.downcast_mut();
    }

    None
  }

  pub fn frame_size(&self) -> usize {
    (self.size.width * self.size.height) as usize
  }

  pub fn load_shader(&self, filename: &str, source: &str, shader_type: shaderc::ShaderKind) -> wgpu::ShaderModule {
    let shader_module = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: None,
      source: wgpu::util::make_spirv(
        shaderc::Compiler::new().unwrap().compile_into_spirv(
          source,
          shader_type,
          filename,
          "main",
          None
        ).unwrap().as_binary_u8()
      )
    });

    shader_module
  }

  pub fn create_texture(
    &self,
    format: wgpu::TextureFormat,
    usage: wgpu::TextureUsages
  ) -> Texture {
    let texture = self.device.create_texture(&wgpu::TextureDescriptor {
      label: None,
      size: self.size,
      mip_level_count: 1,
      sample_count: 1,
      format,
      usage,
      dimension: wgpu::TextureDimension::D2,
      view_formats: &[]
    });

    let view = texture.create_view(&Default::default());
    let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
      address_mode_u: wgpu::AddressMode::ClampToEdge,
      address_mode_v: wgpu::AddressMode::ClampToEdge,
      address_mode_w: wgpu::AddressMode::ClampToEdge,
      mag_filter: wgpu::FilterMode::Linear,
      min_filter: wgpu::FilterMode::Nearest,
      mipmap_filter: wgpu::FilterMode::Nearest,
      ..Default::default()
    });

    Texture {
      texture,
      view,
      sampler
    }
  }

  pub fn create_buffer<T>(
    &self,
    size: u64,
    usage: wgpu::BufferUsages
  ) -> wgpu::Buffer {
    self.device.create_buffer(&wgpu::BufferDescriptor {
      label: None,
      usage,
      size: size * std::mem::size_of::<T>() as u64,
      mapped_at_creation: false
    })
  }

  pub fn create_framebuffer(&self) -> wgpu::Buffer {
    self.create_buffer::<u32>(
      (self.size.width * self.size.height) as u64,
      wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ
    )
  }

  pub fn load_texture(
    &self,
    image: image::DynamicImage,
    format: wgpu::TextureFormat
  ) -> Texture {
    let dimensions = image.dimensions();
    let size = wgpu::Extent3d {
      width: dimensions.0,
      height: dimensions.1,
      depth_or_array_layers: 1
    };

    let texture = self.device.create_texture(&wgpu::TextureDescriptor {
      label: None,
      size,
      format,
      sample_count: 1,
      mip_level_count: 1,
      view_formats: &[],
      dimension: wgpu::TextureDimension::D2,
      usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST
    });

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
      address_mode_u: wgpu::AddressMode::ClampToEdge,
      address_mode_v: wgpu::AddressMode::ClampToEdge,
      address_mode_w: wgpu::AddressMode::ClampToEdge,
      mag_filter: wgpu::FilterMode::Linear,
      min_filter: wgpu::FilterMode::Nearest,
      mipmap_filter: wgpu::FilterMode::Nearest,
      ..Default::default()
    });

    // let texture = self.create_texture(
    //   format, wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST
    // );

    self.queue.write_texture(
      wgpu::ImageCopyTexture {
        mip_level: 0,
        texture: &texture,
        origin: wgpu::Origin3d::ZERO,
        aspect: wgpu::TextureAspect::All
      },
      &image.to_rgba8(),
      wgpu::ImageDataLayout {
        offset: 0,
        bytes_per_row: Some(4 * dimensions.0),
        rows_per_image: Some(dimensions.1),
      },
      size,
    );

    Texture {
      texture,
      view,
      sampler
    }
  }
}