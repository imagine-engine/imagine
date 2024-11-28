use pyo3::prelude::*;

use crate::path::PathBuilder;
use crate::objects::{Mesh, Path};
use crate::render3d::primitives::Vertex3D;

#[pyfunction]
#[pyo3(name="Cube")]
pub fn cube() -> PyResult<Mesh> {
  let mesh = Mesh::new(
    vec![
      Vertex3D { position: [-0.5, 0.5, -0.5], normal: [0.0, 0.0, 0.0], uv: [0.0, 0.0] },
      Vertex3D { position: [0.5, 0.5, -0.5], normal: [0.0, 0.0, 0.0], uv: [0.0, 0.0] },
      Vertex3D { position: [0.5, -0.5, -0.5], normal: [0.0, 0.0, 0.0], uv: [0.0, 0.0] },
      Vertex3D { position: [-0.5, -0.5, -0.5], normal: [0.0, 0.0, 0.0], uv: [0.0, 0.0] },
      Vertex3D { position: [-0.5, 0.5, 0.5], normal: [0.0, 0.0, 0.0], uv: [0.0, 0.0] },
      Vertex3D { position: [0.5, 0.5, 0.5], normal: [0.0, 0.0, 0.0], uv: [0.0, 0.0] },
      Vertex3D { position: [0.5, -0.5, 0.5], normal: [0.0, 0.0, 0.0], uv: [0.0, 0.0] },
      Vertex3D { position: [-0.5, -0.5, 0.5], normal: [0.0, 0.0, 0.0], uv: [0.0, 0.0] }
    ],
    vec![
      0, 2, 1, 0, 3, 2, 5, 7, 4,
      5, 6, 7, 4, 1, 5, 4, 0, 1,
      6, 3, 7, 6, 2, 3, 7, 0, 4,
      7, 3, 0, 2, 5, 1, 2, 6, 5
    ]
  );

  Ok(mesh)
}

#[pyfunction]
#[pyo3(name="Triangle", signature=(size=5.0))]
pub fn triangle(size: f32) -> PyResult<Path> {
  let half_size = size / 2.0;
  let mut path = PathBuilder::new();
  path.move_to(0.0, half_size);
  path.line_to(-half_size, -half_size);
  path.line_to(half_size, -half_size);
  path.close();

  Ok(path.build())
}

#[pyfunction]
#[pyo3(name="Square", signature=(size=5.0))]
pub fn square(size: f32) -> PyResult<Path> {
  let half_size = size / 2.0;

  let mut path = PathBuilder::new();
  path.move_to(-half_size, half_size);
  path.line_to(-half_size, -half_size);
  path.line_to(half_size, -half_size);
  path.line_to(half_size, half_size);
  path.close();

  Ok(path.build())
}

#[pyfunction]
#[pyo3(name="Rectangle", signature=(width=10.0, height=5.0))]
pub fn rectangle(width: f32, height: f32) -> PyResult<Path> {
  let half_width = width / 2.0;
  let half_height = height / 2.0;

  let mut path = PathBuilder::new();
  path.move_to(-half_width, half_height);
  path.line_to(-half_width, -half_height);
  path.line_to(half_width, -half_height);
  path.line_to(half_width, half_height);
  path.close();

  Ok(path.build())
}

#[pyfunction]
#[pyo3(name="Pentagon", signature=(radius=50.0))]
pub fn pentagon(radius: f32) -> PyResult<Path> {
  let mut path = PathBuilder::new();
  path.move_to(0.0, 0.5*radius);
  path.line_to(-0.476*radius, 0.155*radius);
  path.line_to(-0.294*radius, -0.405*radius);
  path.line_to(0.294*radius, -0.405*radius);
  path.line_to(0.476*radius, 0.155*radius);
  path.close();

  Ok(path.build())
}