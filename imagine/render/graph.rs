use std::mem::size_of;
use nalgebra::Vector2;
use std::collections::HashMap;
use crate::world::{World, Domain};
use crate::render::{RenderContext, RenderResource, RenderOperation};
use crate::render::primitives::{
  Uniform3D,
  ModelMaterial,
  PathConfig,
  PathUniform,
  EllipseConfig,
  EllipseUniform,
  FillConfigUniform,
  StrokeConfigUniform
};

pub struct RenderGraph {
  stages: Vec<Vec<String>>,
  pub context: RenderContext,
  pub resources: HashMap<String, RenderResource>,
  pub operations: HashMap<String, RenderOperation>,
}

// pub struct RenderGraph {
//   context: RenderContext,
//   nodes: HashMap<String, Box<dyn RenderOperation>>,
//   adjacency: HashMap<String, Vec<String>>
// }

impl RenderGraph {
  fn empty() -> Self {
    RenderGraph {
      stages: Vec::new(),
      operations: HashMap::new(),
      resources: HashMap::new(),
      context: futures::executor::block_on(
        RenderContext::new()
      )
    }
  }

  pub fn default() -> Self {
    let mut graph = RenderGraph::empty();

    let (operation, resources) = RenderOperation::create_phong_pass(&graph.context);
    graph.resources.extend(resources);
    graph.stages.push(vec![String::from("phong pass")]);
    graph.operations.insert(String::from("phong pass"), operation);

    // let (fill_ops, fill_resources) = RenderOperation::create_fill_pass(&graph.context);
    // graph.resources.extend(fill_resources);
    // graph.stages.push(vec![String::from("fill pass")]);
    // graph.operations.insert(String::from("fill pass"), fill_ops);

    // let (stroke_ops, stroke_resources) = RenderOperation::create_stroke_pass(&graph.context);
    // graph.resources.extend(stroke_resources);
    // graph.stages.push(vec![String::from("stroke pass")]);
    // graph.operations.insert(String::from("stroke pass"), stroke_ops);

    graph.resources.insert(
      String::from("star"),
      RenderResource::Material(ModelMaterial::new_material2d(
        &graph.context, 1.0, 0.0,
        graph.context.load_texture(
          image::load_from_memory(include_bytes!(
            "../resources/materials/star.jpeg"
          )).unwrap(),
          wgpu::TextureFormat::Rgba8UnormSrgb
        )
      )
    ));

    graph.resources.insert(
      String::from("checkerboard"),
      RenderResource::Material(ModelMaterial::new_material2d(
        &graph.context, 1.0, 0.0,
        graph.context.load_texture(
          image::load_from_memory(include_bytes!(
            "../resources/materials/default.jpeg"
          )).unwrap(),
          wgpu::TextureFormat::Rgba8UnormSrgb
        )
      )
    ));

    graph.resources.insert(
      String::from("brick-phong"),
      RenderResource::Material(ModelMaterial::new_phong(
        &graph.context, 1.0, 0.0, 0.0,
        graph.context.load_texture(
          image::load_from_memory(include_bytes!(
            "../resources/materials/brick/normal.jpg"
          )).unwrap(),
          wgpu::TextureFormat::Rgba8Unorm
        ),
        graph.context.load_texture(
          image::load_from_memory(include_bytes!(
            "../resources/materials/brick/diffuse.jpg"
          )).unwrap(),
          wgpu::TextureFormat::Rgba8UnormSrgb
        ),
        graph.context.load_texture(
          image::load_from_memory(include_bytes!(
            "../resources/materials/brick/specular.jpg"
          )).unwrap(),
          wgpu::TextureFormat::Rgba8UnormSrgb
        )
      )
    ));

    graph
  }

  pub fn op(&self, name: &str) -> &RenderOperation {
    match self.operations.get(name) {
      Some(operation) => operation,
      None => &RenderOperation::NotFound
    }
  }

  pub fn resource(&self, name: &str) -> &RenderResource {
    match self.resources.get(name) {
      Some(operation) => operation,
      None => &RenderResource::NotFound
    }
  }

  pub fn run(&mut self, world: &World) {
    let offset = self.context.device.limits().min_uniform_buffer_offset_alignment;

    match world.domain {
      Domain::World3D => {
        if let Some(RenderResource::Uniform { buffer, binding }) = self.resources.get("camera_3d") {
          // self.context.queue.write_buffer(
          //   &buffer,
          //   0,
          //   bytemuck::cast_slice(&[world.camera_3d.uniform()])
          // );
        }

        if let Some(RenderResource::Batch(uniform_buffer, _, models)) = self.resources.get_mut("world_3d") {
          models.clear();
    
          let mut uniforms: Vec<Uniform3D> = Vec::new();
          // for object in world.meshes.values() {
          //   uniforms.push(object.uniform());
          //   models.push(object.model(&self.context.device));
          // }
    
          self.context.queue.write_buffer(&uniform_buffer, 0, unsafe {
            std::slice::from_raw_parts(
              uniforms.as_ptr() as *const u8,
              uniforms.len() * offset as usize,
            )
          });
        }
      },
      _ => {
        if let Some(RenderResource::Uniform { buffer, binding }) = self.resources.get("fill_config") {
          self.context.queue.write_buffer(
            &buffer,
            0,
            bytemuck::cast_slice(&[FillConfigUniform {
              clear: [0.0, 0.0, 0.0, 1.0],
              resolution: [1920.0, 1080.0],
              _padding: [0.0, 0.0],
              view: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0]
                // [world.camera_2d.view.m11, world.camera_2d.view.m21, world.camera_2d.view.m31, 0.0],
                // [world.camera_2d.view.m12, world.camera_2d.view.m22, world.camera_2d.view.m32, 0.0],
                // [world.camera_2d.view.m13, world.camera_2d.view.m23, world.camera_2d.view.m33, 0.0]
              ]
            }])
          );
        }

        if let Some(RenderResource::Uniform { buffer, binding }) = self.resources.get("stroke_config") {
          self.context.queue.write_buffer(
            &buffer,
            0,
            bytemuck::cast_slice(&[StrokeConfigUniform {
              resolution: [1920.0, 1080.0],
              _padding: [0.0, 0.0],
              view: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0]
                // [world.camera_2d.view.m11, world.camera_2d.view.m21, world.camera_2d.view.m31, 0.0],
                // [world.camera_2d.view.m12, world.camera_2d.view.m22, world.camera_2d.view.m32, 0.0],
                // [world.camera_2d.view.m13, world.camera_2d.view.m23, world.camera_2d.view.m33, 0.0]
              ]
            }])
          );
        }
      }
    }
  }
}