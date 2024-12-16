use imagine::world::World;
use imagine::render3d::{
  Transform3DComponent,
  PerspectiveCameraComponent
};
use super::util::Mock;

use rand::seq::SliceRandom;
use rand::thread_rng;

#[test]
fn add_get() {
  let mut world = World::default();

  for seed in 0..100 {
    let entity: (PerspectiveCameraComponent, Transform3DComponent) = (
      Mock::generate(seed),
      Mock::generate(seed)
    );
    world.add_entity(entity);
  }

  let mut seeds: Vec<usize> = (0..100).collect();
  let mut rng = thread_rng();
  seeds.shuffle(&mut rng);
  for seed in seeds.iter() {
    let actual_camera = world.get::<PerspectiveCameraComponent>(*seed);
    let expected_camera: PerspectiveCameraComponent = Mock::generate(*seed);
    assert!(actual_camera.is_some());
    assert!(Mock::approx_eq(&expected_camera, actual_camera.unwrap(), 0.1));

    let actual_transform = world.get::<Transform3DComponent>(*seed);
    let expected_transform: Transform3DComponent = Mock::generate(*seed);
    assert!(actual_transform.is_some());
    assert!(Mock::approx_eq(&expected_transform, actual_transform.unwrap(), 0.1));
  }
}

// #[test]
// fn add_get_with_delete() {}