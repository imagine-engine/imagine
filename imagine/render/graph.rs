use std::any::Any;
use std::collections::{HashMap, HashSet};
use crate::render::{RenderContext, RenderOperation};

use crate::render3d::PhongPassBuilder;

pub struct RenderGraph {
  update_queue: Vec<String>,
  operations: HashMap<String, Box<dyn RenderOperation + Send>>,
  next: HashMap<String, Vec<String>>,
  pub context: RenderContext
}

impl RenderGraph {
  fn empty() -> Self {
    RenderGraph {
      update_queue: Vec::new(),
      operations: HashMap::new(),
      next: HashMap::new(),
      context: futures::executor::block_on(
        RenderContext::new()
      )
    }
  }

  pub fn default() -> Self {
    let mut graph = RenderGraph::empty();

    let phong_pass = PhongPassBuilder::new()
                      .camera("main_camera_3d")
                      .mesh_buffer("world_3d")
                      .output("main_output_texture")
                      .framebuffer("main_frame")
                      .build(&mut graph.context);
    graph.add_node("main_pass", phong_pass);

    graph
  }

  pub fn add_node<T: RenderOperation + Send + 'static>(&mut self, name: &str, node: T) {
    self.operations.insert(String::from(name), Box::new(node));
  }

  pub fn get_resource_mut<T: Any>(&mut self, name: &str) -> Option<&mut T> {
    if let Some(resource) = self.context.resources.get_mut(name) {
      self.update_queue.push(String::from(name));
      return resource.downcast_mut();
    }

    None
  }

  pub fn run(&mut self) {
    let mut visited: Vec<String> = Vec::new();

    while let Some(resource) = self.update_queue.pop() {
      let mut parents: Vec<String> = vec![resource];

      while parents.len() > 0 {
        // Collect all the operations to run
        let mut op_names: Vec<String> = Vec::new();
        for parent in &parents {
          if let Some(children) = self.next.get(parent) {
            for child in children.iter() {
              if visited.contains(child) {
                continue;
              }

              op_names.push(child.to_string());
              visited.push(child.to_string());
            }
          }
        }

        // Run all operations
        let mut level = Vec::new();
        for op_name in &op_names {
          if let Some(op) = self.operations.get(op_name) {
            level.push(op.run(&mut self.context));
          }
        }

        self.context.queue.submit(level);
        parents = op_names.clone();
      }
    }
  }
}