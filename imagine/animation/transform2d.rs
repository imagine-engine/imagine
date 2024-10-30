use nalgebra::{Vector2, Matrix3};

use crate::world::World;
use crate::instance::IMAGINE;
use crate::world::ComponentSet;
use crate::path::Transform2DComponent;
use crate::animation::Interpolate;

pub struct TransformAnimation2D {
  pub entity: usize,
  pub duration: f32,
  pub scale: Option<(Vector2<f32>, Vector2<f32>)>,
  pub position: Option<(Vector2<f32>, Vector2<f32>)>,
  pub rotation: Option<(f32, f32)>
}

impl TransformAnimation2D {
  fn new(
    entity: usize,
    duration: f32,
    scale: Option<Vector2<f32>>,
    position: Option<Vector2<f32>>,
    rotation: Option<f32>
  ) -> Self {
    let mut animation = Self {
      entity,
      duration,
      scale: None,
      position: None,
      rotation: None
    };

    if let Some(target) = IMAGINE.lock().unwrap().world.get::<Transform2DComponent>(entity) {
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

impl Interpolate for TransformAnimation2D {
  fn interpolate(&self, t: f32, world: &mut World) {
    if let Some(mut target) = world.get_mut::<Transform2DComponent>(self.entity) {
      if let Some(s) = self.scale {
        target.scale = s.0.lerp(&s.1, t);
      }

      if let Some(p) = self.position {
        target.position = p.0.lerp(&p.1, t);
      }

      if let Some(r) = self.rotation {
        target.rotation = r.0 + t * (r.1 - r.0);
      }

      let sin_a = target.rotation.sin();
      let cos_a = target.rotation.cos();
      let scale = Matrix3::new_nonuniform_scaling(&Vector2::<f32>::new(
        1.0 / target.scale.x,
        1.0 / target.scale.y
      ));
      let position = Matrix3::new_translation(&Vector2::<f32>::new(
        target.position.y*sin_a - target.position.x*cos_a,
        -target.position.x*sin_a - target.position.y*cos_a,
      ));
      let rotation = Matrix3::new_rotation(target.rotation);
      target.transform = scale * position * rotation;
    }
  }
}