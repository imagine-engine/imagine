use nalgebra::{Vector2, Vector3, Matrix3, Matrix4};

use crate::render::primitives::{
  Vertex3D,
  Texture,
  CameraProjection
};

pub struct Transform3DComponent {
  pub scale: Vector3<f32>,
  pub position: Vector3<f32>,
  pub rotation: Vector3<f32>,
  pub transform: Matrix4<f32>
}

pub struct MeshComponent {
  pub vertices: Vec<Vertex3D>,
  pub indices: Vec<u32>
}

pub struct PhongComponent {
  pub opacity: f32,
  pub shininess: f32,
  pub reflectivity: f32,
  pub normal: Texture,
  pub diffuse: Texture,
  pub specular: Texture,
  pub binding: wgpu::BindGroup
}

pub struct PBRComponent {
  pub albedo: Texture,
  pub normal: Texture,
  pub specular: Texture,
  pub metallic: Texture,
  pub roughness: Texture,
  pub binding: wgpu::BindGroup
}

pub struct PerspectiveCameraComponent {
  pub aspect: f32,
  pub fov: f32,
  pub znear: f32,
  pub zfar: f32,
  pub projection: Matrix4<f32>
}

// pub struct OrthoCameraComponent {
//   pub aspect: f32,
//   pub fov: f32,
//   pub znear: f32,
//   pub zfar: f32,
//   pub projection: Matrix4<f32>
// }

impl Default for PerspectiveCameraComponent {
  fn default() -> Self {
    Self {
      aspect: 16.0 / 9.0,
      fov: 45.0,
      znear: 1.0,
      zfar: 1000.0,
      projection: Matrix4::new_perspective(16.0/9.0, 45.0, 1.0, 1000.0)
    }
  }
}

impl Default for Transform3DComponent {
  fn default() -> Self {
    Self {
      scale: Vector3::new(1.0, 1.0, 1.0),
      position: Vector3::new(0.0, 0.0, 0.0),
      rotation: Vector3::new(0.0, 0.0, 0.0),
      transform: Matrix4::identity()
    }
  }
}

impl Transform3DComponent {
  pub fn new(scale: Vector3<f32>, position: Vector3<f32>, rotation: Vector3<f32>) -> Self {
    let mut transform = Self {
      scale,
      position,
      rotation,
      transform: Matrix4::identity()
    };
    transform.sync();

    transform
  }

  pub fn sync(&mut self) {
    let scale = Matrix4::new_nonuniform_scaling(&self.scale);
    let position = Matrix4::new_translation(&self.position);
    let rotation = Matrix4::from_euler_angles(
      self.rotation.x,
      self.rotation.y,
      self.rotation.z
    );

    self.transform = scale * position * rotation;
  }
}