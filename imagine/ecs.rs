use crate::world::{
  World,
  Queries,
  QueriesMut,
  QueryGet,
  QueryGetMut
};
use std::collections::hash_map::{
  Values,
  ValuesMut
};
use seq_macro::seq;

pub trait Component {
  fn mask() -> u32;
}

pub trait ComponentSet<T> {
  fn push_component(&mut self, component: T);
  fn insert_component(&mut self, id: usize, component: T);
  fn get_component(&self, id: usize) -> Option<&T>;
  fn get_component_mut(&mut self, id: usize) -> Option<&mut T>;
  fn delete_component(&mut self, id: usize) -> Option<T>;
  fn iter_components(&self) -> Values<'_, usize, T>;
  fn iter_components_mut(&mut self) -> ValuesMut<'_, usize, T>;
}

pub trait HandleBundle<T> {
  fn delete_components(&mut self, id: usize) -> Option<T>;
  fn push_components(&mut self, components: T);
  fn insert_components(&mut self, id: usize, components: T);
}

#[macro_export]
macro_rules! impl_handle_bundle {
  ($n:expr, $($types:ident),+) => {
    impl<$($types),+> HandleBundle<($($types),+)> for World
      where
          Self: Sized $(+ ComponentSet<$types>)+,
          $($types: Component),+
    {
      fn push_components(&mut self, components: ($($types),+)) {
        seq!(N in 0..$n { self.push_component(components.N); });
      }

      fn insert_components(&mut self, id: usize, components: ($($types),+)) {
        seq!(N in 0..$n { self.insert_component(id, components.N); });
      }

      fn delete_components(&mut self, id: usize) -> Option<($($types),+)> {
        if let seq!(N in 1..=$n { (#(Some(c~N),)*) }) = ($(
          ComponentSet::<$types>::delete_component(self, id),
        )+) {
          return seq!(N in 1..=$n { Some((#(c~N,)*)) });
        }
    
        None
      }
    }
  };
}
impl_handle_bundle!(2, A, B);
impl_handle_bundle!(3, A, B, C);
impl_handle_bundle!(4, A, B, C, D);
impl_handle_bundle!(5, A, B, C, D, E);

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

#[macro_export]
macro_rules! impl_component_bundle {
  ($($types:ident),+) => {
    impl<'a, $($types),+> ComponentBundle<'a> for ($($types),+)
      where
          $($types: 'a + Component),+
    {
      type Get = ($(&'a $types,)+);
      type GetMut = ($(&'a mut $types,)+);
      fn mask() -> u32 { 0 $(| $types::mask())+ }
    }
  };
}
impl_component_bundle!(A, B);
impl_component_bundle!(A, B, C);
impl_component_bundle!(A, B, C, D);
impl_component_bundle!(A, B, C, D, E);

trait GetBundle<'a, T: ComponentBundle<'a>> {
  fn get_bundle(&self) -> Option<T::Get>;
}

#[macro_export]
macro_rules! impl_get_bundle {
  ($n:expr, $($types:ident),+) => {
    impl<'a, $($types,)+> GetBundle<'a, ($($types,)+)> for Queries<'a, ($($types,)+)>
      where
          Self: Sized $(+ QueryGet<$types>)+,
          $($types: Component,)+
    {
      fn get_bundle(&self) -> Option<($(&'a $types,)+)> {
        if let seq!(N in 1..=$n { (#(Some(c~N),)*) }) = ($(
          QueryGet::<$types>::get(self, self.index),
        )+) {
          return seq!(N in 1..=$n { Some((#(c~N,)*)) });
        }

        None
      }
    }
  };
}
impl_get_bundle!(2, A, B);
impl_get_bundle!(3, A, B, C);
impl_get_bundle!(4, A, B, C, D);
impl_get_bundle!(5, A, B, C, D, E);

trait GetMutBundle<'a, T: ComponentBundle<'a>> {
  fn get_mut_bundle(&mut self) -> Option<T::GetMut>;
}

#[macro_export]
macro_rules! impl_get_mut_bundle {
  ($n:expr, $($types:ident),+) => {
    impl<'a, $($types,)+> GetMutBundle<'a, ($($types,)+)> for QueriesMut<'a, ($($types,)+)>
      where
          Self: Sized $(+ QueryGetMut<$types>)+,
          $($types: Component,)+
    {
      fn get_mut_bundle(&mut self) -> Option<($(&'a mut $types,)+)> {
        if let seq!(N in 1..=$n { (#(Some(c~N),)*) }) = ($(
          QueryGetMut::<$types>::get_mut(self, self.index),
        )+) {
          return seq!(N in 1..=$n { Some((#(c~N,)*)) });
        }

        None
      }
    }
  };
}
impl_get_mut_bundle!(2, A, B);
impl_get_mut_bundle!(3, A, B, C);
impl_get_mut_bundle!(4, A, B, C, D);
impl_get_mut_bundle!(5, A, B, C, D, E);