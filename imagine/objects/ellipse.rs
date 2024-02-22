/*******************************************************************************
  ellipse.rs
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
use crate::animation::*;
use crate::math::Vector;
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use crate::instance::IMAGINE;
use nalgebra::{Vector2, Matrix3};

#[pyclass]
pub struct Ellipse {
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

#[pyfunction]
#[pyo3(name="Circle", signature=(radius=10.0))]
pub fn circle(radius: f32) -> Ellipse {
  Ellipse::new(radius, radius)
}

#[pymethods]
impl Ellipse {
  #[new]
  #[pyo3(signature=(width=10.0, height=10.0))]
  pub fn new(width: f32, height: f32) -> Self {
    IMAGINE.lock().unwrap().world.add_ellipse(width, height)
  }

  #[getter(rotation)]
  fn get_rotation(&self) -> PyResult<f32> {
    Ok(*self.rotation.lock().unwrap())
  }

  #[setter(rotation)]
  fn set_rotation(&mut self, new_rotation: f32) {
    *self.rotation.lock().unwrap() = new_rotation;
  }

  #[pyo3(signature=(x, y, t=1.0))]
  pub fn grow(&self, x: f32, y: f32, t: f32) {
    IMAGINE.lock().unwrap().run(t, &[
      Animation {
        duration: t,
        update: AnimationUpdate::EllipseTransform2D(
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
        update: AnimationUpdate::EllipseTransform2D(
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
        update: AnimationUpdate::EllipseTransform2D(
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