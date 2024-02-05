/*******************************************************************************
  controller.rs
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
use crate::math::Vector;
use crate::instance::IMAGINE;
use crate::animation::Animation;

pub struct Object2DController {
  pub id: i32
}

pub struct Object3DController {
  pub id: i32
}

impl Object2DController {
  pub fn animate(&self, animation: Animation) {
    IMAGINE.lock().unwrap().run(animation.duration, &[animation]);
  }

  pub fn update_transform(&self, scale: Vector, position: Vector, rotation: f32) {}
}

impl Object3DController {
  pub fn update_transform(&self, scale: Vector, position: Vector, rotation: Vector) {}
}