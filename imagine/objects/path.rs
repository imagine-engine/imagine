/*******************************************************************************
  path.rs
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

use crate::Color;
use pyo3::prelude::*;
use ttf_parser as ttf;
use crate::animation::*;
use crate::math::Vector;
use std::f32::consts::PI;
use svgtypes::PathParser;
use svgtypes::PathSegment;
use svgtypes::PathSegment::*;
use std::sync::{Arc, Mutex};
use crate::instance::IMAGINE;
use nalgebra::{Vector2, Matrix3};

pub enum PathAlignment {
  TopLeft,
  TopRight,
  BottomLeft,
  BottomRight,
  Center
}

#[pyclass]
pub struct PathBuilder {
  aligned: bool,
  segment_closed: bool,
  start: Option<[f32; 2]>,
  temp_points: Vec<f32>,
  temp_segments: Vec<u8>,
  pub temp_bounds: Option<[f32; 4]>
}

#[pyclass]
pub struct Path {
  pub id: i32,
  #[pyo3(get, set)]
  pub fill: Py<Color>,
  #[pyo3(get, set)]
  pub stroke: Py<Color>,
  #[pyo3(get, set)]
  pub scale: Py<Vector>,
  #[pyo3(get, set)]
  pub position: Py<Vector>,
  pub rotation: Arc<Mutex<f32>>
}

#[pymethods]
impl Path {
  #[new]
  #[pyo3(signature=(d=""))]
  pub fn new(d: &str) -> Self {
    let mut path = PathBuilder::from(d);
    path.build()
  }

  #[getter(bounds)]
  fn get_bounds(&self) -> PyResult<Option<[f32; 4]>> {
    match IMAGINE.lock().unwrap().world.paths.get(&self.id) {
      Some(path) => Ok(Some(path.bounds)),
      None => Ok(None)
    }
  }

  #[getter(rotation)]
  fn get_rotation(&self) -> PyResult<f32> {
    Ok(*self.rotation.lock().unwrap())
  }

  #[setter(rotation)]
  fn set_rotation(&mut self, new_rotation: f32) {
    *self.rotation.lock().unwrap() = new_rotation;
  }

  // #[pyo3(signature=(py_args="*", t=1.0))]
  // pub fn grow(&self, py_args: &PyTuple, t: f32) {
  #[pyo3(signature=(x, y, t=1.0))]
  pub fn grow(&self, x: f32, y: f32, t: f32) {
    IMAGINE.lock().unwrap().run(t, &[
      Animation {
        duration: t,
        update: AnimationUpdate::PathTransform2D(
          self.id,
          Some(Vector2::new(x, y)),
          None,
          None
        ),
        interpolation: Interpolation::EaseInOut
      }
    ]);
  }

  #[pyo3(signature=(x, y, t=1.0))]
  pub fn translate(&self, x: f32, y: f32, t: f32) {
    IMAGINE.lock().unwrap().run(t, &[
      Animation {
        duration: t,
        update: AnimationUpdate::PathTransform2D(
          self.id,
          None,
          Some(Vector2::new(x, y)),
          None
        ),
        interpolation: Interpolation::EaseInOut
      }
    ]);
  }

  #[pyo3(signature=(t=1.0, angle=2.0*PI))]
  pub fn rotate(&self, t: f32, angle: f32) {
    IMAGINE.lock().unwrap().run(t, &[
      Animation {
        duration: t,
        update: AnimationUpdate::PathTransform2D(
          self.id,
          None,
          None,
          Some(angle)
        ),
        interpolation: Interpolation::EaseInOut
      }
    ]);
  }
}

#[pymethods]
impl PathBuilder {
  pub fn move_to(&mut self, x: f32, y: f32) {
    self.temp_points.push(x);
    self.temp_points.push(y);

    self.update_bounds(x, y);
    self.segment_closed = false;
    match self.start {
      None => self.start = Some([x, y]),
      _ => ()
    }
  }

  pub fn line_to(&mut self, x: f32, y: f32) {
    if self.segment_closed {
      let last_x = self.temp_points[self.temp_points.len()-2];
      let last_y = self.temp_points[self.temp_points.len()-1];
      self.temp_points.push(last_x);
      self.temp_points.push(last_y);
    }

    self.temp_points.push(x);
    self.temp_points.push(y);

    self.update_bounds(x, y);
    self.temp_segments.push(0);
    self.segment_closed = true;
  }

  pub fn h_line_to(&mut self, x: f32) {
    let last_y = self.temp_points[self.temp_points.len()-1];
    if self.segment_closed {
      let last_x = self.temp_points[self.temp_points.len()-2];
      self.temp_points.push(last_x);
      self.temp_points.push(last_y);
    }

    self.temp_points.push(x);
    self.temp_points.push(last_y);

    self.update_bounds(x, last_y);
    self.temp_segments.push(0);
    self.segment_closed = true;
  }

  pub fn v_line_to(&mut self, y: f32) {
    let last_x = self.temp_points[self.temp_points.len()-2];
    if self.segment_closed {
      let last_y = self.temp_points[self.temp_points.len()-1];
      self.temp_points.push(last_x);
      self.temp_points.push(last_y);
    }

    self.temp_points.push(last_x);
    self.temp_points.push(y);

    self.update_bounds(last_x, y);
    self.temp_segments.push(0);
    self.segment_closed = true;
  }

  pub fn quad_to(&mut self, x: f32, y: f32, cx: f32, cy: f32) {
    if self.segment_closed {
      let last_x = self.temp_points[self.temp_points.len()-2];
      let last_y = self.temp_points[self.temp_points.len()-1];
      self.temp_points.push(last_x);
      self.temp_points.push(last_y);
    }

    self.temp_points.push(cx);
    self.temp_points.push(cy);
    self.temp_points.push(x);
    self.temp_points.push(y);

    self.update_bounds(x, y);
    self.temp_segments.push(1);
    self.segment_closed = true;
  }

  pub fn cubic_to(&mut self, x: f32, y: f32, cx1: f32, cy1: f32, cx2: f32, cy2: f32) {
    if self.segment_closed {
      let last_x = self.temp_points[self.temp_points.len()-2];
      let last_y = self.temp_points[self.temp_points.len()-1];
      self.temp_points.push(last_x);
      self.temp_points.push(last_y);
    }

    self.temp_points.push(cx1);
    self.temp_points.push(cy1);
    self.temp_points.push(cx2);
    self.temp_points.push(cy2);
    self.temp_points.push(x);
    self.temp_points.push(y);

    self.update_bounds(x, y);
    self.temp_segments.push(2);
    self.segment_closed = true;
  }

  pub fn shift(&mut self, x: f32, y: f32) {
    for i in (0..self.temp_points.len()-1).step_by(2) {
      self.temp_points[i] += x;
      self.temp_points[i+1] += y;
    }

    if let Some(ref mut bounds) = self.temp_bounds {
      bounds[0] += x;
      bounds[1] += y;
      bounds[2] += x;
      bounds[3] += y;
    }
  }

  pub fn close(&mut self) {
    if let Some(point) = self.start {
      self.line_to(point[0], point[1]);
    }
    self.start = None;
  }

  pub fn build(&mut self) -> Path {
    if !self.aligned {
      self.align(PathAlignment::Center);
    }

    let bounds = match self.temp_bounds {
      Some(bbox) => bbox,
      None => [0.0, 0.0, 0.0, 0.0]
    };

    let path = IMAGINE.lock().unwrap().world.add_path(
      &self.temp_points,
      &self.temp_segments,
      bounds,
      self.temp_segments.len()
    );

    self.temp_points.clear();
    self.temp_segments.clear();
    self.segment_closed = false;
    self.temp_bounds = None;

    path
  }
}

impl PathBuilder {
  pub fn new() -> Self {
    Self {
      start: None,
      aligned: false,
      segment_closed: false,
      temp_points: Vec::new(),
      temp_segments: Vec::new(),
      temp_bounds: None
    }
  }

  pub fn from(d: &str) -> Self {
    let mut builder = Self::new();

    for seg in PathParser::from(d) {
      if let Ok(segment) = seg {
        builder.push(segment);
      }
    }

    builder
  }

  pub fn width(&self) -> f32 {
    if let Some(bounds) = self.temp_bounds {
      return bounds[2] - bounds[0];
    }
    
    0.0
  }

  pub fn height(&self) -> f32 {
    if let Some(bounds) = self.temp_bounds {
      return bounds[3] - bounds[1];
    }
    
    0.0
  }

  pub fn set_bounds(&mut self, bbox: ttf::Rect) {
    self.temp_bounds = Some([
      bbox.x_min as f32, bbox.y_min as f32,
      bbox.x_max as f32, bbox.y_max as f32
    ]);
  }

  fn update_bounds(&mut self, x: f32, y: f32) {
    if let Some(ref mut bounds) = self.temp_bounds {
      if x < bounds[0] { bounds[0] = x; }
      else if x > bounds[2] { bounds[2] = x; }

      if y < bounds[1] { bounds[1] = y; }
      else if y > bounds[3] { bounds[3] = y; }
    } else {
      self.temp_bounds = Some([x, y, x, y]);
    }
  }

  fn push(&mut self, segment: PathSegment) {
    let absolute = match segment {
      // MoveTo { abs, x, y } => abs,
      LineTo { abs, x, y } => abs,
      HorizontalLineTo { abs, x } => abs,
      VerticalLineTo { abs, y } => abs,
      Quadratic { abs, x1, y1, x, y } => abs,
      CurveTo { abs, x1, y1, x2, y2, x, y } => abs,
      // ClosePath { abs } => abs,
      _ => true
    };

    let rx = if absolute { 0.0 } else { self.temp_points[self.temp_points.len()-2] as f64 };
    let ry = if absolute { 0.0 } else { self.temp_points[self.temp_points.len()-1] as f64 };
    match segment {
      MoveTo { abs, x, y } => self.move_to((rx+x) as f32, (rx-y) as f32),
      LineTo { abs, x, y } => self.line_to((rx+x) as f32, (ry-y) as f32),
      HorizontalLineTo { abs, x } => self.h_line_to((rx+x) as f32),
      VerticalLineTo { abs, y } => self.v_line_to((ry-y) as f32),
      Quadratic { abs, x1, y1, x, y } => self.quad_to(
        (rx+x) as f32, (ry-y) as f32,
        (rx+x1) as f32, (ry-y1) as f32
      ),
      CurveTo { abs, x1, y1, x2, y2, x, y } => self.cubic_to(
        (rx+x) as f32, (ry-y) as f32,
        (rx+x1) as f32, (ry-y1) as f32,
        (rx+x2) as f32, (ry-y2) as f32
      ),
      ClosePath { abs } => self.close(),
      _ => ()
    }
  }

  pub fn append(&mut self, other: &PathBuilder) {
    if let Some(bounds) = other.temp_bounds {
      self.update_bounds(bounds[0], bounds[1]);
      self.update_bounds(bounds[2], bounds[3]);
    }

    self.temp_points.extend(&other.temp_points);
    self.temp_segments.extend(&other.temp_segments);
  }

  pub fn fit_height(&mut self, height: f32) {
    let scale = if let Some(bounds) = self.temp_bounds {
      height / (bounds[3] - bounds[1])
    } else { 1.0 };

    for value in self.temp_points.iter_mut() {
      *value *= scale;
    }

    if let Some(ref mut bounds) = self.temp_bounds {
      bounds[0] *= scale;
      bounds[1] *= scale;
      bounds[2] *= scale;
      bounds[3] *= scale;
    }
  }

  pub fn align(&mut self, alignment: PathAlignment) {
    self.aligned = true;
    if let Some(bounds) = self.temp_bounds {
      match alignment {
        PathAlignment::BottomLeft => self.shift(-bounds[0], -bounds[1]),
        PathAlignment::TopLeft => self.shift(-bounds[0], -bounds[3]),
        PathAlignment::BottomRight => self.shift(-bounds[2], -bounds[1]),
        PathAlignment::TopRight => self.shift(-bounds[2], -bounds[3]),
        PathAlignment::Center => {
          let cx = (bounds[0] + bounds[2]) / 2.0;
          let cy = (bounds[1] + bounds[3]) / 2.0;
          self.shift(-cx, -cy);
        }
      }
    }
  }
}

impl ttf::OutlineBuilder for PathBuilder {
  fn move_to(&mut self, x: f32, y: f32) {
    self.move_to(x, y);
  }

  fn line_to(&mut self, x: f32, y: f32) {
    self.line_to(x, y);
  }

  fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
    self.quad_to(x, y, x1, y1);
  }

  fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
    self.cubic_to(x, y, x1, y1, x2, y2);
  }

  fn close(&mut self) {
    self.close();
  }
}