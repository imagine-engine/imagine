/*******************************************************************************
  font.rs
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
use ttf_parser as ttf;
use pyo3::exceptions::PyValueError;

#[pyfunction]
pub fn load_font(path: &str) -> PyResult<Font> {
  if let Ok(font_data) = std::fs::read(Path::new(path)) {
    if let Ok(face) = ttf::Face::parse(&font_data, 0) {
      Ok(Font { face })
    }
  }

  Err(PyValueError::new_err("failed to load font"))
}