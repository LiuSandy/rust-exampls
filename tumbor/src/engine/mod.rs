use crate::pb::Spec;

mod photon;
use image::ImageFormat;
pub use photon::Photon;

pub trait Engine {
    fn apply(&mut self, spec: &[Spec]);
    fn generate(self, format: ImageFormat) -> Vec<u8>;
}

pub trait SpecTransform<T> {
    fn transform(&mut self, op: T);
}
