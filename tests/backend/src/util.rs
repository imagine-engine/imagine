use imagine::{
  render3d::Transform3DComponent,
  render3d::PerspectiveCameraComponent,
  testing::Vector3
};

use approx::AbsDiffEq;
use rand::{rngs::StdRng, Rng, SeedableRng};

pub trait Mock {
  fn generate(seed: usize) -> Self;
  fn approx_eq(a: &Self, b: &Self, threshold: f32) -> bool;
}

impl Mock for PerspectiveCameraComponent {
  fn generate(seed: usize) -> Self {
    Self::default()
  }

  fn approx_eq(a: &Self, b: &Self, threshold: f32) -> bool {
    if (a.aspect - b.aspect).abs() > threshold { return false; }
    if (a.fov - b.fov).abs() > threshold { return false; }
    if (a.znear - b.znear).abs() > threshold { return false; }
    if (a.zfar - b.zfar).abs() > threshold { return false; }

    true
  }
}

impl Mock for Transform3DComponent {
  fn generate(seed: usize) -> Self {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    Self::new(
      Vector3::repeat(rng.gen::<f32>()),
      Vector3::repeat(rng.gen::<f32>()),
      Vector3::repeat(rng.gen::<f32>())
    )
  }

  fn approx_eq(a: &Self, b: &Self, threshold: f32) -> bool {
    if a.scale.abs_diff_ne(&b.scale, threshold) { return false; }
    if a.position.abs_diff_ne(&b.position, threshold) { return false; }
    if a.rotation.abs_diff_ne(&b.rotation, threshold) { return false; }

    true
  }
}