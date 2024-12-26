use pyo3::prelude::*;
use std::slice::Iter;
use std::default::Default;
use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};

use crate::instance::IMAGINE;

use imagine_macros::register;

use std::sync::Mutex;

use crate::video::VideoComponent;
use crate::render3d::{
  MeshComponent,
  PhongComponent,
  PBRComponent,
  Transform3DComponent,
  PerspectiveCameraComponent,
  OrthoCameraComponent
};

use crate::path::StrokeLinecap;
use crate::path::{
  PathComponent,
  EllipseComponent,
  BackgroundComponent,
  Transform2DComponent,
  Camera2DComponent
};

use crate::color::Color;
use nalgebra::{Vector2, Vector3, Matrix3, Matrix4};

// struct Archetype {
//   pub mask: u32,
//   pub entities: Vec<usize>,
//   pub children: Vec<Archetype>
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
        Self: AddBundle<T>
  {
    let id = self.max_entity_id;
    self.push_components(bundle);

    self.archetypes.entry(T::mask())
                   .or_insert(HashSet::new())
                   .insert(id);
    self.max_entity_id += 1;

    id
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

//
impl<A, B> AddBundle<(A, B)> for World
  where
      A: Component,
      B: Component,
      Self: ComponentSet<A> + ComponentSet<B>
{
  fn push_components(&mut self, components: (A, B)) {
    self.push_component(components.0);
    self.push_component(components.1);
  }
  fn insert_components(&mut self, id: usize, components: (A, B)) {
    self.insert_component(id, components.0);
    self.insert_component(id, components.1);
  }
}
impl<A, B, C> AddBundle<(A, B, C)> for World
  where
      A: Component,
      B: Component,
      C: Component,
      Self: ComponentSet<A> + ComponentSet<B> + ComponentSet<C>
{
  fn push_components(&mut self, components: (A, B, C)) {
    self.push_component(components.0);
    self.push_component(components.1);
    self.push_component(components.2);
  }
  fn insert_components(&mut self, id: usize, components: (A, B, C)) {
    self.insert_component(id, components.0);
    self.insert_component(id, components.1);
    self.insert_component(id, components.2);
  }
}

// Will cut off early if any entity ID is wrong
impl<'a, T> Iterator for Queries<'a, T>
  where
      T: ComponentBundle<'a>,
      Self: GetBundle<'a, T>
{
  type Item = T::Get;

  fn next(&mut self) -> Option<Self::Item> {
    let bundle = self.get_bundle();
    self.index += 1;
    bundle
  }
}

impl<'a, T> Iterator for QueriesMut<'a, T>
  where
      T: ComponentBundle<'a>,
      Self: GetMutBundle<'a, T>
{
  type Item = T::GetMut;

  fn next(&mut self) -> Option<Self::Item> {
    let bundle = self.get_mut_bundle();
    self.index += 1;
    bundle
  }
}

pub trait ComponentBundle<'a> {
  type Get;
  type GetMut;
  fn mask() -> u32;
}
impl<'a, A, B> ComponentBundle<'a> for (A, B)
  where
      A: 'a + Component,
      B: 'a + Component
{
  type Get = (&'a A, &'a B);
  type GetMut = (&'a mut A, &'a mut B);
  fn mask() -> u32 { A::mask() | B::mask() }
}
impl<'a, A, B, C> ComponentBundle<'a> for (A, B, C)
  where
      A: 'a + Component,
      B: 'a + Component,
      C: 'a + Component
{
  type Get = (&'a A, &'a B, &'a C);
  type GetMut = (&'a mut A, &'a mut B, &'a mut C);
  fn mask() -> u32 { A::mask() | B::mask() | C::mask() }
}

trait GetBundle<'a, T: ComponentBundle<'a>> {
  fn get_bundle(&self) -> Option<T::Get>;
}
impl<'a, A, B> GetBundle<'a, (A, B)> for Queries<'a, (A, B)>
  where
      A: Component,
      B: Component,
      Self: QueryGet<A> + QueryGet<B>
{
  fn get_bundle(&self) -> Option<(&'a A, &'a B)> {
    if let (Some(c_a), Some(c_b)) = (self.get(self.index), self.get(self.index)) {
      return Some((c_a, c_b));
    }

    None
  }
}

trait GetMutBundle<'a, T: ComponentBundle<'a>> {
  fn get_mut_bundle(&mut self) -> Option<T::GetMut>;
}
impl<'a, A, B> GetMutBundle<'a, (A, B)> for QueriesMut<'a, (A, B)>
  where
      A: Component,
      B: Component,
      Self: QueryGetMut<A> + QueryGetMut<B>
{
  fn get_mut_bundle(&mut self) -> Option<(&'a mut A, &'a mut B)> {
    if let (Some(a), Some(b)) = (
      self.get_mut(self.index),
      self.get_mut(self.index)
    ) {
      return Some((a, b));
    }

    None
  }
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
