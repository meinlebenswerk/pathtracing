use std::{ops::{AddAssign, Add, Sub, SubAssign, Div, Mul, MulAssign, DivAssign, Neg}, fmt};

use crate::config::RaytracerFloat;

pub mod rgb;

#[derive(Copy, Clone)]
#[allow(unused)]
pub struct CoefficientSpectrum<const NSAMPLES: usize> {
    n_samples: usize,
    c: [RaytracerFloat; NSAMPLES],
}

impl<const NSAMPLES: usize> CoefficientSpectrum<NSAMPLES> {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            n_samples: NSAMPLES,
            c: [0.0; NSAMPLES]
        }
    }

    #[allow(unused)]
    pub fn is_black(&self) -> bool {
        for i in 0..NSAMPLES {
            if self.c[i] != 0.0 {
                return false
            }
        }
        true
    }

    #[allow(unused)]
    pub fn sqrt(&self) -> Self {
        let mut result = self.clone();
        for i in 0..NSAMPLES {
            result.c[i] = result.c[i].sqrt();
        }
        result
    }

    #[allow(unused)]
    pub fn powf(&self, e: RaytracerFloat) -> Self {
        let mut result = self.clone();
        for i in 0..NSAMPLES {
            result.c[i] = result.c[i].powf(e);
        }
        result
    }

    #[allow(unused)]
    pub fn clamp(&self, min: RaytracerFloat, max: RaytracerFloat) -> Self {
        let mut result = self.clone();
        for i in 0..NSAMPLES {
            let value = if result.c[i] < min { min } else { if result.c[i] > max { max } else { result.c[i] } };
            result.c[i] = value;
        }
        result
    }

    #[allow(unused)]
    pub fn max_component_value(&self) -> RaytracerFloat {
        let mut max = self.c[0];
        for i in 1..NSAMPLES {
            if self.c[i] > max {
                max = self.c[i];
            }
        }
        max
    }

    #[allow(unused)]
    pub fn has_nans(&self) -> bool {
        let mut max = self.c[0];
        for i in 1..NSAMPLES {
            if self.c[i].is_nan() {
                return true;
            }
        }
        false
    }
}

impl<const N: usize> AddAssign<CoefficientSpectrum<N>> for CoefficientSpectrum<N> {
    fn add_assign(&mut self, rhs: Self) {
        // TODO check has NaNs
        for i in 0..N {
            self.c[i] += rhs.c[i];
        }
    }
}

impl<const N: usize> Add<CoefficientSpectrum<N>> for CoefficientSpectrum<N> {
    type Output = CoefficientSpectrum<N>;
    fn add(self, rhs: CoefficientSpectrum<N>) -> Self::Output {
        let mut output = self.clone();
        output += rhs;
        output
    }
}

impl<const N: usize> SubAssign<CoefficientSpectrum<N>> for CoefficientSpectrum<N> {
    fn sub_assign(&mut self, rhs: CoefficientSpectrum<N>) {
        // TODO check has NaNs
        for i in 0..N {
            self.c[i] -= rhs.c[i];
        }
    }
}

impl<const N: usize> Sub<CoefficientSpectrum<N>> for CoefficientSpectrum<N> {
    type Output = CoefficientSpectrum<N>;
    fn sub(self, rhs: CoefficientSpectrum<N>) -> Self::Output {
        let mut output = self.clone();
        output -= rhs;
        output
    }
}

impl<const N: usize> Div<CoefficientSpectrum<N>> for CoefficientSpectrum<N> {
    type Output = CoefficientSpectrum<N>;
    fn div(self, rhs: CoefficientSpectrum<N>) -> Self::Output {
        let mut output = self.clone();
        for i in 0..N {
            output.c[i] /= rhs.c[i];
        }
        output
    }
}

impl<const N: usize> DivAssign<RaytracerFloat> for CoefficientSpectrum<N> {
    fn div_assign(&mut self, rhs: RaytracerFloat) {
        for i in 0..N {
            self.c[i] /= rhs;
        }
    }
}

impl<const N: usize> MulAssign<CoefficientSpectrum<N>> for CoefficientSpectrum<N> {
    fn mul_assign(&mut self, rhs: CoefficientSpectrum<N>) {
        // TODO check has NaNs
        for i in 0..N {
            self.c[i] *= rhs.c[i];
        }
    }
}

impl<const N: usize> Mul<CoefficientSpectrum<N>> for CoefficientSpectrum<N> {
    type Output = CoefficientSpectrum<N>;
    fn mul(self, rhs: CoefficientSpectrum<N>) -> Self::Output {
        let mut output = self.clone();
        output *= rhs;
        output
    }
}

impl<const N: usize> Mul<RaytracerFloat> for CoefficientSpectrum<N> {
    type Output = CoefficientSpectrum<N>;
    fn mul(self, rhs: RaytracerFloat) -> Self::Output {
        let mut output = self.clone();
        for i in 0..N {
            output.c[i] *= rhs;
        }
        output
    }
}

impl<const N: usize> Div<RaytracerFloat> for CoefficientSpectrum<N> {
    type Output = CoefficientSpectrum<N>;
    fn div(self, rhs: RaytracerFloat) -> Self::Output {
        let mut output = self.clone();
        for i in 0..N {
            output.c[i] /= rhs;
        }
        output
    }
}

impl<const N: usize> PartialEq<CoefficientSpectrum<N>> for CoefficientSpectrum<N> {
    fn eq(&self, other: &CoefficientSpectrum<N>) -> bool {
        for i in 0..N {
            if self.c[i] != other.c[i] { 
                return false;
            }
        }
        return true
    }

    fn ne(&self, other: &CoefficientSpectrum<N>) -> bool {
        !(self == other)
    }
}

impl<const N: usize> Neg for CoefficientSpectrum<N> {
    type Output = CoefficientSpectrum<N>;
    fn neg(self) -> Self::Output {
        let mut output = self.clone();
        for i in 0..N {
            output.c[i] = -output.c[i];
        }
        output
    }
}


// Printing

impl<const N: usize> fmt::Display for CoefficientSpectrum<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CoefficientSpectrum [")?;
        for i in 0..N {
            write!(f, "{}{}", self.c[i], if i<(N-1) { ", "} else { " " })?;
        }
        write!(f, "]")?;
        // write!(formatter, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
        Ok(())
    }
  }