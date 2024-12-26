use std::slice::Iter;

use nalgebra::{Vector2, Matrix3};

use crate::color::Color;
use crate::render::primitives::Texture;

#[derive(Copy, Clone)]
pub enum StrokeLinecap {
  NoStroke,
  RoundCap,
  ButtCap,
  SquareCap
}

pub struct PathComponent {
  pub filled: bool,
  pub evenodd: bool,
  pub linecap: StrokeLinecap,
  pub bounds: [f32; 4],
  pub path_segments: usize
}

pub struct EllipseComponent {
  pub opacity: f32,
  pub width: f32,
  pub height: f32
}

pub struct BackgroundComponent {
  pub opacity: f32,
  pub fill: Color,
  pub stroke: Color,
  // pub fill: Texture
  // pub stroke: Texture
}

pub struct Camera2DComponent {
  pub aspect: f32,
  pub rotation: f32
}

pub struct Transform2DComponent {
  pub scale: Vector2<f32>,
  pub position: Vector2<f32>,
  pub rotation: f32,
  transform: Matrix3<f32>
}

impl Default for Transform2DComponent {
  fn default() -> Self {
    Self {
      scale: Vector2::new(1.0, 1.0),
      position: Vector2::new(0.0, 0.0),
      rotation: 0.0,
      transform: Matrix3::identity()
    }
  }
}

impl Transform2DComponent {
  pub fn new(scale: Vector2<f32>, position: Vector2<f32>, rotation: f32) -> Self {
    Self {
      scale,
      position,
      rotation,
      transform: Self::calculate_transform(
        &scale,
        &position,
        rotation
      )
    }
  }

  pub fn sync(&mut self) {
    self.transform = Self::calculate_transform(
      &self.scale,
      &self.position,
      self.rotation
    );
  }

  fn calculate_transform(
    scale: &Vector2<f32>,
    position: &Vector2<f32>,
    rotation: f32
  ) -> Matrix3<f32> {
    let scale = Matrix3::new_nonuniform_scaling(scale);
    // let scale = Matrix3::new_nonuniform_scaling(&Vector2::<f32>::new(
    //   1.0 / self.scale.x,
    //   1.0 / self.scale.y
    // ));

    let sin_a = rotation.sin();
    let cos_a = rotation.cos();
    let position = Matrix3::new_translation(&Vector2::<f32>::new(
      position.y * sin_a - position.x * cos_a,
      -position.x * sin_a - position.y * cos_a,
    ));
    let rotation = Matrix3::new_rotation(rotation);

    scale * position * rotation
  }
}