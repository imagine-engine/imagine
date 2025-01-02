use pyo3::prelude::*;
use std::slice::Iter;
use std::default::Default;
use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};

use crate::video::VideoComponent;
use crate::render3d::{
  MeshComponent,
  PhongComponent,
  PBRComponent,
  Transform3DComponent,
  PerspectiveCameraComponent,
  OrthoCameraComponent
};

use crate::path::{
  PathComponent,
  EllipseComponent,
  BackgroundComponent,
  Transform2DComponent,
  Camera2DComponent
};

use crate::instance::IMAGINE;
use imagine_macros::register;

// struct Archetype {
//   pub mask: u32,
//   pub entities: HashSet<usize>,
//   pub children: Vec<u32>
// }

#[register(
  MeshComponent,
  PhongComponent,
  PBRComponent,
  Transform3DComponent,
  PerspectiveCameraComponent,
  OrthoCameraComponent,
  PathComponent,
  EllipseComponent,
  BackgroundComponent,
  Transform2DComponent,
  Camera2DComponent,
  VideoComponent
)]
pub struct World {
  pub age: f32,
  archetypes: HashMap<u32, HashSet<usize>>,
  max_entity_id: usize
}

impl World {
  pub fn add_entity<T>(&mut self, bundle: T) -> usize
    where
        T: for<'a> ComponentBundle<'a>,
        Self: HandleBundle<T>
  {
    let id = self.max_entity_id;
    self.push_components(bundle);

    self.archetypes.entry(T::mask())
                   .or_insert(HashSet::new())
                   .insert(id);
    self.max_entity_id += 1;

    id
  }

  pub fn delete<T>(&mut self, id: usize) -> Option<T>
    where
        T: for<'a> ComponentBundle<'a>,
        Self: HandleBundle<T>
  {
    if let Some(archetype) = self.archetypes.get_mut(&T::mask()) {
      if archetype.remove(&id) {
        return self.delete_components(id);
      }
    }

    None
  }

  pub fn get<T>(&self, id: usize) -> Option<&T>
    where
        Self: ComponentSet<T>
  {
    self.get_component(id)
  }

  pub fn get_mut<T>(&mut self, id: usize) -> Option<&mut T>
    where
        Self: ComponentSet<T>
  {
    self.get_component_mut(id)
  }

  pub fn iter<T>(&self) -> Values<'_, usize, T>
    where
        Self: ComponentSet<T>
  {
    self.iter_components()
  }

  pub fn iter_mut<T>(&mut self) -> ValuesMut<'_, usize, T>
    where
        Self: ComponentSet<T>
  {
    self.iter_components_mut()
  }

  pub fn query<'a, T: ComponentBundle<'a>>(&self) -> Queries<'_, T> {
    self.new_query(self.find_entities::<T>())
  }

  pub fn query_mut<'a, T: ComponentBundle<'a>>(&mut self) -> QueriesMut<'_, T> {
    self.new_mut_query(self.find_entities::<T>())
  }

  pub fn find_entities<'a, T: ComponentBundle<'a>>(&self) -> Vec<usize> {
    let bundle_mask = T::mask();
    let mut entities: Vec<usize> = Vec::new();
    for (archetype, entity_ids) in self.archetypes.iter() {
      if bundle_mask & archetype == bundle_mask {
        entities.extend(entity_ids);
      }
    }

    entities
  }

  // pub fn delete() -> ____ {}
}

// Python wrapper
#[pyclass(name="World")]
pub struct PyWorld;

#[pymethods]
impl PyWorld {
  #[getter(age)]
  fn get_age(&self) -> PyResult<f32> {
    Ok(IMAGINE.lock().unwrap().world.age)
  }

  // // #[cfg(debug_assertions)]
  // #[getter(points)]
  // fn get_points(&self) -> PyResult<Vec<f32>> {
  //   Ok(IMAGINE.lock().unwrap().world.points.clone())
  // }

  // // #[cfg(debug_assertions)]
  // #[getter(controls)]
  // fn get_controls(&self) -> PyResult<Vec<u8>> {
  //   Ok(IMAGINE.lock().unwrap().world.controls.clone())
  // }
}
