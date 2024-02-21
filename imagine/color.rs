/*******************************************************************************
  color.rs
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

#[pyclass]
pub struct Color {
  #[pyo3(get, set)]
  pub r: u8,
  #[pyo3(get, set)]
  pub g: u8,
  #[pyo3(get, set)]
  pub b: u8
}

#[pymethods]
impl Color {
  #[classattr]
  const BLACK: Color = Color { r: 0, g: 0, b: 0 };
  #[classattr]
  const WHITE: Color = Color { r: 255, g: 255, b: 255 };
  #[classattr]
  const BLUE: Color = Color { r: 33, g: 150, b: 243 };

  #[new]
  fn new(hex: String) -> PyResult<Self> {
    // let shift = if _____ { 1 } else { 0 }
    Ok(
      Self {
        r: u8::from_str_radix(&hex[0..1], 16).unwrap(),
        g: u8::from_str_radix(&hex[2..3], 16).unwrap(),
        b: u8::from_str_radix(&hex[4..5], 16).unwrap()
      }
    )
  }

  fn __eq__(&self, other: &Color) -> PyResult<bool> {
    Ok(self.r == other.r && self.g == other.g && self.b == other.b)
  }

  fn __repr__(&self) -> PyResult<String> {
    let r = format!("{:02X}", self.r);
    let g = format!("{:02X}", self.g);
    let b = format!("{:02X}", self.b);
    Ok(format!("#{}{}{}", r, g, b).to_string())
  }
}