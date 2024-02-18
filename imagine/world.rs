/*******************************************************************************
  world.rs
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

// use std::cell::Cell;
use pyo3::prelude::*;
use nalgebra::Matrix3;
use crate::controller::*;
use crate::instance::IMAGINE;
use crate::render::primitives::*;
use crate::math::Vector;
use std::collections::{HashMap, BTreeMap};

use crate::objects::Path;

pub enum Domain {
  World3D,
  World2D,
  Default
}

pub struct World {
  pub age: f32,
  pub domain: Domain,
  pub camera_3d: Camera3D,
  pub camera_2d: Camera2D,
  // pub default_camera_2d: i32,
  // pub default_camera_3d: i32,
  // pub cameras_3d: HashMap<i32, Camera3D>,
  // pub cameras_2d: HashMap<i32, Camera2D>,
  pub lights: HashMap<i32, WorldLight>,
  pub meshes: HashMap<i32, Object3D>,
  pub paths: BTreeMap<i32, PathConfig>,
  pub points: Vec<f32>,
  pub controls: Vec<u8>,
  pub animating: bool
}

impl World {
  pub fn add_mesh(&mut self, object: Object3D) -> Object3DController {
    let id = self.meshes.len() as i32;
    self.meshes.insert(id, object);

    Object3DController { id }
  }

  pub fn add_path(
    &mut self,
    points: &Vec<f32>,
    controls: &Vec<u8>,
    bounds: [f32; 4],
    path_segments: usize
  ) -> Path {
    let id = self.paths.len() as i32;
    self.points.extend(points);
    self.controls.extend(controls);

    Python::with_gil(|py| {
      let scale = Py::new(py, Vector::new(1.0, 1.0, 0.0)).unwrap();
      let position = Py::new(py, Vector::new(0.0, 0.0, 0.0)).unwrap();
      let config = PathConfig {
        opacity: 1.0,
        bounds,
        path_segments,
        rotation: 0.0,
        // rotation: Cell::new(0.0),
        scale: Py::clone_ref(&scale, py),
        position: Py::clone_ref(&position, py),
        transform: Matrix3::identity()
      };
      self.paths.insert(id, config);

      Path {
        id,
        scale,
        position,
        rotation: 0.0
        // rotation: Cell::new(0.0)
      }
    })
  }

  pub fn add_camera2d(&mut self, camera: Camera2D) -> Camera2DController {
    // let id = self.cameras_2d.len() as i32;
    // self.cameras_2d.insert(id, camera);

    // Camera2DController { id }
    Camera2DController { id: 0 }
  }

  pub fn add_camera3d(&mut self, camera: Camera3D) -> Camera3DController {
    // let id = self.cameras_3d.len() as i32;
    // self.cameras_3d.insert(id, camera);

    // Camera3DController { id }
    Camera3DController { id: 0 }
  }

  pub fn access_mesh<F>(
    &mut self,
    id: i32,
    modify: F
  ) where F: Fn(&mut Object3D) {
    if self.meshes.contains_key(&id) {
      self.meshes.entry(id).and_modify(modify);
    }
  }

  pub fn access_path<F>(
    &mut self,
    id: i32,
    modify: F
  ) where F: Fn(&mut PathConfig) {
    if self.paths.contains_key(&id) {
      self.paths.entry(id).and_modify(modify);
    }
  }
}

#[pyclass(name="World")]
pub struct PyWorld;

#[pymethods]
impl PyWorld {
  #[getter(age)]
  fn get_age(&self) -> PyResult<f32> {
    Ok(IMAGINE.lock().unwrap().world.age)
  }

  // #[cfg(debug_assertions)]
  #[getter(points)]
  fn get_points(&self) -> PyResult<Vec<f32>> {
    Ok(IMAGINE.lock().unwrap().world.points.clone())
  }

  // #[cfg(debug_assertions)]
  #[getter(controls)]
  fn get_controls(&self) -> PyResult<Vec<u8>> {
    Ok(IMAGINE.lock().unwrap().world.controls.clone())
  }
}