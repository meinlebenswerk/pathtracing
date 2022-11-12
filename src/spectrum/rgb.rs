use crate::config::RaytracerFloat;

use super::CoefficientSpectrum;


pub type RGBSpectrum = CoefficientSpectrum<3>;

impl RGBSpectrum {
    #[allow(unused)]
    pub fn y(&self) -> RaytracerFloat {
        let y_weight: [RaytracerFloat; 3] = [ 0.212671, 0.715160, 0.072169 ];
        y_weight[0] * self.c[0] + y_weight[1] * self.c[1] + y_weight[2] * self.c[2]
    }
}

//     static RGBSpectrum FromSampled(const Float *lambda, const Float *v, int n) {
//         // Sort samples if unordered, use sorted for returned spectrum
//         if (!SpectrumSamplesSorted(lambda, v, n)) {
//             std::vector<Float> slambda(&lambda[0], &lambda[n]);
//             std::vector<Float> sv(&v[0], &v[n]);
//             SortSpectrumSamples(&slambda[0], &sv[0], n);
//             return FromSampled(&slambda[0], &sv[0], n);
//         }
//         Float xyz[3] = {0, 0, 0};
//         for (int i = 0; i < nCIESamples; ++i) {
//             Float val = InterpolateSpectrumSamples(lambda, v, n, CIE_lambda[i]);
//             xyz[0] += val * CIE_X[i];
//             xyz[1] += val * CIE_Y[i];
//             xyz[2] += val * CIE_Z[i];
//         }
//         Float scale = Float(CIE_lambda[nCIESamples - 1] - CIE_lambda[0]) /
//                       Float(CIE_Y_integral * nCIESamples);
//         xyz[0] *= scale;
//         xyz[1] *= scale;
//         xyz[2] *= scale;
//         return FromXYZ(xyz);
//     }