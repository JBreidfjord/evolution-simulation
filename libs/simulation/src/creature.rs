use crate::*;

#[derive(Debug, Clone)]
pub struct Creature {
    crate position: na::Point2<f32>,
    // Could replace rotation and speed with velocity Vector2
    crate rotation: na::Rotation2<f32>,
    crate speed: f32,
    crate eye: Eye,
    crate brain: Brain,
    crate satiation: usize,
    crate energy: f32,
    crate alive: bool,
}

impl Creature {
    pub fn random(rng: &mut dyn RngCore, config: &Config) -> Creature {
        let eye = Eye::default();
        let brain = Brain::random(rng, &eye);

        Creature::new(eye, brain, rng, &config)
    }

    fn new(eye: Eye, brain: Brain, rng: &mut dyn RngCore, config: &Config) -> Creature {
        Creature {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: config.speed_min,
            eye,
            brain,
            satiation: 0,
            energy: config.starting_energy,
            alive: true,
        }
    }

    crate fn as_chromosome(&self) -> ga::Chromosome {
        // Could add more encoding here for things like size or colour
        self.brain.as_chromosome()
    }

    crate fn from_chromosome(
        chromosome: ga::Chromosome,
        rng: &mut dyn RngCore,
        config: &Config,
    ) -> Creature {
        let eye = Eye::default();
        let brain = Brain::from_chromosome(chromosome, &eye);

        Creature::new(eye, brain, rng, &config)
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }

    pub fn energy(&self) -> f32 {
        self.energy
    }

    pub fn alive(&self) -> bool {
        self.alive
    }

    pub fn fitness(&self) -> f32 {
        if !self.alive {
            0.0
        } else {
            self.satiation as f32
        }
    }
}
