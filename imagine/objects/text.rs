/*******************************************************************************
  text.rs
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
use std::f32::consts::PI;
use crate::instance::IMAGINE;
use std::collections::HashMap;
use crate::objects::{Path, PathBuilder};

#[pyclass]
pub struct Font {
  cmap: HashMap<char, ttf::GlyphId>,
  glyphs: HashMap<ttf::GlyphId, PathBuilder>
}

#[pyclass]
pub struct Text {
  pub content: String,
  pub font: Font,
  pub path: Path
}

#[pymethods]
impl Text {
  #[new]
  #[pyo3(signature=(content, size=20.0))]
  pub fn new(content: String, size: f32) -> PyResult<Self> {
    let font = Font::default();
    let mut path = PathBuilder::new();
    let mut offset_x = 0.0;
    let mut offset_y = 0.0;
    for character in content.chars() {
      if character == ' ' {
        offset_x += 0.2 * size;
        continue;
      } else if character == '\n' {
        offset_x = 0.0;
        offset_y -= size;
        continue;
      }

      if let Some(glyph_id) = font.cmap.get(&character) {
        if let Some(glyph) = font.glyphs.get(glyph_id) {
          let mut glyph_path = PathBuilder::new();
          glyph_path.append(glyph);
          glyph_path.fit_height(size);
          glyph_path.shift(offset_x, offset_y);
          path.append(&glyph_path);
          offset_x += glyph_path.width();
        }
      }
    }

    Ok(Text { content, font, path: path.build() })
  }

  #[getter(bounds)]
  fn get_bounds(&self) -> PyResult<Option<[f32; 4]>> {
    match IMAGINE.lock().unwrap().world.paths.get(&self.path.id) {
      Some(path) => Ok(Some(path.bounds)),
      None => Ok(None)
    }
  }

  #[pyo3(signature=(x, y, t=1.0))]
  pub fn grow(&mut self, x: f32, y: f32, t: f32) {
    self.path.grow(x, y, t);
  }

  #[pyo3(signature=(x, y, t=1.0))]
  pub fn translate(&mut self, x: f32, y: f32, t: f32) {
    self.path.translate(x, y, t);
  }

  #[pyo3(signature=(t=1.0, angle=2.0*PI))]
  pub fn rotate(&mut self, t: f32, angle: f32) {
    self.path.rotate(t, angle);
  }
}

impl Font {
  pub fn default() -> Self {
    if let Ok(face) = ttf::Face::parse(include_bytes!("../resources/fonts/Rubik-Regular.ttf"), 0) {
      return Self::from(face);
    }

    Self {
      cmap: HashMap::new(),
      glyphs: HashMap::new()
    }
  }

  pub fn from(face: ttf::Face) -> Self {
    let mut cmap = HashMap::new();
    let mut glyphs = HashMap::new();
    let utf8 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789?!.";

    for code in utf8.chars() {
      if let Some(glyph) = face.glyph_index(code) {
        let mut builder = PathBuilder::new();
        face.outline_glyph(glyph, &mut builder);
        if let Some(ref mut bounds) = builder.temp_bounds {
          let advance = face.glyph_hor_advance(glyph).unwrap_or(0) as f32;
          let bbox = face.global_bounding_box();
          bounds[0] = bbox.x_min as f32;
          bounds[1] = bbox.y_min as f32;
          bounds[2] = bounds[0] + advance;
          bounds[3] = bbox.y_max as f32;
        }
        glyphs.insert(glyph, builder);
        cmap.insert(code, glyph);
      }
    }

    Self {
      cmap,
      glyphs
    }
  }
}