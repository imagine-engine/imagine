/*******************************************************************************
  render/mod.rs
********************************************************************************
  Copyright 2024 Menelik Eyasu

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.
*******************************************************************************/

mod graph;
mod context;
mod resource;
mod operation;
pub mod primitives;

pub use graph::RenderGraph;
pub use context::RenderContext;
pub use resource::RenderResource;
pub use operation::RenderOperation;

use crate::world::World;

pub async fn render(world: &World, graph: &mut RenderGraph) -> Vec<u8> {
  graph.run(world);

  match graph.resources.get("framebuffer") {
    Some(RenderResource::Buffer(buffer)) => {
      let mut pixels = Vec::new();
      {
        let buffer_slice = buffer.slice(..);

        let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
          tx.send(result).unwrap();
        });
        graph.context.device.poll(wgpu::Maintain::Wait);
        rx.receive().await.unwrap().unwrap();

        pixels = buffer_slice.get_mapped_range().to_vec();
      }

      buffer.unmap();

      pixels
    },
    _ => vec![255; 4 * graph.context.frame_size()]
  }
}