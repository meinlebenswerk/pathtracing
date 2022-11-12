use crate::{camera::Camera, sampler::RTXSampler};
use super::RTXIntegrator;


// There's no need for dynamic dispatch, here
// Since there will only ever be one type of sampler tied to a specific SamplerIntegrator
pub struct WhittedIntegrator<SamplerType>
where SamplerType: RTXSampler {
    camera: Camera,
    sampler: Arc<SamplerType>
}

impl<SamplerType> SamplerIntegrator<SamplerType>
where SamplerType: RTXSampler {
    #[allow(unused)]
    pub fn new(camera: Camera, sampler: Arc<SamplerType>) -> Self {
        Self {
            camera,
            sampler: Arc::clone(&sampler)
        }
    }
}

impl<SamplerType> RTXIntegrator for SamplerIntegrator<SamplerType>
where SamplerType: RTXSampler {
    fn render(&self, scene: &crate::scene::Scene) {
        // Preprocess
        // Render tiles
        // Save image when done
    }
}

