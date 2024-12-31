use crate::{
  render3d::MeshComponent,
  render3d::primitives::Vertex3D,
  render3d::Transform3DComponent,
  render3d::PerspectiveCameraComponent,
  render3d::OrthoCameraComponent
};
pub use nalgebra::Vector3;

use approx::AbsDiffEq;
use rand::{rngs::StdRng, Rng, SeedableRng};

macro_rules! assert_approx_eq {
  ($a:expr, $b:expr) => {
    assert!($a.approx_eq($b, f32::EPSILON));
  };
}
pub(crate) use assert_approx_eq;

pub trait Mock {
  fn generate(seed: usize) -> Self;
  fn approx_eq(&self, other: &Self, epsilon: f32) -> bool;
}

impl Mock for PerspectiveCameraComponent {
  fn generate(seed: usize) -> Self {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    Self::new(
      rng.gen_range(0.5..2.0),
      rng.gen_range(54.0..360.0),
      rng.gen_range(0.1..10.0),
      rng.gen_range(100.0..10_000.0)
    )
  }

  fn approx_eq(&self, other: &Self, epsilon: f32) -> bool {
    if (self.aspect - other.aspect).abs() > epsilon { return false; }
    if (self.fov - other.fov).abs() > epsilon { return false; }
    if (self.near - other.near).abs() > epsilon { return false; }
    if (self.far - other.far).abs() > epsilon { return false; }

    true
  }
}

impl Mock for OrthoCameraComponent {
  fn generate(seed: usize) -> Self {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    Self::new(
      -rng.gen_range(500.0..1000.0),
      rng.gen_range(500.0..1000.0),
      rng.gen_range(200.0..500.0),
      -rng.gen_range(200.0..500.0),
      rng.gen_range(0.1..10.0),
      rng.gen_range(100.0..10_000.0)
    )
  }

  fn approx_eq(&self, other: &Self, epsilon: f32) -> bool {
    if (self.left - other.left).abs() > epsilon { return false; }
    if (self.right - other.right).abs() > epsilon { return false; }
    if (self.bottom - other.bottom).abs() > epsilon { return false; }
    if (self.top - other.top).abs() > epsilon { return false; }
    if (self.near - other.near).abs() > epsilon { return false; }
    if (self.far - other.far).abs() > epsilon { return false; }

    true
  }
}

impl Mock for Transform3DComponent {
  fn generate(seed: usize) -> Self {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    Self::new(
      Vector3::repeat(rng.gen()),
      Vector3::repeat(rng.gen()),
      Vector3::repeat(rng.gen())
    )
  }

  fn approx_eq(&self, other: &Self, epsilon: f32) -> bool {
    if self.scale.abs_diff_ne(&other.scale, epsilon) { return false; }
    if self.position.abs_diff_ne(&other.position, epsilon) { return false; }
    if self.rotation.abs_diff_ne(&other.rotation, epsilon) { return false; }

    true
  }
}

impl Mock for MeshComponent {
  fn generate(seed: usize) -> Self {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let v_count: u32 = rng.gen_range(50..200);
    let i_count: u32 = rng.gen_range(20..50);

    Self {
      vertices: (0..v_count).map(|v| Vertex3D {
        position: [rng.gen_range(-200.0..200.0); 3],
        normal: [rng.gen_range(0.0..360.0); 3],
        uv: [rng.gen(); 2]
      }).collect(),
      indices: (0..i_count).map(|v| rng.gen_range(0..v_count)).collect()
    }
  }

  fn approx_eq(&self, other: &Self, epsilon: f32) -> bool {
    for (a, b) in self.vertices.iter().zip(other.vertices.iter()) {
      let v_a: Vec<f32> = a.position.iter()
                          .chain(a.normal.iter())
                          .chain(a.uv.iter())
                          .copied().collect();

      let v_b: Vec<f32> = b.position.iter()
                          .chain(b.normal.iter())
                          .chain(b.uv.iter())
                          .copied().collect();

      if v_a.iter().zip(v_b.iter()).any(|(x, y)| (x - y).abs() < epsilon) {
        return false;
      }
    }

    self.indices.iter()
                .zip(other.indices.iter())
                .all(|(a, b)| a == b)
  }
}

// impl Mock for _____ {
//   fn generate(seed: usize) -> Self {
//   }

//   fn approx_eq(&self, other: &Self, epsilon: f32) -> bool {
//   }
// }