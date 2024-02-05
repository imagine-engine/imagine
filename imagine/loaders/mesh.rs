/*******************************************************************************
  mesh.rs
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

// use std::fs::File;
use pyo3::prelude::*;
// use std::io::BufReader;
use crate::objects::Mesh;
use crate::instance::IMAGINE;
use crate::math::vector::Vector;
use nalgebra::{Vector3, Matrix4};
use crate::render::primitives::{
  Vertex3D,
  Object3D
};

#[pyfunction]
pub fn load_mesh(path: &str) -> PyResult<Mesh> {
  // let reader = BufReader::new(File::open(path)?);

  let mut indices: Vec<u32> = Vec::new();
  let mut vertices: Vec<Vertex3D> = Vec::new();

  if path.ends_with(".obj") {
    let (models, _) = tobj::load_obj(
      path,
      &tobj::LoadOptions {
        triangulate: true,
        single_index: true,
        ..Default::default()
      }
    ).expect("failed to load obj file");

    indices.extend(&models[0].mesh.indices);

    for i in 0..models[0].mesh.positions.len() / 3 {
      vertices.push(Vertex3D {
        position: [
          models[0].mesh.positions[3 * i],
          models[0].mesh.positions[3 * i + 1],
          models[0].mesh.positions[3 * i + 2],
        ],
        normal: [
          models[0].mesh.normals[3 * i],
          models[0].mesh.normals[3 * i + 1],
          models[0].mesh.normals[3 * i + 2],
        ],
        uv: [
          models[0].mesh.texcoords[2 * i],
          models[0].mesh.texcoords[2 * i + 1]
        ]
      });
    }
  }

  Ok(Mesh {
    scale: Vector::new(1.0, 1.0, 1.0),
    position: Vector::new(0.0, 0.0, 0.0),
    rotation: Vector::new(1.0, 1.0, 1.0),
    world_object: IMAGINE.lock().unwrap().world.add_mesh(Object3D {
      vertices,
      indices,
      scale: Vector3::new(1.0, 1.0, 1.0),
      position: Vector3::zeros(),
      rotation: Vector3::zeros(),
      transform: Matrix4::identity(),
      material: String::from("brick-phong")
    })
  })
}