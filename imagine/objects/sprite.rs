/*******************************************************************************
  sprite.rs
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
use nalgebra::{Vector2, Matrix3};
use crate::math::vector::Vector;
use crate::instance::IMAGINE;
use crate::render::primitives::SpriteConfig;

#[pyclass]
pub struct Sprite {
  #[pyo3(get)]
  pub scale: Vector,
  #[pyo3(get)]
  pub position: Vector,
  #[pyo3(get)]
  pub rotation: f32
}

#[pymethods]
impl Sprite {
  pub fn new() -> Self {
    Self {
      scale: Vector::new(1.0, 1.0, 1.0),
      position: Vector::new(0.0, 0.0, 0.0),
      rotation: Vector::new(1.0, 1.0, 1.0),
      world_object: IMAGINE.lock().unwrap().world.add_sprite(SpriteConfig {
        scale: Vector3::new(1.0, 1.0),
        position: Vector3::zeros(),
        rotation: Vector3::zeros(),
        transform: Matrix4::identity(),
        material: String::from("brick-phong")
      })
    }
  }
}

#[pymethods]
impl Sprite {
  #[setter(scale)]
  fn set_scale(&mut self, new_scale: Vector) {
    self.scale = new_scale;
    self.world_object.update_transform(
      self.scale.clone(),
      self.position.clone(),
      self.rotation.clone()
    );
  }

  #[setter(position)]
  fn set_position(&mut self, new_position: Vector) {
    self.position = new_position;
    self.world_object.update_transform(
      self.scale.clone(),
      self.position.clone(),
      self.rotation.clone()
    );
  }

  #[setter(rotation)]
  fn set_rotation(&mut self, new_rotation: Vector) {
    self.rotation = new_rotation;
    self.world_object.update_transform(
      self.scale.clone(),
      self.position.clone(),
      self.rotation.clone()
    );
  }
}