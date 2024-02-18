/*******************************************************************************
  lib.rs
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

mod math;
mod color;
mod frame;
mod video;
mod world;
mod camera;
mod output;
mod render;
mod shader;
mod loaders;
mod objects;
mod instance;
mod animation;
mod controller;

use shader::*;
use output::*;
use loaders::*;
use math::Vector;
use color::Color;
use objects::Path;
use camera::Camera;
use world::PyWorld;
use pyo3::wrap_pymodule;
use objects::basic_shapes::*;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;

#[pymodule]
#[pyo3(name = "math")]
fn math_module(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
  m.add_class::<Vector>()?;

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
#[pyo3(name = "objects")]
fn object_module(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(square, m)?)?;
  m.add_function(wrap_pyfunction!(triangle, m)?)?;
  m.add_function(wrap_pyfunction!(rectangle, m)?)?;
  m.add_function(wrap_pyfunction!(pentagon, m)?)?;

  m.add_class::<Path>()?;

  Ok(())
}

#[pymodule]
#[pyo3(name = "loaders")]
fn loader_module(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(load_mesh, m)?)?;
  // m.add_function(wrap_pyfunction!(load_svg, m)?)?;
  // m.add_function(wrap_pyfunction!(load_scene, m)?)?;

  Ok(())
}

#[pymodule]
fn imagine(_py: Python, m: &PyModule) -> PyResult<()> {
  m.add_wrapped(wrap_pymodule!(math_module))?;
  m.add_wrapped(wrap_pymodule!(shader_module))?;
  m.add_wrapped(wrap_pymodule!(loader_module))?;
  m.add_wrapped(wrap_pymodule!(object_module))?;

  let sys = PyModule::import(_py, "sys")?;
  let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
  sys_modules.set_item("imagine.math", m.getattr("math")?)?;
  sys_modules.set_item("imagine.shader", m.getattr("shaders")?)?;
  sys_modules.set_item("imagine.loaders", m.getattr("loaders")?)?;
  sys_modules.set_item("imagine.objects", m.getattr("objects")?)?;

  let world = Py::new(_py, PyWorld {}).unwrap();
  m.add("world", world)?;

  let output = Py::new(_py, PyOutput {}).unwrap();
  m.add("output", output)?;

  m.add_function(wrap_pyfunction!(wait, m)?)?;
  m.add_function(wrap_pyfunction!(record, m)?)?;
  m.add_function(wrap_pyfunction!(stop, m)?)?;

  Ok(())
}