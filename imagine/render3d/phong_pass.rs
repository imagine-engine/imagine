use wgpu::util::DeviceExt;

use crate::render::{
  RenderContext,
  RenderOperation,
  NodeBuilder
};

use crate::render::primitives::Texture;
use crate::render3d::resources::Uniform;
use crate::render3d::primitives::{
  Vertex3D,
  Uniform3D,
  Camera3DUniform
};

#[derive(Clone)]
pub struct PhongPassBuilder {
  camera: String,
  mesh_buffer: String,
  output_texture: String,
  framebuffer: String
}

pub struct PhongPass {
  pipeline: wgpu::RenderPipeline,
  // bindings: wgpu::BindGroup,
  camera: String,
  mesh_buffer: String,
  output_texture: String,
  framebuffer: String
}

impl PhongPassBuilder {
  pub fn new() -> Self {
    Self {
      camera: String::new(),
      mesh_buffer: String::new(),
      output_texture: String::new(),
      framebuffer: String::new()
    }
  }

  pub fn camera(&mut self, name: &str) -> &mut Self {
    self.camera = String::from(name);
    self
  }

  pub fn mesh_buffer(&mut self, name: &str) -> &mut Self {
    self.mesh_buffer = String::from(name);
    self
  }

  pub fn output(&mut self, name: &str) -> &mut Self {
    self.output_texture = String::from(name);
    self
  }

  pub fn framebuffer(&mut self, name: &str) -> &mut Self {
    self.framebuffer = String::from(name);
    self
  }

  pub fn collect(&self) -> Self {
    self.clone()
  }
}

impl NodeBuilder for PhongPassBuilder {
  type Op = PhongPass;

  fn build(&self, context: &mut RenderContext) -> Self::Op {
    // Prepare camera uniforms
    let camera_layout = context.device.create_bind_group_layout(&PhongPass::camera_layout());
    let camera_buffer = context.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: None,
      contents: bytemuck::cast_slice(&[Camera3DUniform::default()]),
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let camera_binding = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
      label: None,
      layout: &camera_layout,
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: camera_buffer.as_entire_binding()
      }]
    });

    // Prepare model uniforms
    let offset = context.device.limits().min_uniform_buffer_offset_alignment;
    let model_layout = context.device.create_bind_group_layout(&PhongPass::model_layout());
    let model_buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
      label: None,
      size: context.max_models * offset as u64,
      usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
      mapped_at_creation: false
    });

    let model_bindings = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
      label: None,
      layout: &model_layout,
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
          offset: 0,
          buffer: &model_buffer,
          size: wgpu::BufferSize::new(std::mem::size_of::<Uniform3D>() as _)
        })
      }]
    });

    // Initialize resources
    // context.add_resource(
    //   &self.camera,
    //   Uniform {
    //     buffer: camera_buffer,
    //     binding: camera_binding
    //   }
    // );
    // context.add_resource(
    //   &self.output_texture,
    //   context.create_texture(
    //     wgpu::TextureFormat::Rgba8UnormSrgb,
    //     wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT
    //   )
    // );
    // context.add_resource(&self.framebuffer, context.create_framebuffer());
    // context.add_resource(&self.mesh_buffer, Batch {
    //   buffer: model_buffer,
    //   binding: model_binding,
    //   Vec::new()
    // });

    PhongPass::new(
      context,
      self.camera.clone(),
      self.mesh_buffer.clone(),
      self.output_texture.clone(),
      self.framebuffer.clone()
    )
  }
}

impl PhongPass {
  pub fn new(
    context: &RenderContext,
    camera: String,
    mesh_buffer: String,
    output_texture: String,
    framebuffer: String
  ) -> Self {
    let vs_module = context.device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: None,
      source: wgpu::util::make_spirv(
        shaderc::Compiler::new().unwrap().compile_into_spirv(
          include_str!("../resources/shaders/main3d.vs"),
          shaderc::ShaderKind::Vertex,
          "../resources/shaders/main3d.vs",
          "main",
          None
        ).unwrap().as_binary_u8()
      )
    });

    let fs_module = context.device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: None,
      source: wgpu::util::make_spirv(
        shaderc::Compiler::new().unwrap().compile_into_spirv(
          include_str!("../resources/shaders/phong.fs"),
          shaderc::ShaderKind::Fragment,
          "../resources/shaders/phong.fs",
          "main",
          None
        ).unwrap().as_binary_u8()
      )
    });

    let phong_layout = context.device.create_bind_group_layout(&Self::layout());
    let model_layout = context.device.create_bind_group_layout(&Self::model_layout());
    let camera_layout = context.device.create_bind_group_layout(&Self::camera_layout());
    let pipeline_layout = context.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: None,
      bind_group_layouts: &[
        &phong_layout,
        &camera_layout,
        &model_layout
      ],
      push_constant_ranges: &[]
    });

    let pipeline = context.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: None,
      multiview: None,
      layout: Some(&pipeline_layout),
      vertex: wgpu::VertexState {
        module: &vs_module,
        entry_point: "main",
        buffers: &[Vertex3D::desc()]
      },
      fragment: Some(wgpu::FragmentState {
        module: &fs_module,
        entry_point: "main",
        targets: &[
          Some(wgpu::ColorTargetState {
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
            write_mask: wgpu::ColorWrites::ALL
          })
        ]
      }),
      primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: Some(wgpu::Face::Back),
        polygon_mode: wgpu::PolygonMode::Fill,
        unclipped_depth: false,
        conservative: false
      },
      depth_stencil: None,
      multisample: wgpu::MultisampleState {
        count: 1,
        mask: !0,
        alpha_to_coverage_enabled: false,
      }
    });

    PhongPass {
      pipeline,
      camera,
      mesh_buffer,
      output_texture,
      framebuffer
    }
  }

  pub fn layout() -> wgpu::BindGroupLayoutDescriptor<'static> {
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

  pub fn camera_layout() -> wgpu::BindGroupLayoutDescriptor<'static> {
    wgpu::BindGroupLayoutDescriptor {
      label: None,
      entries: &[wgpu::BindGroupLayoutEntry {
        binding: 0,
        count: None,
        visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer {
          ty: wgpu::BufferBindingType::Uniform,
          has_dynamic_offset: false,
          min_binding_size: None
        }
      }]
    }
  }

  pub fn model_layout() -> wgpu::BindGroupLayoutDescriptor<'static> {
    wgpu::BindGroupLayoutDescriptor {
      label: None,
      entries: &[wgpu::BindGroupLayoutEntry {
        binding: 0,
        count: None,
        visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer {
          ty: wgpu::BufferBindingType::Uniform,
          has_dynamic_offset: true,
          min_binding_size: None
        }
      }]
    }
  }
}

impl RenderOperation for PhongPass {
  fn input(&self) -> Vec<String> {
    vec![self.camera.clone(), self.mesh_buffer.clone()]
  }

  fn output(&self) -> Vec<String> {
    vec![self.output_texture.clone(), self.framebuffer.clone()]
  }

  fn run(&self, context: &mut RenderContext) {
    let mut encoder = context.device.create_command_encoder(
      &wgpu::CommandEncoderDescriptor { label: None }
    );

    {
      let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        depth_stencil_attachment: None,
        color_attachments: &[
          match context.get::<Texture>(&self.output_texture) {
            Some(output) => Some(wgpu::RenderPassColorAttachment {
              view: &output.view,
              resolve_target: None,
              ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                  r: 0.0,
                  g: 0.0,
                  b: 0.0,
                  a: 1.0
                }),
                store: true
              }
            }),
            _ => None
          }
        ]
      });

      pass.set_pipeline(&self.pipeline);
      if let Some(uniform) = context.get::<Uniform>(&self.camera) {
        pass.set_bind_group(1, &uniform.binding, &[]);
      }

      let uniform_offset = context.device.limits().min_uniform_buffer_offset_alignment as usize;
      // for (i, model) in models.iter().enumerate() {
      //   if let Some(RenderResource::Material(material)) = context.get(&model.material) {
      //     let offset = (i * uniform_offset) as wgpu::DynamicOffset;
      //     pass.set_bind_group(0, material.binding(), &[]);
      //     pass.set_bind_group(2, &uniform_binding, &[offset]);
      //     pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));
      //     pass.set_index_buffer(model.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
      //     pass.draw_indexed(0..model.size, 0, 0..1);
      //   }
      // }
      // for (mesh, material) in world.query::<(MeshComponent, PhongComponent)>() {
      //   let offset = (i * uniform_offset) as wgpu::DynamicOffset;
      //   pass.set_bind_group(0, &material.binding, &[]);
      //   pass.set_bind_group(2, &model_bindings, &[offset]);
      //   pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));
      //   pass.set_index_buffer(model.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
      //   pass.draw_indexed(0..model.indices.len(), 0, 0..1);
      // }
    }

    if let (Some(output), Some(framebuffer)) = (
      context.get::<Texture>(&self.output_texture),
      context.get::<wgpu::Buffer>(&self.framebuffer)
    ) {
      let u32_size = std::mem::size_of::<u32>() as u32;
      encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
          aspect: wgpu::TextureAspect::All,
          texture: &output.texture,
          mip_level: 0,
          origin: wgpu::Origin3d::ZERO
        },
        wgpu::ImageCopyBuffer {
          buffer: framebuffer,
          layout: wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(u32_size * context.size.width),
            rows_per_image: Some(context.size.height)
          }
        },
        context.size
      );
    }

    context.queue.submit([encoder.finish()]);
  }
}