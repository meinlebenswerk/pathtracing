use std::{fs::File, io::Write};

use crate::vector3::Vector3;

const OPENEXR_MAGIC_NUMBER: [u8; 4] = [0x76, 0x2f, 0x31, 0x01];


fn write_openexr_attribute(attr_name: &str, attr_type: &str, attr_size: usize, attr_value: &[u8]) {
  let attrib_field_size = 0;
  
}

pub fn export_openexr(data: &[Vector3], size: (usize, usize), filename: &str) -> std::io::Result<()> {

  // Dump framebuffer.data
  let capacity = (size.0 * size.1 * 3) + 256;
  let mut buffer: Vec<u8> = Vec::with_capacity(capacity);


  // write the header:
  
  // magic number
  buffer[0] = OPENEXR_MAGIC_NUMBER[0];
  buffer[1] = OPENEXR_MAGIC_NUMBER[1];
  buffer[2] = OPENEXR_MAGIC_NUMBER[2];
  buffer[3] = OPENEXR_MAGIC_NUMBER[3];

  // Version field
  buffer[4] = 2;    // OpenEXR Version 2
  buffer[5] = 0x00; // Single-part scan line
  buffer[6] = 0;    // empty
  buffer[7] = 0;    // emtpy

  // Actual file header



  let mut file = File::create(filename)?;
  // file.write_all(b"Hello, world!")?;
  buffer.extend(format!("P6\n{} {}\n255\n", size.0, size.1).as_bytes().iter().cloned());
  // file.write_fmt(format_args!("P6\n{} {}\n255\n", size.0, size.1))?;

  // let max = 0xff as f32;
  // for pixel in data {
  //   let r = f32::sqrt(pixel.r());
  //   let g = f32::sqrt(pixel.g());
  //   let b = f32::sqrt(pixel.b());

  //   let ri = (clamp(r, 0.0, 0.999) * max) as u8;
  //   let gi = (clamp(g, 0.0, 0.999) * max) as u8;
  //   let bi = (clamp(b, 0.0, 0.999) * max) as u8;
  //   // file.write(&[ri, gi, bi])?;
  //   buffer.extend([ri, gi, bi].iter().cloned());
  // }

  file.write_all(&buffer)?;

  Ok(())
}