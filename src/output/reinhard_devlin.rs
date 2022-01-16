use crate::geometry::vector::Vector3f;

use super::{
  QuantisedColor,
  rgb_to_luminance
};

// Code was partially stolen from
// https://github.com/tizian/tonemapper
// And the original paper

#[allow(dead_code)]
pub fn tonemap_rd(framebuffer: &Vec<Vector3f>, f_: f32, c: f32, a: f32) -> Vec<QuantisedColor> {
  let mut luminance_avg = 0.0;
  let mut luminance_min = f32::INFINITY;
  let mut luminance_max = f32::EPSILON;
  let mut average_color = Vector3f::default();

  for pixel in framebuffer {
    let luminance = rgb_to_luminance(pixel);
    luminance_avg += luminance;
    average_color += *pixel;
    if luminance > luminance_max { luminance_max = luminance; }
    if luminance < luminance_min { luminance_min = luminance; }
  }

  luminance_avg /= framebuffer.len() as f32;
  average_color /= framebuffer.len() as f32;

  let lmax = luminance_max.ln();
  let lmin = luminance_min.ln();
  let lav = luminance_avg.ln();

  let k = (lmax - lav) / (lmax - lmin);
  let m = 0.3 + 0.7 * k.powf(1.4);
  let f = f32::exp(-f_);

  framebuffer.iter().map(|pixel| {
    let luminance = rgb_to_luminance(pixel);

    let mut values = [0.0; 3];
    for i in 0..2 {
      let i_l = (c * pixel[i]) + (1.0 - c) * luminance;
      let i_g = (c * average_color[i]) + (1.0 - c) * luminance_avg;
      let i_a = (a * i_l) + (1.0 - a) * i_g;
      values[i] = pixel[i] / (pixel[i] + (f * i_a).powf(m));
    }

    QuantisedColor::new(
      values[0],
      values[1],
      values[2]
    )
  }).collect()
}