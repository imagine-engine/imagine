use crate::video::VideoContext;

pub struct VideoComponent {
  pub width: i32,
  pub height: i32,
  pub fps: i32,
  context: VideoContext
}

impl VideoComponent {
  pub fn new(width: i32, height: i32, fps: i32) -> Self {
    Self {
      width,
      height,
      fps,
      context: VideoContext::new()
    }
  }
}