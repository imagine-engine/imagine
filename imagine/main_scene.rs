/*******************************************************************************
  main_scene.rs
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
use std::sync::Mutex;
use nalgebra::Matrix4;
use super::video::Video;
use super::camera::Camera;
use lazy_static::lazy_static;
use std::collections::HashMap;
use super::renderer::Renderer;
use super::scene_children::SceneLight;
use super::scene_children::SceneObject;

lazy_static! {
  pub static ref MAIN_SCENE: Mutex<MainScene> = Mutex::new(MainScene {
    duration: 0.0,
    output: Video::new(),
    camera: Camera::new(),
    objects: HashMap::new(),
    lights: HashMap::new(),
    renderer: futures::executor::block_on(Renderer::new())
  });
}

pub struct MainScene {
  pub duration: f32,
  pub renderer: Renderer,
  pub output: Video,
  pub camera: Camera,
  pub objects: HashMap<i32, SceneObject>,
  pub lights: HashMap<i32, SceneLight>,
  // pub groups: Vec<SceneGroup>
}

impl MainScene {
  pub fn vertex_buffer(&self) -> Vec<f32> {
    let vertices: Vec<f32> = vec![
      -0.5, -0.5, 0.0,
      0.5, -0.5, 0.0,
      0.0, 0.5, 0.0
    ];

    vertices
  }

  pub fn index_buffer(&self) -> Vec<i32> {
    let indices: Vec<i32> = vec![0, 1, 2];
    indices
  }

  // pub fn get_object(&mut self, id: i32) -> *SceneObject {
  //   self.objects.get(id).unwrap()
  // }

  pub fn transform_object(&mut self, id: i32, new_transform: Matrix4<f32>) {
    self.objects.entry(id).and_modify(|o| *o.transform = *new_transform);
  }

  pub fn add_object(&mut self, object: SceneObject) -> i32 {
    let id = self.objects.len() as i32;
    self.objects.insert(id, object);
    id
  }

  pub fn add_light(&mut self, object: SceneLight) -> i32 {
    let id = self.lights.len() as i32;
    self.lights.insert(id, object);
    id
  }

  // pub fn animate_object(&mut self, object: SceneObject) {}

  pub fn wait(&mut self, t: f32) {
    if self.output.writing {
      self.duration += t;
      self.output.write(
        futures::executor::block_on(
          self.renderer.render()
        ),
        (t * self.output.get_fps().unwrap() as f32) as i32
      );
    }
  }
}

#[pyclass(name="MainScene")]
pub struct PyMainScene;

#[pymethods]
impl PyMainScene {
  #[getter(duration)]
  fn get_duration(&self) -> PyResult<f32> {
    Ok(MAIN_SCENE.lock().unwrap().duration)
  }

  #[getter(vertices)]
  fn get_vertex_buffer(&self) -> PyResult<Vec<f32>> {
    Ok(MAIN_SCENE.lock().unwrap().vertex_buffer())
  }

  #[getter(indices)]
  fn get_index_buffer(&self) -> PyResult<Vec<i32>> {
    Ok(MAIN_SCENE.lock().unwrap().index_buffer())
  }

  #[getter(objects)]
  fn get_objects(&self) -> PyResult<Vec<SceneObject>> {
    Ok(
      MAIN_SCENE.lock()
                .unwrap()
                .objects
                .values()
                .cloned()
                .collect::<Vec<SceneObject>>()
    )
  }

  #[getter(lights)]
  fn get_lights(&self) -> PyResult<Vec<SceneLight>> {
    Ok(
      MAIN_SCENE.lock()
                .unwrap()
                .lights
                .values()
                .cloned()
                .collect::<Vec<SceneLight>>()
    )
  }
}

#[pyfunction]
pub fn wait(t: f32) {
  MAIN_SCENE.lock().unwrap().wait(t);
}