/*******************************************************************************
  app.rs
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

use pyo3::prelude::*;
use std::sync::Mutex;
use crate::animation::*;
use crate::video::Video;
use crate::output::Output;
use crate::render::render;
use lazy_static::lazy_static;
use crate::render::RenderGraph;
use crate::render::primitives::*;
use crate::world::{World, Domain};
use std::collections::{HashMap, BTreeMap};
use nalgebra::{Vector2, Vector3, Matrix3, Matrix4};

lazy_static! {
  pub static ref IMAGINE: Mutex<App> = Mutex::new(App {
    world: World {
      age: 0.0,
      animating: false,
      // domain: Domain::World3D,
      domain: Domain::Default,
      lights: HashMap::new(),
      ellipses: HashMap::new(),
      camera_2d: Camera2D::default(),
      camera_3d: Camera3D::default(),
      paths: BTreeMap::new(),
      points: Vec::new(),
      controls: Vec::new(),
      meshes: HashMap::from([
        (1, Object3D {
          vertices: vec![
            Vertex3D { position: [-0.5, 0.5, -0.5], normal: [0.0, 0.0, 0.0], uv: [0.0, 0.0] },
            Vertex3D { position: [0.5, 0.5, -0.5], normal: [0.0, 0.0, 0.0], uv: [0.0, 0.0] },
            Vertex3D { position: [0.5, -0.5, -0.5], normal: [0.0, 0.0, 0.0], uv: [0.0, 0.0] },
            Vertex3D { position: [-0.5, -0.5, -0.5], normal: [0.0, 0.0, 0.0], uv: [0.0, 0.0] },
            Vertex3D { position: [-0.5, 0.5, 0.5], normal: [0.0, 0.0, 0.0], uv: [0.0, 0.0] },
            Vertex3D { position: [0.5, 0.5, 0.5], normal: [0.0, 0.0, 0.0], uv: [0.0, 0.0] },
            Vertex3D { position: [0.5, -0.5, 0.5], normal: [0.0, 0.0, 0.0], uv: [0.0, 0.0] },
            Vertex3D { position: [-0.5, -0.5, 0.5], normal: [0.0, 0.0, 0.0], uv: [0.0, 0.0] }
          ],
          indices: vec![
            0, 2, 1, 0, 3, 2, 5, 7, 4,
            5, 6, 7, 4, 1, 5, 4, 0, 1,
            6, 3, 7, 6, 2, 3, 7, 0, 4,
            7, 3, 0, 2, 5, 1, 2, 6, 5
          ],
          scale: Vector3::new(1.0, 1.0, 1.0),
          position: Vector3::new(-1.0, 0.0, 0.0),
          rotation: Vector3::zeros(),
          transform: Matrix4::new_translation(&Vector3::new(-1.0, 0.0, 0.0)),
          material: String::from("brick-phong")
        }),
        (2, Object3D {
          vertices: vec![
            Vertex3D { position: [-0.5, 0.5, -0.5], normal: [0.0, 0.0, 0.0], uv: [0.0, 0.0] },
            Vertex3D { position: [0.5, 0.5, -0.5], normal: [0.0, 0.0, 0.0], uv: [0.0, 0.0] },
            Vertex3D { position: [0.5, -0.5, -0.5], normal: [0.0, 0.0, 0.0], uv: [0.0, 0.0] },
            Vertex3D { position: [-0.5, -0.5, -0.5], normal: [0.0, 0.0, 0.0], uv: [0.0, 0.0] },
            Vertex3D { position: [-0.5, 0.5, 0.5], normal: [0.0, 0.0, 0.0], uv: [0.0, 0.0] },
            Vertex3D { position: [0.5, 0.5, 0.5], normal: [0.0, 0.0, 0.0], uv: [0.0, 0.0] },
            Vertex3D { position: [0.5, -0.5, 0.5], normal: [0.0, 0.0, 0.0], uv: [0.0, 0.0] },
            Vertex3D { position: [-0.5, -0.5, 0.5], normal: [0.0, 0.0, 0.0], uv: [0.0, 0.0] }
          ],
          indices: vec![
            0, 2, 1, 0, 3, 2, 5, 7, 4,
            5, 6, 7, 4, 1, 5, 4, 0, 1,
            6, 3, 7, 6, 2, 3, 7, 0, 4,
            7, 3, 0, 2, 5, 1, 2, 6, 5
          ],
          scale: Vector3::new(1.0, 1.0, 1.0),
          position: Vector3::new(1.0, 0.0, 0.0),
          rotation: Vector3::zeros(),
          transform: Matrix4::new_translation(&Vector3::new(1.0, 0.0, 0.0)),
          material: String::from("brick-phong")
        })
      ])
    },
    output: Output {
      video: Video::new(),
      render_graph: RenderGraph::default()
    }
  });
}

pub struct App {
  pub world: World,
  pub output: Output
}

impl App {
  pub fn wait(&mut self, t: f32) {
    self.world.age += t;
    if let Some(fps) = self.output.video.get_fps() {
      self.output.write(
        &mut self.world,
        (t * fps as f32) as i32
      );
    }
  }

  pub fn keyframes<F>(&mut self, duration: f32, f: F) where F: Fn() {
    if let Some(fps) = self.output.video.get_fps() {
      let delta = 1.0 / fps as f32;
      for _ in 0..(duration * fps as f32) as usize {
        f();
        self.world.age += delta;
        self.output.write(&mut self.world, 1);
      }
    }
  }

  pub fn run(&mut self, duration: f32, animations: &[Animation]) {
    self.world.animating = true;
    let frames = (duration * self.output.video.get_fps().unwrap() as f32) as u32;
    Python::with_gil(|py| {
      for t in (0..frames as usize).map(|i| i as f32 / (frames-1) as f32) {
        for animation in animations {
          match &animation.update {
            AnimationUpdate::Transform3D(id, scale, position, rotation) => self.world.access_mesh(
              *id, |object| {
                object.transform = animation.interpolation.transform3d(
                  t,
                  &object.scale,
                  &object.position,
                  &object.rotation,
                  &scale,
                  &position,
                  &rotation
                );
              }
            ),
            AnimationUpdate::PathTransform2D(id, scale, position, rotation) => self.world.access_path(
              *id, |object| {
                let og_scale = object.scale.borrow(py);
                let og_position = object.position.borrow(py);

                object.transform = animation.interpolation.transform2d(
                  t,
                  &Vector2::<f32>::new(og_scale.x, og_scale.y),
                  &Vector2::<f32>::new(og_position.x, og_position.y),
                  *object.rotation.lock().unwrap(),
                  scale.as_ref(),
                  position.as_ref(),
                  *rotation
                );
              }
            ),
            AnimationUpdate::EllipseTransform2D(id, scale, position, rotation) => self.world.access_ellipse(
              *id, |object| {
                let og_scale = object.scale.borrow(py);
                let og_position = object.position.borrow(py);

                object.transform = animation.interpolation.transform2d(
                  t,
                  &Vector2::<f32>::new(og_scale.x, og_scale.y),
                  &Vector2::<f32>::new(og_position.x, og_position.y),
                  *object.rotation.lock().unwrap(),
                  scale.as_ref(),
                  position.as_ref(),
                  *rotation
                );
              }
            ),
            AnimationUpdate::Camera3DTransform(scale, position, rotation) => {
              self.world.camera_3d.view = animation.interpolation.camera_transform3d(
                t,
                &self.world.camera_3d.scale,
                &self.world.camera_3d.position,
                &self.world.camera_3d.rotation,
                &scale,
                &position,
                &rotation
              );
            },
            AnimationUpdate::Camera2DTransform(scale, position, rotation) => {
              self.world.camera_2d.view = animation.interpolation.camera_transform2d(
                t,
                &self.world.camera_2d.scale,
                &self.world.camera_2d.position,
                self.world.camera_2d.rotation,
                &scale,
                &position,
                *rotation
              );
            },
            AnimationUpdate::Perspective(final_fov, final_near, final_far) => {
              if let CameraProjection::Perspective(
                initial_fov,
                initial_near,
                initial_far
              ) = self.world.camera_3d.config {
                self.world.camera_3d.projection = animation.interpolation.perspective(
                  t,
                  self.world.camera_3d.aspect,
                  initial_fov,
                  initial_near,
                  initial_far,
                  *final_fov,
                  *final_near,
                  *final_far
                );
              }
            },
            // AnimationUpdate::Orthograpic(left, right, bottom, top, near, far) => {
            //   if let CameraProjection::Orthograpic(config) = self.world.camera_3d.config {
            //     self.world.camera_3d.projection = animation.interpolation.orthograpic(
            //       t,
            //       config.left, config.right, config.bottom, config.top,
            //       config.near, config.far,
            //       left, right, bottom, top, near, far
            //     );
            //   }
            // }
          }
        }

        self.output.write(&mut self.world, 1);
      }

      for animation in animations {
        match &animation.update {
          AnimationUpdate::Transform3D(id, scale, position, rotation) => self.world.access_mesh(
            *id,
            |mesh| {
              mesh.scale = *scale;
              mesh.position = *position;
              mesh.rotation = *rotation;
            }
          ),
          AnimationUpdate::PathTransform2D(id, s, p, r) => self.world.access_path(
            *id,
            |object| {
              if let Some(new_scale) = s {
                let mut scale = object.scale.borrow_mut(py);
                scale.x = new_scale.x;
                scale.y = new_scale.y;
              }
              if let Some(new_position) = p {
                let mut position = object.position.borrow_mut(py);
                position.x = new_position.x;
                position.y = new_position.y;
              }
              if let Some(new_rotation) = r {
                *object.rotation.lock().unwrap() = *new_rotation;
              }
            }
          ),
          AnimationUpdate::EllipseTransform2D(id, s, p, r) => self.world.access_ellipse(
            *id,
            |object| {
              if let Some(new_scale) = s {
                let mut scale = object.scale.borrow_mut(py);
                scale.x = new_scale.x;
                scale.y = new_scale.y;
              }
              if let Some(new_position) = p {
                let mut position = object.position.borrow_mut(py);
                position.x = new_position.x;
                position.y = new_position.y;
              }
              if let Some(new_rotation) = r {
                *object.rotation.lock().unwrap() = *new_rotation;
              }
            }
          ),
          AnimationUpdate::Camera3DTransform(scale, position, rotation) => {
            self.world.camera_3d.scale = *scale;
            self.world.camera_3d.position = *position;
            self.world.camera_3d.rotation = *rotation;
          },
          AnimationUpdate::Camera2DTransform(scale, position, rotation) => {
            self.world.camera_2d.scale = *scale;
            self.world.camera_2d.position = *position;
            self.world.camera_2d.rotation = *rotation;
          },
          AnimationUpdate::Perspective(final_fov, final_near, final_far) => {
            if let CameraProjection::Perspective(mut fov, mut near, mut far) = self.world.camera_3d.config {
              fov = *final_fov;
              near = *final_near;
              far = *final_far;
            }
          }
        }
      }
    });

    self.world.age += duration;
    self.world.animating = false;
  }

  // pub fn snapshot(&mut self) {
  //   image::load_from_memory(futures::executor::block_on(
  //     render(&self.world, &mut self.output.render_graph)
  //   )).unwrap()
  // }
}