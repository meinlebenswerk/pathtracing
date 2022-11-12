// https://64.github.io/tonemapping/#filmic-tone-mapping-operators

use crate::geometry::vector::Vector3f;

use super::{
    QuantisedColor
};


fn uncharted2_tonemap_partial(color: &Vector3f) -> Vector3f {
    let output = color.clone();
    let A = 0.15;
    let B = 0.50;
    let C = 0.10;
    let D = 0.20;
    let E = 0.02;
    let F = 0.30;
    
    let tmp0 = output*(A*output+C*B)+D*E;
    let tmp1 = output*(A*output+B)+D*F;

    let tmp2 = E/F;

    Vector3f::new(
        tmp0.x / tmp1.x - tmp2,
        tmp0.y / tmp1.y - tmp2,
        tmp0.z / tmp1.z - tmp2
    )
}

fn uncharted2_filmic(v: &Vector3f) -> Vector3f {
    let exposure_bias = 2.0;
    let curr = uncharted2_tonemap_partial(&(v * exposure_bias));

    let W = Vector3f::new(11.2, 11.2, 11.2);
    let tmp0 = uncharted2_tonemap_partial(&W);
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