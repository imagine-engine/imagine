/*******************************************************************************
  imagine_object.rs
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

use pyo3::prelude::*;
use nalgebra::Vector3;
use nalgebra::Matrix4;
use nalgebra::Rotation3;
use crate::math::vector::Vector;
use crate::main_scene::MAIN_SCENE;
use crate::scene_children::SceneObject;

#[pyclass]
pub struct ImagineComponent {
  pub id: i32,
  #[pyo3(get)]
  pub scale: Vector,
  #[pyo3(get)]
  pub position: Vector,
  #[pyo3(get)]
  pub rotation: Vector
}

impl ImagineComponent {
  pub fn new(object: SceneObject) -> Self {
    Self {
      id: MAIN_SCENE.lock().unwrap().add_object(object),
      scale: Vector { x: 1.0, y: 1.0, z: 1.0 },
      position: Vector { x: 0.0, y: 0.0, z: 0.0 },
      rotation: Vector { x: 0.0, y: 0.0, z: 0.0 }
    }
  }

  fn update_global_transform(&self) {
    let scale = Matrix4::new_nonuniform_scaling(&Vector3::new(
      self.scale.x,
      self.scale.y,
      self.scale.z
    ));
    let position = Matrix4::new_translation(&Vector3::new(
      self.position.x,
      self.position.y,
      self.position.z
    ));
    let rotation = Matrix4::from_euler_angles(
      self.rotation.x,
      self.rotation.y,
      self.rotation.z
    );

    MAIN_SCENE.lock()
              .unwrap()
              .transform_object(self.id, scale * position * rotation);
  }
}

#[pymethods]
impl ImagineComponent {
  #[setter(scale)]
  fn set_scale(&mut self, new_scale: Vector) {
    self.scale = new_scale;
    self.update_global_transform();
  }

  #[setter(position)]
  fn set_position(&mut self, new_position: Vector) {
    self.position = new_position;
    self.update_global_transform();
  }

  #[setter(rotation)]
  fn set_rotation(&mut self, new_rotation: Vector) {
    self.rotation = new_rotation;
    self.update_global_transform();
  }
}