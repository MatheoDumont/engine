pub mod line;
pub mod obb;
pub mod plane;
pub mod sphere;

use crate::math::{math_essentials::*, Mat3};
use downcast_rs::DowncastSync;

pub use line::Line;
pub use obb::OBB;
pub use plane::Plane;
pub use sphere::Sphere;

#[derive(Copy, Clone)]
pub enum ShapeType {
    Sphere = 0,
    OBB = 1,
    Plane = 2,
    Line = 3,
}

pub trait Shape: DowncastSync {
    fn inertia_matrix(&self, mass: Real) -> Mat3;
    fn shape_type(&self) -> ShapeType;
    // fn transform_ref(&self) -> &Transform;
    // fn set_transform(&self, t: Transform);
}

impl_downcast!(sync Shape);

#[cfg(test)]
mod tests {
    use super::*;

    fn presque_bidon() {
        assert!(ShapeType::Sphere as usize == 0usize);
        assert!(ShapeType::OBB as usize == 1usize);
        assert!(ShapeType::Plane as usize == 2usize);
    }
}
