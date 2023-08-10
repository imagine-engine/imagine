/*******************************************************************************
  camera.rs
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

use std::path::Path;
use pyo3::prelude::*;
use super::frame::Frame;
use super::main_scene::MAIN_SCENE;
use super::video::Video;
// use pyo3::exceptions::PyRuntimeError;

#[pyclass]
pub struct Camera {
  #[pyo3(get, set)]
  pub recording: bool,
  // pub projection: Matrix4
}

#[pymethods]
impl Camera {
  #[staticmethod]
  pub fn new() -> Self {
    Self {
      recording: false
    }
  }

  #[getter(recording)]
  fn get_recording_status(&self) -> PyResult<bool> {
    Ok(MAIN_SCENE.lock().unwrap().output.writing)
  }

  // pub fn resolution(&self) -> (i32, i32) {
  //   if self.output.is_none() {
  //     return (self.output.width, self.output.height);
  //   } else {
  //     return (0, 0);
  //   }
  // }

  #[pyo3(signature = (path="video.mp4", fps=24, width=1920, height=1080, bitrate=8000))]
  pub fn record(
    &mut self,
    path: &str,
    fps: i32,
    width: i32,
    height: i32,
    bitrate: usize
  ) {
    self.recording = true;
    MAIN_SCENE.lock().unwrap().output.make(
      path,
      fps,
      width,
      height,
      // bitrate
    );
  }

  pub fn stop(&mut self) {
    self.recording = false;
    MAIN_SCENE.lock().unwrap().output.free();
  }

  // #[pyo3(signature = (filename="snapshot.png"))]
  // fn snapshot(&self, filename: &str) {
  //   let renderer = MAIN_SCENE.lock().unwrap().renderer;
  //   Frame {
  //     width: renderer.size.width,
  //     height: renderer.size.height,
  //     pixels: futures::executor::block_on(
  //       renderer.render()
  //     )
  //   }.save(filename);
  // }
}