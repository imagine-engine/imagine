/*******************************************************************************
  operation.rs
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
use std::collections::HashMap;
use crate::render::RenderContext;
use crate::render::RenderResource;
use crate::render::primitives::{
  Vertex3D,
  Uniform3D,
  Camera3DUniform,
  FillConfigUniform,
  ModelMaterial,
  PhongMaterial
};

use std::borrow::Cow;

pub enum RenderOperation {
  NotFound,
  Command {
    input: Vec<String>,
    output: Vec<String>,
    execute: fn(
      &RenderContext,
      &HashMap<String, RenderResource>
    ) -> wgpu::CommandBuffer
  },
  RenderPass {
    input: Vec<String>,
    output: Vec<String>,
    pipeline: wgpu::RenderPipeline,
    render: fn(
      &RenderContext,
      &wgpu::RenderPipeline,
      &HashMap<String, RenderResource>
    ) -> wgpu::CommandBuffer
  },
  ComputeDispatch {
    input: Vec<String>,
    output: Vec<String>,
    pipeline: wgpu::ComputePipeline,
    compute: fn(
      &RenderContext,
      &wgpu::ComputePipeline,
      &HashMap<String, RenderResource>
    ) -> wgpu::CommandBuffer
  }
}

impl RenderOperation {
  pub fn run(
    &self,
    context: &RenderContext,
    resources: &HashMap<String, RenderResource>
  ) -> wgpu::CommandBuffer {
    match self {
      RenderOperation::RenderPass { input, output, pipeline, render } => render(
        context,
        &pipeline,
        resources
      ),
      RenderOperation::ComputeDispatch { input, output, pipeline, compute } => compute(
        context,
        &pipeline,
        resources
      ),
      RenderOperation::Command { input, output, execute } => execute(context, resources),
      RenderOperation::NotFound => {
        context.device.create_command_encoder(
          &wgpu::CommandEncoderDescriptor { label: None }
        ).finish()
      }
    }
  }

  pub fn create_phong_pass(context: &RenderContext) -> (Self, HashMap<String, RenderResource>) {
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

    // Prepare camera uniforms
    let camera_layout = context.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
    });

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
    let model_layout = context.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
    });

    let offset = context.device.limits().min_uniform_buffer_offset_alignment;
    let model_buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
      label: None,
      size: context.max_models * offset as u64,
      usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
      mapped_at_creation: false
    });

    let model_binding = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
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

    // Prepare render pipeline
    let pipeline_layout = context.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: None,
      bind_group_layouts: &[
        &context.phong_layout,
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

    let mut resources = HashMap::new();
    resources.insert(String::from("camera_3d"), RenderResource::Uniform {
      buffer: camera_buffer,
      binding: camera_binding
    });
    resources.insert(String::from("output_texture"), RenderResource::Texture(
      context.create_texture(
        wgpu::TextureFormat::Rgba8UnormSrgb,
        wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT
      )
    ));
    resources.insert(String::from("framebuffer"), context.create_framebuffer());
    resources.insert(String::from("world_3d"), RenderResource::Batch(
      model_buffer,
      model_binding,
      Vec::new()
    ));

    let operation = RenderOperation::RenderPass {
      input: vec![
        String::from("world_3d")
      ],
      output: vec![String::from("output_texture")],
      pipeline,
      render: |context, pipeline, resources| {
        let mut encoder = context.device.create_command_encoder(
          &wgpu::CommandEncoderDescriptor { label: None }
        );

        if let Some(RenderResource::Batch(_, uniform_binding, models)) = resources.get("world_3d") {
          let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            depth_stencil_attachment: None,
            color_attachments: &[
              match resources.get("output_texture") {
                Some(RenderResource::Texture(output)) => Some(wgpu::RenderPassColorAttachment {
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

          pass.set_pipeline(pipeline);
          if let Some(RenderResource::Uniform { buffer, binding }) = resources.get("camera_3d") {
            pass.set_bind_group(1, &binding, &[]);
          }

          let uniform_offset = context.device.limits().min_uniform_buffer_offset_alignment as usize;
          for (i, model) in models.iter().enumerate() {
            if let Some(RenderResource::Material(material)) = resources.get(&model.material) {
              let offset = (i * uniform_offset) as wgpu::DynamicOffset;
              pass.set_bind_group(0, material.binding(), &[]);
              pass.set_bind_group(2, &uniform_binding, &[offset]);
              pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));
              pass.set_index_buffer(model.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
              pass.draw_indexed(0..model.size, 0, 0..1);
            }
          }
        }

        if let (
          Some(RenderResource::Texture(output)),
          Some(RenderResource::Buffer(buffer))
        ) = (
          resources.get("output_texture"),
          resources.get("framebuffer")
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
              buffer,
              layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(u32_size * context.size.width),
                rows_per_image: Some(context.size.height)
              }
            },
            context.size
          );
        }

        encoder.finish()
      }
    };

    (operation, resources)
  }

  pub fn create_fill_pass(context: &RenderContext) -> (Self, HashMap<String, RenderResource>) {
    let cs_module = context.device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: None,
      source: wgpu::util::make_spirv(
        shaderc::Compiler::new().unwrap().compile_into_spirv(
          include_str!("../resources/shaders/fill.comp"),
          shaderc::ShaderKind::Compute,
          "../resources/shaders/fill.comp",
          "main",
          None
        ).unwrap().as_binary_u8()
      )
    });

    // Prepare camera uniforms
    let config_layout = context.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      label: None,
      entries: &[wgpu::BindGroupLayoutEntry {
        binding: 0,
        count: None,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer {
          ty: wgpu::BufferBindingType::Uniform,
          has_dynamic_offset: false,
          min_binding_size: None
        }
      }]
    });

    let config_buffer = context.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: None,
      contents: bytemuck::cast_slice(&[FillConfigUniform::default()]),
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let config_binding = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
      label: None,
      layout: &config_layout,
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: config_buffer.as_entire_binding()
      }]
    });

    // Prepare storage buffers
    let segment_buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
      label: None,
      size: 4 * context.max_paths * std::mem::size_of::<f32>() as u64,
      usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
      mapped_at_creation: false
    });

    let winding_buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
      label: None,
      size: context.max_paths * std::mem::size_of::<i32>() as u64,
      usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
      mapped_at_creation: false
    });

    let path_buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
      label: None,
      size: context.max_paths * std::mem::size_of::<f32>() as u64,
      usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
      mapped_at_creation: false
    });

    let fill_layout = context.device.create_bind_group_layout(
      &wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
          wgpu::BindGroupLayoutEntry {
            binding: 0,
            count: None,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            }
          },
          wgpu::BindGroupLayoutEntry {
            binding: 1,
            count: None,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            }
          },
          wgpu::BindGroupLayoutEntry {
            binding: 2,
            count: None,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            }
          },
          wgpu::BindGroupLayoutEntry {
            binding: 3,
            count: None,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::StorageTexture {
              view_dimension: wgpu::TextureViewDimension::D2,
              format: wgpu::TextureFormat::Rgba8Unorm,
              access: wgpu::StorageTextureAccess::WriteOnly,
            }
          }
        ]
      }
    );

    let fill_texture = context.create_texture(
      wgpu::TextureFormat::Rgba8Unorm,
      wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::STORAGE_BINDING
    );

    let fill_bindings = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
      label: None,
      layout: &fill_layout,
      entries: &[
        wgpu::BindGroupEntry {
          binding: 0,
          resource: segment_buffer.as_entire_binding()
        },
        wgpu::BindGroupEntry {
          binding: 1,
          resource: winding_buffer.as_entire_binding()
        },
        wgpu::BindGroupEntry {
          binding: 2,
          resource: path_buffer.as_entire_binding()
        },
        wgpu::BindGroupEntry {
          binding: 3,
          resource: wgpu::BindingResource::TextureView(&fill_texture.view),
        }
      ]
    });

    // Prepare render pipeline
    let pipeline_layout = context.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: None,
      bind_group_layouts: &[
        &config_layout,
        &fill_layout
      ],
      push_constant_ranges: &[]
    });

    let pipeline = context.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
      label: None,
      layout: Some(&pipeline_layout),
      module: &cs_module,
      entry_point: "main"
    });

    let mut resources = HashMap::new();
    resources.insert(String::from("framebuffer"), context.create_framebuffer());
    resources.insert(String::from("output_texture"), RenderResource::Texture(fill_texture));
    resources.insert(String::from("fill_config"), RenderResource::Uniform {
      buffer: config_buffer,
      binding: config_binding
    });
    resources.insert(String::from("world_2d"), RenderResource::Layer(
      segment_buffer,
      winding_buffer,
      path_buffer,
      fill_bindings
    ));

    let operation = RenderOperation::ComputeDispatch {
      input: vec![
        String::from("world_2d"),
        String::from("fill_config")
      ],
      output: vec![String::from("output_texture")],
      pipeline,
      compute: |context, pipeline, resources| {
        let mut encoder = context.device.create_command_encoder(
          &wgpu::CommandEncoderDescriptor { label: None }
        );

        if let (
          Some(RenderResource::Layer(_, _, _, bindings)),
          Some(RenderResource::Uniform { buffer, binding })
        ) = (
          resources.get("world_2d"),
          resources.get("fill_config")
        ) {
          let mut pass = encoder.begin_compute_pass(
            &wgpu::ComputePassDescriptor { label: None }
          );

          pass.set_pipeline(pipeline);
          pass.set_bind_group(0, &binding, &[]);
          pass.set_bind_group(1, &bindings, &[]);
          pass.dispatch_workgroups(
            (context.size.width as f32 / 16.0).ceil() as u32,
            (context.size.height as f32 / 16.0).ceil() as u32,
            1
          );
        }

        if let (
          Some(RenderResource::Texture(output)),
          Some(RenderResource::Buffer(buffer))
        ) = (
          resources.get("output_texture"),
          resources.get("framebuffer")
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
              buffer,
              layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(u32_size * context.size.width),
                rows_per_image: Some(context.size.height)
              }
            },
            context.size
          );
        }

        encoder.finish()
      }
    };

    (operation, resources)
  }
}