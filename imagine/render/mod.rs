mod graph;
mod context;
mod operation;
pub mod primitives;

pub use graph::*;
pub use context::*;
pub use operation::*;
use crate::world::World;

pub async fn render(world: &World, graph: &mut RenderGraph) -> Vec<u8> {
  // graph.run();

  // match graph.context.get::<Buffer>("framebuffer") {
  //   Some(Buffer(buffer)) => {
  //     let mut pixels = Vec::new();
  //     {
  //       let buffer_slice = buffer.slice(..);

  //       let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
  //       buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
  //         tx.send(result).unwrap();
  //       });
  //       graph.context.device.poll(wgpu::Maintain::Wait);
  //       rx.receive().await.unwrap().unwrap();

  //       pixels = buffer_slice.get_mapped_range().to_vec();
  //     }

  //     buffer.unmap();

  //     pixels
  //   },
  //   _ => vec![255; 4 * graph.context.frame_size()]
  // }
  vec![255; 4 * graph.context.frame_size()]
}