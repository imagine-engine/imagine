/*******************************************************************************
  output.rs
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
use crate::video::Video;
use crate::world::World;
use crate::render::render;
use crate::render::RenderGraph;

pub struct Output {
  pub video: Video,
  pub render_graph: RenderGraph
}

impl Output {
  pub fn stop(&mut self) {
    self.video.free();
  }

  pub fn start_video(
    &mut self,
    path: &str,
    fps: i32,
    width: i32,
    height: i32,
    bitrate: usize
  ) {
    self.video.make(
      path,
      fps,
      width,
      height,
      // bitrate
    );
  }

  pub fn write(&mut self, world: &World, frames: i32) {
    if self.video.writing {
      self.video.write(
        futures::executor::block_on(
          render(&world, &mut self.render_graph)
        ),
        frames
      );
    }
  }
}

#[pyclass(name="Output")]
pub struct PyOutput;

#[pymethods]
impl PyOutput {
  #[getter(width)]
  fn get_width(&self) -> PyResult<u32> {
    Ok(1920)
  }

  #[getter(height)]
  fn get_height(&self) -> PyResult<u32> {
    Ok(1080)
  }
}

// #[pyfunction]
// pub fn wait(t: f32) {
//   IMAGINE.lock().unwrap().world.age += t;
//   IMAGINE.lock().unwrap().output.update();
// }