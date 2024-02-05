/*******************************************************************************
  frame.rs
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
use crate::color::Color;
use pyo3::types::PyTuple;

#[pyclass]
pub struct Frame {
  #[pyo3(get)]
  pub width: u32,
  #[pyo3(get)]
  pub height: u32,
  pub pixels: Vec<u8>
}

#[pymethods]
impl Frame {
  fn row(&self, y: u32) -> PyResult<Vec<Color>> {
    let mut column: Vec<Color> = Vec::new();
    for x in 0..self.width {
      let index: usize = 4 * (y*self.width+x) as usize;
      column.push(Color {
        r: self.pixels[index],
        g: self.pixels[index+1],
        b: self.pixels[index+2]
      })
    }

    Ok(column)
  }

  fn column(&self, x: u32) -> PyResult<Vec<Color>> {
    let mut column: Vec<Color> = Vec::new();
    for y in 0..self.height {
      let index: usize = 4 * (x*self.width+y) as usize;
      column.push(Color {
        r: self.pixels[index],
        g: self.pixels[index+1],
        b: self.pixels[index+2]
      })
    }

    Ok(column)
  }

  #[getter(pixels)]
  fn get_pixels(&self) -> PyResult<Vec<Vec<Color>>> {
    let mut rows: Vec<Vec<Color>> = Vec::new();

    for y in 0..self.height {
      let mut row: Vec<Color> = Vec::new();
      for x in 0..self.width {
        let index: usize = 4 * (y*self.width+x) as usize;
        row.push(Color {
          r: self.pixels[index],
          g: self.pixels[index+1],
          b: self.pixels[index+2]
        })
      }
      rows.push(row);
    }

    Ok(rows)
  }

  fn __getitem__(&self, coords: &PyTuple) -> PyResult<Color> {
    let x = coords.get_item(0).unwrap().extract::<u32>().unwrap();
    let y = coords.get_item(1).unwrap().extract::<u32>().unwrap();

    let index: usize = 4 * (y*self.width+x) as usize;
    Ok(Color {
      r: self.pixels[index],
      g: self.pixels[index+1],
      b: self.pixels[index+2]
    })
  }

  // #[pyo3(signature = (filename="frame.png"))]
  // pub fn save(&self, filename: &str) {
  //   let output_file = File::create(Path::empty(filename)).unwrap();
  //   let mut encoder = Encoder::new(
  //     output_file,
  //     self.width,
  //     self.height,
  //   );
  //   encoder.set_color(ColorType::Rgba);
  //   encoder.set_depth(BitDepth::Eight);

  //   let mut image = encoder.write_header().unwrap();
  //   image.write_image_data(&self.pixels).unwrap();
  // }
}