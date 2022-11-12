// https://64.github.io/tonemapping/#filmic-tone-mapping-operators

use crate::geometry::vector3::Vector3f;

use super::{
    QuantisedColor
};


fn uncharted2_tonemap_partial(color: &Vector3f) -> Vector3f {
    let output = color.clone();
    let a = 0.15;
    let b = 0.50;
    let c = 0.10;
    let d = 0.20;
    let e = 0.02;
    let f = 0.30;
    
    let tmp0 = output*(a*output+c*b)+d*e;
    let tmp1 = output*(a*output+b)+d*f;

    let tmp2 = e/f;

    Vector3f::new(
        tmp0.x / tmp1.x - tmp2,
        tmp0.y / tmp1.y - tmp2,
        tmp0.z / tmp1.z - tmp2
    )
}

fn uncharted2_filmic(v: &Vector3f) -> Vector3f {
    let exposure_bias = 2.0;
    let curr = uncharted2_tonemap_partial(&(v * exposure_bias));

    let w = Vector3f::new(11.2, 11.2, 11.2);
    let tmp0 = uncharted2_tonemap_partial(&w);
    let white_scale = Vector3f::new(
        1.0/tmp0.x,
        1.0/tmp0.y,
        1.0/tmp0.z
    );
    return curr * white_scale;
}

#[allow(unused)]
pub fn tonemap_filmic(framebuffer: &Vec<Vector3f>) -> Vec<QuantisedColor> {
    framebuffer.iter().map(|pixel| {
        let tonemapped = uncharted2_filmic(&pixel);
        QuantisedColor::new(
            tonemapped.x,
            tonemapped.y,
            tonemapped.z
        )
      }).collect()
}