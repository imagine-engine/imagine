/*******************************************************************************
  shader.rs
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

use std::fs;
use std::str;
use std::path::Path;
use pyo3::prelude::*;
use crate::instance::IMAGINE;

#[pyclass]
pub struct Shader {
  pub module: wgpu::ShaderModule,
  shader_type: shaderc::ShaderKind
}

impl Shader {
  pub fn new(_source: &str, shader_type: shaderc::ShaderKind) -> Self {
    let path = Path::new(&_source);
    let mut source = String::from(_source);
    let mut filename = "unnamed_shader";
    if path.exists() {
      filename = _source;
      source = fs::read_to_string(path).expect("failed to open shader file");
    }

    Self {
      module: IMAGINE.lock().unwrap().output.render_graph.context.load_shader(
        filename,
        &source,
        shader_type
      ),
      shader_type
    }
  }
}

#[pymethods]
impl Shader {
  #[getter(type)]
  fn get_type(&self) -> PyResult<String> {
    match self.shader_type {
      shaderc::ShaderKind::Vertex => Ok("vertex".to_string()),
      shaderc::ShaderKind::Fragment => Ok("fragment".to_string()),
      _ => Ok("N/A".to_string())
    }
  }
}

#[pyfunction]
#[pyo3(name = "VertexShader")]
pub fn vertex_shader(_path: &str) -> PyResult<Shader> {
  Ok(Shader::new(_path, shaderc::ShaderKind::Vertex))
}

#[pyfunction]
#[pyo3(name = "FragmentShader")]
pub fn fragment_shader(_path: &str) -> PyResult<Shader> {
  Ok(Shader::new(_path, shaderc::ShaderKind::Fragment))
}

#[pyfunction]
#[pyo3(name = "ComputeShader")]
pub fn compute_shader(_path: &str) -> PyResult<Shader> {
  Ok(Shader::new(_path, shaderc::ShaderKind::Compute))
}