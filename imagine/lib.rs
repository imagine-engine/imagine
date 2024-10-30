mod math;
mod path;
mod color;
mod video;
mod world;
mod render;
mod objects;
mod instance;
mod animation;

use objects::*;
use math::Vector;
use color::Color;
use world::PyWorld;
use path::PathBuilder;
use pyo3::wrap_pymodule;
use objects::basic_shapes::*;
// use crate::animation::keyframe::interpolate;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;

#[pymodule]
#[pyo3(name="math")]
fn math_module(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
  m.add_class::<Vector>()?;

  Ok(())
}

#[pymodule]
#[pyo3(name="objects")]
fn object_module(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(square, m)?)?;
  m.add_function(wrap_pyfunction!(triangle, m)?)?;
  m.add_function(wrap_pyfunction!(rectangle, m)?)?;
  m.add_function(wrap_pyfunction!(pentagon, m)?)?;
  // m.add_function(wrap_pyfunction!(circle, m)?)?;

  m.add_class::<Path>()?;
  m.add_class::<PathBuilder>()?;
  m.add_class::<Text>()?;
  m.add_class::<Ellipse>()?;

  Ok(())
}

#[pymodule]
fn imagine(_py: Python, m: &PyModule) -> PyResult<()> {
  m.add_wrapped(wrap_pymodule!(math_module))?;
  m.add_wrapped(wrap_pymodule!(object_module))?;

  let sys_module = PyModule::import(_py, "sys")?;
  let sys: &PyDict = sys_module.getattr("modules")?.downcast()?;
  sys.set_item("imagine.math", m.getattr("math")?)?;
  sys.set_item("imagine.objects", m.getattr("objects")?)?;

  // sys.set_item("world", Py::new(_py, PyWorld {})?)?;
  // sys.set_item("output", Py::new(_py, PyOutput {})?)?;

  let world = Py::new(_py, PyWorld {}).unwrap();
  m.add("world", world)?;

  // sys.set_item("world", m.getattr("world")?)?;

  // m.add_function(wrap_pyfunction!(wait, m)?)?;
  // m.add_function(wrap_pyfunction!(record, m)?)?;
  // m.add_function(wrap_pyfunction!(stop, m)?)?;

  m.add_class::<Color>()?;

  Ok(())
}