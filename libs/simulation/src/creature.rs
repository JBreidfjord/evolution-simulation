use crate::*;

#[derive(Debug)]
pub struct Creature {
    crate position: na::Point2<f32>,
    // Could replace rotation and speed with velocity Vector2
    crate rotation: na::Rotation2<f32>,
    crate speed: f32,
    crate eye: Eye,
    crate brain: Brain,
    crate satiation: usize,
}

impl Creature {
    pub fn random(rng: &mut dyn RngCore) -> Creature {
        let eye = Eye::default();
        let brain = Brain::random(rng, &eye);

        Creature::new(eye, brain, rng)
    }

    fn new(eye: Eye, brain: Brain, rng: &mut dyn RngCore) -> Creature {
        Creature {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: SPEED_MIN,
            eye,
            brain,
            satiation: 0,
        }
    }

    crate fn as_chromosome(&self) -> ga::Chromosome {
        // Could add more encoding here for things like size or colour
        self.brain.as_chromosome()
    }

    crate fn from_chromosome(chromosome: ga::Chromosome, rng: &mut dyn RngCore) -> Creature {
        let eye = Eye::default();
        let brain = Brain::from_chromosome(chromosome, &eye);

        Creature::new(eye, brain, rng)
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }

    pub fn fitness(&self) -> f32 {
        self.satiation as f32
    }
}
