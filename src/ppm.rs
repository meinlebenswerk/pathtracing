
use std::fs::File;
use std::io::prelude::*;

use crate::geometry::vector::Vector3f;
use crate::output::QuantisedColor;

fn clamp(val: f32, min: f32, max: f32) -> f32 {
  f32::max(min, f32::min(val, max))
}

// This is a lot faster on WSL due to the bad fs-translation
#[allow(dead_code)]
pub fn dump_ppm(data: &[Vector3f], size: (usize, usize), filename: &str) -> std::io::Result<()> {

  // Dump framebuffer.data
  let capacity = (size.0 * size.1 * 3) + 256;
  let mut buffer: Vec<u8> = Vec::with_capacity(capacity);

  let mut file = File::create(filename)?;
  // file.write_all(b"Hello, world!")?;
  buffer.extend(format!("P6\n{} {}\n255\n", size.0, size.1).as_bytes().iter().cloned());
  // file.write_fmt(format_args!("P6\n{} {}\n255\n", size.0, size.1))?;

  let max = 0xff as f32;
  for pixel in data {
    let r = f32::sqrt(pixel.x);
    let g = f32::sqrt(pixel.y);
    let b = f32::sqrt(pixel.z);

    let ri = (clamp(r, 0.0, 0.999) * max) as u8;
    let gi = (clamp(g, 0.0, 0.999) * max) as u8;
    let bi = (clamp(b, 0.0, 0.999) * max) as u8;
    // file.write(&[ri, gi, bi])?;
    buffer.extend([ri, gi, bi].iter().cloned());
  }

  file.write_all(&buffer)?;

  Ok(())
}

#[allow(dead_code)]
pub fn dump_ppm_raw(data: &[QuantisedColor], size: (usize, usize), filename: &str) -> std::io::Result<()> {
  let capacity = (size.0 * size.1 * 3) + 256;
  let mut buffer: Vec<u8> = Vec::with_capacity(capacity);

  let mut file = File::create(filename)?;
  buffer.extend(format!("P6\n{} {}\n255\n", size.0, size.1).as_bytes().iter().cloned());
  for pixel in data {
    buffer.extend([pixel.r, pixel.g, pixel.b].iter().cloned());
  }

  file.write_all(&buffer)?;

  Ok(())
}