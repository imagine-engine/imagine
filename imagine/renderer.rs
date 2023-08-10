/*******************************************************************************
  renderer.rs
********************************************************************************
  Copyright 2023 Menelik Eyasu

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

use std::ptr;
use pyo3::prelude::*;
use std::sync::Mutex;
use std::ffi::CString;
use std::mem::size_of;
use super::frame::Frame;
use super::main_scene::MAIN_SCENE;

pub struct Renderer {
  pub size: wgpu::Extent3d,
  pub device: wgpu::Device,
  queue: wgpu::Queue,
  buffer: wgpu::Buffer,
  texture: wgpu::Texture,
  texture_view: wgpu::TextureView,
  pipeline: wgpu::RenderPipeline
}

impl Renderer {
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

    let texture_desc = wgpu::TextureDescriptor {
      size: wgpu::Extent3d {
        width: 1920,
        height: 1080,
        depth_or_array_layers: 1
      },
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: wgpu::TextureFormat::Rgba8UnormSrgb,
      usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
      label: None,
      view_formats: &[]
    };
    let texture = device.create_texture(&texture_desc);
    let texture_view = texture.create_view(&Default::default());

    let u32_size = std::mem::size_of::<u32>() as u32;
    let buffer_size = (u32_size * 1920 * 1080) as wgpu::BufferAddress;
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
      size: buffer_size,
      usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
      label: None,
      mapped_at_creation: false
    });

    let spirv_compiler = shaderc::Compiler::new().unwrap();
    let vs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: Some("default vertex shader"),
      source: wgpu::util::make_spirv(
        spirv_compiler.compile_into_spirv(
            include_str!("res/shaders/default.vert"),
            shaderc::ShaderKind::Vertex,
            "res/shaders/default.vert",
            "main",
            None
        ).unwrap().as_binary_u8()
      )
    });

    let fs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: Some("default fragment shader"),
      source: wgpu::util::make_spirv(
        spirv_compiler.compile_into_spirv(
          include_str!("res/shaders/default.frag"),
          shaderc::ShaderKind::Fragment,
          "res/shaders/default.frag",
          "main",
          None
        ).unwrap().as_binary_u8()
      )
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("Render Pipeline Layout"),
      bind_group_layouts: &[],
      push_constant_ranges: &[]
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("Render Pipeline"),
      layout: Some(&pipeline_layout),
      vertex: wgpu::VertexState {
        module: &vs_module,
        entry_point: "main",
        buffers: &[],
      },
      fragment: Some(wgpu::FragmentState {
        module: &fs_module,
        entry_point: "main",
        targets: &[
          Some(wgpu::ColorTargetState {
            format: texture_desc.format,
            blend: Some(wgpu::BlendState {
              alpha: wgpu::BlendComponent::REPLACE,
              color: wgpu::BlendComponent::REPLACE,
            }),
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
      },
      multiview: None
    });

    Self {
      device,
      queue,
      buffer,
      texture,
      texture_view,
      size: texture_desc.size,
      pipeline
    }
  }

  pub async fn render(&self) -> Vec<u8> {
    // let vertices = MAIN_SCENE.lock().unwrap().vertex_buffer();
    // let indices = MAIN_SCENE.lock().unwrap().index_buffer();

    // let vertices: Vec<f32> = vec![
    //   // positions		  // colors
    //   0.5, 0.5, 0.0,	  1.0, 0.0, 0.0,
    //   0.5, -0.5, 0.0,   0.0, 1.0, 0.0,
    //   -0.5, -0.5, 0.0,  0.0, 1.0, 0.0,
    //   -0.5,  0.5, 0.0,	0.0, 0.0, 1.0
    // ];
    // let indices: Vec<i32> = vec![
    //   3, 0, 1,
    //   3, 2, 1
    // ];

    let mut encoder = self.device.create_command_encoder(
      &wgpu::CommandEncoderDescriptor { label: None }
    );

    {
      let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[
          Some(wgpu::RenderPassColorAttachment {
            view: &self.texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
              load: wgpu::LoadOp::Clear(wgpu::Color {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 1.0
              }),
              store: true
            }
          })
        ],
        depth_stencil_attachment: None
      });
      render_pass.set_pipeline(&self.pipeline);
      render_pass.draw(0..3, 0..1);
    }

    let u32_size = std::mem::size_of::<u32>() as u32;
    encoder.copy_texture_to_buffer(
      wgpu::ImageCopyTexture {
        aspect: wgpu::TextureAspect::All,
        texture: &self.texture,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO
      },
      wgpu::ImageCopyBuffer {
        buffer: &self.buffer,
        layout: wgpu::ImageDataLayout {
          offset: 0,
          bytes_per_row: Some(u32_size * 1920),
          rows_per_image: Some(1080)
        }
      },
      self.size
    );

    self.queue.submit(Some(encoder.finish()));

    let mut pixels = vec![];
    {
      let buffer_slice = self.buffer.slice(..);

      let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
      buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
        tx.send(result).unwrap();
      });
      self.device.poll(wgpu::Maintain::Wait);
      rx.receive().await.unwrap().unwrap();

      pixels = buffer_slice.get_mapped_range().to_vec();
    }

    self.buffer.unmap();

    pixels
  }

  pub fn create_shader(&self, filename: &str, source: &str, shader_type: shaderc::ShaderKind) -> wgpu::ShaderModule {
    self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: Some("shader"),
      source: wgpu::util::make_spirv(
        shaderc::Compiler::new().unwrap().compile_into_spirv(
          source,
          shader_type,
          filename,
          "main",
          None
        ).unwrap().as_binary_u8()
      )
    })
  }
}

// #[pyclass(name="Renderer")]
// pub struct PyRenderer;

// #[pymethods]
// impl PyRenderer {
//   // fn __clear__(&self) {
//   //   device.destroy_context(&mut self.context).unwrap();
//   // }
// }

#[pyfunction]
pub fn render() -> PyResult<Frame> {
  let renderer = &MAIN_SCENE.lock().unwrap().renderer;

  Ok(Frame {
    width: renderer.size.width,
    height: renderer.size.height,
    pixels: futures::executor::block_on(
      renderer.render()
    )
  })
}