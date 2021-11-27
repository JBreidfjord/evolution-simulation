use crate::*;

#[derive(Debug)]
pub struct Creature {
    crate position: na::Point2<f32>,
    // Could replace rotation and speed with velocity Vector2
    crate rotation: na::Rotation2<f32>,
    crate speed: f32,
}

impl Creature {
    pub fn random(rng: &mut dyn RngCore) -> Creature {
        Creature {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}
