/*******************************************************************************
  keyframe.rs
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
use crate::instance::IMAGINE;

#[pyfunction]
#[pyo3(signature=(duration=5.0))]
pub fn interpolate(duration: f32) -> KeyframeIterator {
  IMAGINE.lock().unwrap().keyframes(duration)
}

#[pyclass]
pub struct Keyframe {
  #[pyo3(get)]
  pub t: f32
}

#[pyclass]
pub struct KeyframeIterator {
  pub i: usize,
  pub delta: f32,
  pub frames: usize
}

#[pymethods]
impl KeyframeIterator {
  fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
    slf
  }

  fn __next__(mut slf: PyRefMut<Self>) -> Option<Keyframe> {
    IMAGINE.lock().unwrap().next_frame();
    slf.i += 1;

    if slf.i > slf.frames {
      return None;
    }

    Some(Keyframe {
      t: slf.i as f32 / slf.frames as f32
    })
  }
}