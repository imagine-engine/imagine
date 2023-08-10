/*******************************************************************************
  lib.rs
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
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;

mod math;
mod color;
mod frame;
mod video;
mod camera;
mod shader;
mod renderer;
mod components;
mod main_scene;
mod scene_children;

use color::Color;
use camera::Camera;
use main_scene::wait;
use renderer::render;
use pyo3::wrap_pymodule;
use components::svg::svg;
use math::vector::Vector;
use shader::vertex_shader;
use shader::compute_shader;
use shader::fragment_shader;
use main_scene::PyMainScene;
use components::basic_shapes::square;
use components::basic_shapes::triangle;
use components::basic_shapes::rectangle;

#[pymodule]
#[pyo3(name = "math")]
fn math_module(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
  m.add_class::<Vector>()?;

  Ok(())
}

#[pymodule]
#[pyo3(name = "components")]
fn component_module(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(svg, m)?)?;
  m.add_function(wrap_pyfunction!(square, m)?)?;
  m.add_function(wrap_pyfunction!(triangle, m)?)?;
  m.add_function(wrap_pyfunction!(rectangle, m)?)?;

  Ok(())
}

#[pymodule]
#[pyo3(name = "shaders")]
fn shader_module(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(vertex_shader, m)?)?;
  m.add_function(wrap_pyfunction!(compute_shader, m)?)?;
  m.add_function(wrap_pyfunction!(fragment_shader, m)?)?;

  Ok(())
}

#[pymodule]
fn imagine(_py: Python, m: &PyModule) -> PyResult<()> {
  m.add_wrapped(wrap_pymodule!(math_module))?;
  m.add_wrapped(wrap_pymodule!(shader_module))?;
  m.add_wrapped(wrap_pymodule!(component_module))?;

  let sys = PyModule::import(_py, "sys")?;
  let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
  sys_modules.set_item("imagine.math", m.getattr("math")?)?;
  sys_modules.set_item("imagine.shader", m.getattr("shaders")?)?;
  sys_modules.set_item("imagine.components", m.getattr("components")?)?;

  m.add_class::<Color>()?;

  let main_scene = Py::new(_py, PyMainScene {}).unwrap();
  m.add("main_scene", main_scene)?;
  m.add_function(wrap_pyfunction!(wait, m)?)?;

  let camera = Py::new(_py, Camera::new()).unwrap();
  m.add("camera", camera)?;

  m.add_function(wrap_pyfunction!(render, m)?)?;

  Ok(())
}