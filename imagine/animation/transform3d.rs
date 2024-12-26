use nalgebra::{Vector3, Matrix4};

use crate::world::World;
use crate::instance::IMAGINE;
use crate::world::ComponentSet;
use crate::render3d::Transform3DComponent;
use crate::animation::Interpolate;

pub struct TransformAnimation3D {
  pub entity: usize,
  pub duration: f32,
  pub scale: Option<(Vector3<f32>, Vector3<f32>)>,
  pub position: Option<(Vector3<f32>, Vector3<f32>)>,
  pub rotation: Option<(Vector3<f32>, Vector3<f32>)>
}

impl TransformAnimation3D {
  fn new(
    entity: usize,
    duration: f32,
    scale: Option<Vector3<f32>>,
    position: Option<Vector3<f32>>,
    rotation: Option<Vector3<f32>>
  ) -> Self {
    let mut animation = Self {
      entity,
      duration,
      scale: None,
      position: None,
      rotation: None
    };

    if let Some(target) = IMAGINE.lock().unwrap().world.get::<Transform3DComponent>(entity) {
      if let Some(new_scale) = scale {
        animation.scale = Some((target.scale, new_scale));
      }
      if let Some(new_position) = position {
        animation.position = Some((target.position, new_position));
      }
      if let Some(new_rotation) = rotation {
        animation.rotation = Some((target.rotation, new_rotation));
      }
    }

    animation
  }
}

impl Interpolate for TransformAnimation3D {
  fn interpolate(&self, t: f32, world: &mut World) {
    if let Some(mut target) = world.get_mut::<Transform3DComponent>(self.entity) {
      if let Some(s) = self.scale {
        target.scale = s.0.lerp(&s.1, t);
      }

      if let Some(p) = self.position {
        target.position = p.0.lerp(&p.1, t);
      }

      if let Some(r) = self.rotation {
        target.rotation = r.0.lerp(&r.1, t);
      }

      target.sync();
    }
  }
}