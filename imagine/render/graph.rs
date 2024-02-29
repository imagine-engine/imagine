/*******************************************************************************
  graph.rs
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

use pyo3::Python;
use std::mem::size_of;
use nalgebra::Vector2;
use wgpu::util::DeviceExt;
use std::collections::HashMap;
use crate::world::{World, Domain};
use crate::render::RenderOperation::*;
use crate::render::{RenderContext, RenderResource, RenderOperation};
use crate::render::primitives::{
  Model,
  Uniform3D,
  ModelMaterial,
  PathConfig,
  PathUniform,
  EllipseConfig,
  EllipseUniform,
  FillConfigUniform,
  StrokeConfigUniform
};

pub struct RenderGraph  {
  stages: Vec<Vec<String>>,
  pub context: RenderContext,
  pub resources: HashMap<String, RenderResource>,
  pub operations: HashMap<String, RenderOperation>
}

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

    // let (operation, resources) = RenderOperation::create_phong_pass(&graph.context);
    // graph.resources.extend(resources);
    // graph.stages.push(vec![String::from("phong pass")]);
    // graph.operations.insert(String::from("phong pass"), operation);

    let (fill_ops, fill_resources) = RenderOperation::create_fill_pass(&graph.context);
    graph.resources.extend(fill_resources);
    graph.stages.push(vec![String::from("fill pass")]);
    graph.operations.insert(String::from("fill pass"), fill_ops);

    let (stroke_ops, stroke_resources) = RenderOperation::create_stroke_pass(&graph.context);
    graph.resources.extend(stroke_resources);
    graph.stages.push(vec![String::from("stroke pass")]);
    graph.operations.insert(String::from("stroke pass"), stroke_ops);

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

  // pub fn resources<T>(&self, name: &[&str]) -> T {
  //   match self.resources.get(name) {
  //     Some(operation) => operation,
  //     None => &RenderResource::NotFound
  //   }
  // }
 
  // pub fn resources(&self, names: Vec<String>) -> HashMap<String, &RenderResource> {
  //   let mut bundle = HashMap::new();
  //   for name in names {
  //     bundle.insert(name, self.resource(&name));
  //   }

  //   bundle
  // }

  pub fn run(&mut self, world: &World) {
    let offset = self.context.device.limits().min_uniform_buffer_offset_alignment;

    match world.domain {
      Domain::World3D => {
        if let Some(RenderResource::Uniform { buffer, binding }) = self.resources.get("camera_3d") {
          self.context.queue.write_buffer(
            &buffer,
            0,
            bytemuck::cast_slice(&[world.camera_3d.uniform()])
          );
        }

        if let Some(RenderResource::Batch(uniform_buffer, _, models)) = self.resources.get_mut("world_3d") {
          models.clear();
    
          let mut uniforms: Vec<Uniform3D> = Vec::new();
          for object in world.meshes.values() {
            uniforms.push(object.uniform());
            models.push(object.model(&self.context.device));
          }
    
          self.context.queue.write_buffer(&uniform_buffer, 0, unsafe {
            std::slice::from_raw_parts(
              uniforms.as_ptr() as *const u8,
              uniforms.len() * offset as usize,
            )
          });
        }

        // world.camera_3d.updated = false;
        // world.meshes.updated.clear();
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
                [world.camera_2d.view.m11, world.camera_2d.view.m21, world.camera_2d.view.m31, 0.0],
                [world.camera_2d.view.m12, world.camera_2d.view.m22, world.camera_2d.view.m32, 0.0],
                [world.camera_2d.view.m13, world.camera_2d.view.m23, world.camera_2d.view.m33, 0.0]
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
                [world.camera_2d.view.m11, world.camera_2d.view.m21, world.camera_2d.view.m31, 0.0],
                [world.camera_2d.view.m12, world.camera_2d.view.m22, world.camera_2d.view.m32, 0.0],
                [world.camera_2d.view.m13, world.camera_2d.view.m23, world.camera_2d.view.m33, 0.0]
              ]
            }])
          );
        }

        let mut idx = 0;
        let mut size = 0;
        let mut path_offset = 0;
        let mut segments: Vec<f32> = Vec::new();
        let mut paths: Vec<PathUniform> = Vec::new();
        let mut ellipses: Vec<EllipseUniform> = Vec::new();

        for ellipse in world.ellipses.values() {
          size += size_of::<EllipseUniform>();
          ellipses.push(EllipseConfig::uniform(ellipse, world.animating));
          if size + size_of::<EllipseUniform>() > self.context.batch_size_2d {
            size = 0;
            self.draw_layer(&segments, &paths, &ellipses);
            ellipses.clear();
          }
        }

        for path in world.paths.values() {
          let mut segment_count = 0;
          for i in path_offset..path.path_segments {
            let controls = world.controls[path_offset+i];
            match controls {
              0 => {
                segments.push(world.points[idx]);
                segments.push(world.points[idx+1]);
                segments.push(world.points[idx+2]);
                segments.push(world.points[idx+3]);
                segment_count += 1;
              },
              1 => {
                let p1 = Vector2::new(world.points[idx], world.points[idx+1]);
                let p2 = Vector2::new(world.points[idx+4], world.points[idx+5]);
                let c1 = Vector2::new(world.points[idx+2], world.points[idx+3]);
                segments.push(p1.x);
                segments.push(p1.y);

                for i in 1..10 {
                  let t = i as f32 / 10.0;
                  let pt = (1.0 - t) * (1.0 - t) * p1 + 2.0 * (1.0 - t) * t * c1 + t * t * p2;
                  segments.push(pt.x);
                  segments.push(pt.y);
                  segments.push(pt.x);
                  segments.push(pt.y);
                }

                segments.push(p2.x);
                segments.push(p2.y);
                segment_count += 10;
              },
              2 => {
                let p1 = Vector2::new(world.points[idx], world.points[idx+1]);
                let p2 = Vector2::new(world.points[idx+6], world.points[idx+7]);
                let c1 = Vector2::new(world.points[idx+2], world.points[idx+3]);
                let c2 = Vector2::new(world.points[idx+4], world.points[idx+5]);
                segments.push(p1.x);
                segments.push(p1.y);

                for i in 1..100 {
                  let t = i as f32 / 100.0;
                  let m = (1.0 - t) * (1.0 - t) * p1 + 2.0 * (1.0 - t) * t * p2 + t * t * c1;
                  segments.push(m.x);
                  segments.push(m.y);
                  segments.push(m.x);
                  segments.push(m.y);
                }

                segments.push(p2.x);
                segments.push(p2.y);
                segment_count += 100;
              },
              _ => ()
            }

            idx += (4 + 2 * controls) as usize;
          }

          let path_size = size_of::<PathUniform>() + 4 * segment_count * std::mem::size_of::<f32>();
          size += path_size;

          if size > self.context.batch_size_2d {
            paths.clear();
            ellipses.clear();
            segments.drain(0..segments.len() - 4 * segment_count);
          }

          path_offset += path.path_segments;
          paths.push(PathConfig::uniform(path, world.animating, segment_count));
        }

        if size > 0 {
          self.draw_layer(&segments, &paths, &ellipses);
        }
      }
    }

    // for level in self.stages.iter() {
    //   let mut operations = Vec::new();
    //   for id in level.iter() {
    //     operations.push(self.op(id).run(
    //       &self.context,
    //       &self.resources
    //     ));
    //   }

    //   self.context.queue.submit(operations);
    // }
  }

  pub fn draw_layer(
    &mut self,
    segments: &Vec<f32>,
    paths: &Vec<PathUniform>,
    ellipses: &Vec<EllipseUniform>
  ) {
    if let Some(RenderResource::Layer(
      segment_buffer,
      path_buffer,
      ellipse_buffer,
      _
    )) = self.resources.get_mut("world_2d") {
      self.context.queue.write_buffer(
        &segment_buffer, 0, bytemuck::cast_slice(&segments)
      );

      self.context.queue.write_buffer(
        &path_buffer, 0, bytemuck::cast_slice(&paths)
      );

      self.context.queue.write_buffer(
        &ellipse_buffer, 0, bytemuck::cast_slice(&ellipses)
      );
    }

    self.update("world_2d");
  }

  pub fn update(&mut self, resource: &str) {
    for level in self.stages.iter() {
      let mut operations = Vec::new();
      for id in level.iter() {
        operations.push(self.op(id).run(
          &self.context,
          &self.resources
        ));
      }

      self.context.queue.submit(operations);
    }

    // let mut updated: Vec<String> = vec![resource.to_string()];
    // for level in self.stages.iter() {
    //   let mut operations = Vec::new();
    //   let mut next: Vec<String> = Vec::new();
    //   for id in level.iter() {
    //     let op = self.op(id);
    //     if op.has_input(updated.clone()) {
    //       operations.push(op.run(
    //         &self.context,
    //         &self.resources
    //       ));

    //       next.extend(op.output());
    //     }
    //   }

    //   updated = next;
    //   self.context.queue.submit(operations);
    // }
  }
}