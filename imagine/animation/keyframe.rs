use pyo3::prelude::*;

// #[pyfunction]
// #[pyo3(signature=(duration=5.0))]
// pub fn interpolate(duration: f32) -> KeyframeIterator {
//   IMAGINE.lock().unwrap().keyframes(duration)
// }

#[pyclass]
pub struct Keyframe {
  #[pyo3(get)]
  pub t: f32
}

#[pyclass]
pub struct KeyframeIterator {
  pub i: usize,
  pub delta: f32,
  pub frames: usize
}

#[pymethods]
impl KeyframeIterator {
  fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
    slf
  }

  fn __next__(mut slf: PyRefMut<Self>) -> Option<Keyframe> {
    // IMAGINE.lock().unwrap().next_frame();
    slf.i += 1;

    if slf.i > slf.frames {
      return None;
    }

    Some(Keyframe {
      t: slf.i as f32 / slf.frames as f32
    })
  }
}