/*******************************************************************************
  animation.rs
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

use nalgebra::{Vector2, Vector3, Matrix3, Matrix4};

pub enum AnimationUpdate {
  Transform3D(i32, Vector3<f32>, Vector3<f32>, Vector3<f32>),
  Transform2D(i32, Vector2<f32>, Vector2<f32>, f32),
  Camera3DTransform(Vector3<f32>, Vector3<f32>, Vector3<f32>),
  Camera2DTransform(Vector2<f32>, Vector2<f32>, f32),
  Perspective(f32, f32, f32),
  // Orthograpic(f32, f32, f32, f32, f32, f32)
}

pub enum Interpolation {
  Linear,
  EaseIn,
  EaseOut,
  // EaseOutBounce,
  EaseInOut
}

impl Interpolation {
  fn apply(&self, t: f32) -> f32 {
    match self {
      Interpolation::Linear => t,
      Interpolation::EaseIn => t*t*t,
      Interpolation::EaseOut => 1.0 - (1.0-t).powi(3),
      Interpolation::EaseInOut => if t < 0.5 {
        4.0 * t * t * t
      } else {
        1.0 - (2.0 - 2.0*t).powi(3) / 2.0
      },
      // Interpolation::EaseOutBounce => if t < 1 / d1 {
      //   7.5625 * t * t
      // } else if x < 2 / 2.75 {
      //   7.5625 * (t - 1.5 / 2.75) * t + 0.75
      // } else if t < 2.5 / 2.75 {
      //   7.5625 * (t - 2.25 / 2.75) * t + 0.9375
      // } else {
      //   7.5625 * (t - 2.625 / 2.75) * t + 0.984375
      // }
    }
  }

  pub fn transform3d(
    &self,
    t: f32,
    initial_scale: &Vector3<f32>,
    initial_pos: &Vector3<f32>,
    initial_rot: &Vector3<f32>,
    final_scale: &Vector3<f32>,
    final_pos: &Vector3<f32>,
    final_rot: &Vector3<f32>
  ) -> Matrix4<f32> {
    let ease = self.apply(t);

    let scale = Matrix4::new_nonuniform_scaling(
      &initial_scale.lerp(final_scale, ease)
    );
    let position = Matrix4::new_translation(
      &initial_pos.lerp(final_pos, ease)
    );
    let rotation = Matrix4::from_euler_angles(
      initial_rot.x + ease * (final_rot.x - initial_rot.x),
      initial_rot.y + ease * (final_rot.y - initial_rot.y),
      initial_rot.z + ease * (final_rot.z - initial_rot.z)
    );

    scale * position * rotation
  }

  pub fn transform2d(
    &self,
    t: f32,
    initial_scale: &Vector2<f32>,
    initial_pos: &Vector2<f32>,
    initial_rot: f32,
    final_scale: &Vector2<f32>,
    final_pos: &Vector2<f32>,
    final_rot: f32
  ) -> Matrix3<f32> {
    let ease = self.apply(t);

    let scale = Matrix3::new_nonuniform_scaling(
      &initial_scale.lerp(final_scale, ease)
    );
    let position = Matrix3::new_translation(
      &initial_pos.lerp(final_pos, ease)
    );
    let rotation = Matrix3::new_rotation(
      initial_rot + ease * (final_rot - initial_rot)
    );

    scale * position * rotation
  }

  pub fn camera_transform3d(
    &self,
    t: f32,
    initial_scale: &Vector3<f32>,
    initial_eye: &Vector3<f32>,
    initial_rot: &Vector3<f32>,
    final_scale: &Vector3<f32>,
    final_eye: &Vector3<f32>,
    final_rot: &Vector3<f32>
  ) -> Matrix4<f32> {
    let ease = self.apply(t);

    let eye = -1.0 * initial_eye.lerp(final_eye, ease);
    let position = Matrix4::new_translation(&eye);
    let rotation = Matrix4::from_euler_angles(
      -initial_rot.x - ease * (final_rot.x - initial_rot.x),
      -initial_rot.y - ease * (final_rot.y - initial_rot.y),
      -initial_rot.z - ease * (final_rot.z - initial_rot.z)
    );
    let scale = Matrix4::new_nonuniform_scaling(
      &initial_scale.lerp(final_scale, ease)
    );

    scale * position * rotation
  }

  pub fn camera_transform2d(
    &self,
    t: f32,
    initial_scale: &Vector2<f32>,
    initial_eye: &Vector2<f32>,
    initial_rot: f32,
    final_scale: &Vector2<f32>,
    final_eye: &Vector2<f32>,
    final_rot: f32
  ) -> Matrix3<f32> {
    let ease = self.apply(t);

    let eye = -1.0 * initial_eye.lerp(final_eye, ease);
    let position = Matrix3::new_translation(&eye);
    let rotation = Matrix3::new_rotation(
      -initial_rot - ease * (final_rot - initial_rot)
    );
    let scale = Matrix3::new_nonuniform_scaling(
      &initial_scale.lerp(final_scale, ease)
    );

    scale * position * rotation
  }

  pub fn perspective(
    &self,
    t: f32,
    aspect_ratio: f32,
    initial_fov: f32,
    initial_near: f32,
    initial_far: f32,
    final_fov: f32,
    final_near: f32,
    final_far: f32
  ) -> Matrix4<f32> {
    let ease = self.apply(t);

    Matrix4::new_perspective(
      aspect_ratio,
      initial_fov + ease * (final_fov - initial_fov),
      initial_near + ease * (final_near - initial_near),
      initial_far + ease * (final_far - initial_far)
    )
  }

  pub fn orthographic(
    &self,
    t: f32,
    initial_left: f32,
    initial_right: f32,
    initial_bottom: f32,
    initial_top: f32,
    initial_near: f32,
    initial_far: f32,
    final_left: f32,
    final_right: f32,
    final_bottom: f32,
    final_top: f32,
    final_near: f32,
    final_far: f32
  ) -> Matrix4<f32> {
    let ease = self.apply(t);

    Matrix4::new_orthographic(
      initial_left + ease * (final_left - initial_left),
      initial_right + ease * (final_right - initial_right),
      initial_bottom + ease * (final_bottom - initial_bottom),
      initial_top + ease * (final_top - initial_top),
      initial_near + ease * (final_near - initial_near),
      initial_far + ease * (final_far - initial_far)
    )
  }
}

pub struct Animation {
  pub duration: f32,
  pub update: AnimationUpdate,
  pub interpolation: Interpolation
}