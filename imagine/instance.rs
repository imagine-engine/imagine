use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::render::RenderGraph;
use crate::animation::Interpolate;
use crate::world::World;
use crate::video::VideoComponent;

lazy_static! {
  pub static ref IMAGINE: Mutex<Instance> = Mutex::new(Instance {
    world: World::default(),
    animations: Vec::new(),
    render_graph: RenderGraph::default()
  });
}

pub struct Instance {
  pub world: World,
  animations: Vec<Box<dyn Interpolate + Send>>,
  render_graph: RenderGraph
}

impl Instance {
  pub fn wait(&mut self, t: f32) {
    self.world.age += t;
  }

  pub fn run(&mut self) {
    for video in self.world.iter_mut::<VideoComponent>() {}

    for t in 0..10 {
      for animation in self.animations.iter() {
        animation.interpolate(0.0, &mut self.world);
      }
      // self.render_graph.run(&self.world);
    }
  }
}