use crate::world::*;
use crate::render3d::{
  MeshComponent,
  PBRComponent,
  PhongComponent,
  Transform3DComponent,
  PerspectiveCameraComponent,
  OrthoCameraComponent
};
use super::util::{Mock, assert_approx_eq};

use rand::{Rng, seq::SliceRandom};

use std::collections::HashSet;
use std::iter::FromIterator;

#[test]
fn basic_add_get() {
  let mut world = World::default();

  for seed in 0..100 {
    world.add_entity::<(PerspectiveCameraComponent, Transform3DComponent)>((
      Mock::generate(seed),
      Mock::generate(seed)
    ));
  }

  let mut seeds: Vec<usize> = (0..100).collect();
  let mut rng = rand::thread_rng();
  seeds.shuffle(&mut rng);
  for seed in seeds.iter() {
    let actual_camera = world.get::<PerspectiveCameraComponent>(*seed);
    let expected_camera: PerspectiveCameraComponent = Mock::generate(*seed);
    assert!(actual_camera.is_some());
    assert_approx_eq!(expected_camera, actual_camera.unwrap());

    let actual_transform = world.get::<Transform3DComponent>(*seed);
    let expected_transform: Transform3DComponent = Mock::generate(*seed);
    assert!(actual_transform.is_some());
    assert_approx_eq!(expected_transform, actual_transform.unwrap());
  }
}

#[test]
fn basic_delete() {
  let mut world = World::default();

  for seed in 0..100 {
    world.add_entity::<(PerspectiveCameraComponent, Transform3DComponent)>((
      Mock::generate(seed),
      Mock::generate(seed)
    ));
  }

  let mut seeds: Vec<usize> = (0..100).collect();
  let mut rng = rand::thread_rng();
  seeds.shuffle(&mut rng);
  for seed in seeds.iter() {
    assert!(world.get::<PerspectiveCameraComponent>(*seed).is_some());
    assert!(world.get::<Transform3DComponent>(*seed).is_some());

    let entity = world.delete::<(PerspectiveCameraComponent, Transform3DComponent)>(*seed);
    assert!(entity.is_some());

    let (actual_camera, actual_transform) = entity.unwrap();
    let expected_camera: PerspectiveCameraComponent = Mock::generate(*seed);
    let expected_transform: Transform3DComponent = Mock::generate(*seed);
    assert_approx_eq!(expected_camera, &actual_camera);
    assert_approx_eq!(expected_transform, &actual_transform);

    assert!(world.get::<PerspectiveCameraComponent>(*seed).is_none());
    assert!(world.get::<Transform3DComponent>(*seed).is_none());
  }
}

#[test]
fn basic_query() {
  let mut world = World::default();

  let mut mesh: HashSet<usize> = HashSet::new();
  let mut transforms: HashSet<usize> = HashSet::new();
  let mut perspective: HashSet<usize> = HashSet::new();
  let mut ortho: HashSet<usize> = HashSet::new();

  let mut rng = rand::thread_rng();
  for seed in 0..50 {
    match rng.gen_range(0..3) {
      0 => {
        let id = world.add_entity::<(MeshComponent, Transform3DComponent)>((
          Mock::generate(seed),
          Mock::generate(seed)
        ));
        mesh.insert(id);
        transforms.insert(id);
      },
      1 => {
        let id = world.add_entity::<(PerspectiveCameraComponent, Transform3DComponent)>((
          Mock::generate(seed),
          Mock::generate(seed)
        ));
        transforms.insert(id);
        perspective.insert(id);
      },
      _ => {
        let id = world.add_entity::<(MeshComponent, Transform3DComponent, OrthoCameraComponent)>((
          Mock::generate(seed),
          Mock::generate(seed),
          Mock::generate(seed)
        ));
        mesh.insert(id);
        transforms.insert(id);
        ortho.insert(id);
      }
    }
  }

  assert_eq!(
    mesh.intersection(&transforms)
        .copied()
        .collect::<HashSet<usize>>(),
    HashSet::from_iter(
      world.find_entities::<(MeshComponent, Transform3DComponent)>()
    ),
    "incorrect mesh/transform query"
  );

  assert_eq!(
    perspective.intersection(&transforms)
               .copied()
               .collect::<HashSet<usize>>(),
    HashSet::from_iter(
      world.find_entities::<(PerspectiveCameraComponent, Transform3DComponent)>()
    ),
    "incorrect perspective camera/transform query"
  );

  assert_eq!(
    mesh.intersection(&transforms)
        .copied()
        .collect::<HashSet<usize>>()
        .intersection(&ortho)
        .copied()
        .collect::<HashSet<usize>>(),
    HashSet::from_iter(
      world.find_entities::<(MeshComponent, Transform3DComponent, OrthoCameraComponent)>()
    ),
    "incorrect mesh/orthographic camera/transform query"
  );
}