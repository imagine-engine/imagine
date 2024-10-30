use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct Color {
  #[pyo3(get, set)]
  pub r: u8,
  #[pyo3(get, set)]
  pub g: u8,
  #[pyo3(get, set)]
  pub b: u8
}

#[pymethods]
impl Color {
  #[classattr]
  pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
  #[classattr]
  pub const WHITE: Color = Color { r: 255, g: 255, b: 255 };
  #[classattr]
  pub const BLUE: Color = Color { r: 33, g: 150, b: 243 };

  #[new]
  fn new(hex: String) -> PyResult<Self> {
    // let shift = if _____ { 1 } else { 0 }
    Ok(
      Self {
        r: u8::from_str_radix(&hex[0..1], 16).unwrap(),
        g: u8::from_str_radix(&hex[2..3], 16).unwrap(),
        b: u8::from_str_radix(&hex[4..5], 16).unwrap()
      }
    )
  }

  fn __eq__(&self, other: &Color) -> PyResult<bool> {
    Ok(self.r == other.r && self.g == other.g && self.b == other.b)
  }

  fn __repr__(&self) -> PyResult<String> {
    let r = format!("{:02X}", self.r);
    let g = format!("{:02X}", self.g);
    let b = format!("{:02X}", self.b);
    Ok(format!("#{}{}{}", r, g, b).to_string())
  }
}