extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{
  Type,
  Field,
  Token,
  ItemStruct,
  parse::Parser,
  parse_macro_input,
  punctuated::Punctuated
};

#[proc_macro_attribute]
pub fn register(args: TokenStream, input: TokenStream) -> TokenStream {
  let mut def = parse_macro_input!(input as ItemStruct);
  let components = parse_macro_input!(
    args with Punctuated<Type, Token![,]>::parse_terminated
  );
  let name = def.ident.clone();

  let mut fields = Vec::new();
  let mut pointers = Vec::new();
  let mut pointer_inits = Vec::new();
  let mut component_impls = Vec::new();

  if let syn::Fields::Named(ref struct_fields) = def.fields {
    for field in struct_fields.named.iter() {
      let vis = &field.vis;
      let field_name = &field.ident;
      let ty = &field.ty;
      fields.push(quote! { #vis #field_name: #ty, })
    }

    for (i, ty) in components.into_iter().enumerate() {
      if i >= 32 {
        panic!("only 32 components can be registered in total");
      }

      let n = i + 1;
      let field_name = format_ident!("column{}", n);

      fields.push(quote! { #field_name: HashMap<usize, #ty>, });
      pointers.push(quote! { #field_name: NonNull<HashMap<usize, #ty>>, });
      pointer_inits.push(quote! { #field_name: NonNull::from(&self.#field_name), });

      component_impls.push(quote! {
        impl Component for #ty {
          fn mask() -> u32 {
            0 | (1 << #n)
          }
        }

        impl ComponentSet<#ty> for #name {
          fn get_component(&self, id: usize) -> Option<&#ty> {
            self.#field_name.get(&id)
          }

          fn get_component_mut(&mut self, id: usize) -> Option<&mut #ty> {
            self.#field_name.get_mut(&id)
          }

          fn insert_component(&mut self, id: usize, component: #ty) {
            self.#field_name.insert(id, component);
          }

          fn push_component(&mut self, component: #ty) {
            self.#field_name.insert(self.max_entity_id, component);
          }

          fn delete_component(&mut self, id: usize) {
            self.#field_name.remove(&id);
          }

          fn iter_components(&self) -> Values<'_, usize, #ty> {
            self.#field_name.values()
          }

          fn iter_components_mut(&mut self) -> ValuesMut<'_, usize, #ty> {
            self.#field_name.values_mut()
          }
        }

        impl<T> QueryGet<#ty> for Queries<'_, T> {
          fn get<'a>(&self, id: usize) -> Option<&'a #ty> {
            unsafe { self.#field_name.as_ref().get(&id) }
          }
        }

        impl<T> QueryGetMut<#ty> for QueriesMut<'_, T> {
          fn get_mut<'a>(&mut self, id: usize) -> Option<&'a mut #ty> {
            unsafe { self.#field_name.as_mut().get_mut(&id) }
          }
        }
      });
    }
  }

  return quote! {
    use std::ptr::NonNull;
    use std::marker::PhantomData;
    use std::collections::hash_map::{Values, ValuesMut};

    pub trait Component {
      fn mask() -> u32;
    }

    pub trait ComponentSet<T> {
      fn push_component(&mut self, component: T) {}
      fn insert_component(&mut self, id: usize, component: T) {}
      fn get_component(&self, id: usize) -> Option<&T> { None }
      fn get_component_mut(&mut self, id: usize) -> Option<&mut T> { None }
      fn delete_component(&mut self, id: usize) {}
      fn iter_components(&self) -> Values<'_, usize, T>;
      fn iter_components_mut(&mut self) -> ValuesMut<'_, usize, T>;
    }

    pub trait AddBundle<T> {
      fn push_components(&mut self, components: T);
      fn insert_components(&mut self, id: usize, components: T);
    }

    #[derive(Default)]
    pub struct #name {
      #(#fields)*
    }

    pub trait QueryGet<T> {
      fn get<'a>(&self, id: usize) -> Option<&'a T>;
    }

    pub trait QueryGetMut<T> {
      fn get_mut<'a>(&mut self, id: usize) -> Option<&'a mut T>;
    }

    pub struct Queries<'a, T> {
      #(#pointers)*
      phantom: PhantomData<&'a T>,
      entities: Vec<usize>,
      index: usize
    }

    pub struct QueriesMut<'a, T> {
      #(#pointers)*
      phantom: PhantomData<&'a T>,
      entities: Vec<usize>,
      index: usize
    }

    impl #name {
      fn new_query<T>(&self, ids: Vec<usize>) -> Queries<'_, T> {
        Queries {
          #(#pointer_inits)*
          phantom: PhantomData,
          entities: ids,
          index: 0
        }
      }

      fn new_mut_query<T>(&self, ids: Vec<usize>) -> QueriesMut<'_, T> {
        QueriesMut {
          #(#pointer_inits)*
          phantom: PhantomData,
          entities: ids,
          index: 0
        }
      }
    }

    #(#component_impls)*
  }.into();
}