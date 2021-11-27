use crate::*;

#[derive(Debug)]
pub struct Food {
    crate position: na::Point2<f32>,
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Food {
        Food {
            position: rng.gen(),
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
}
