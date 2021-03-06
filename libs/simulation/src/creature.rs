use crate::*;

#[derive(Debug, Clone)]
pub struct Creature {
    pub(crate) position: na::Point2<f32>,
    pub(crate) eye: Eye,
    pub(crate) brain: Brain,
    pub(crate) body: Body,
    pub(crate) satiation: usize,
    pub(crate) generation: usize,
}

impl Creature {
    pub fn random(rng: &mut dyn RngCore, config: &Config) -> Creature {
        let eye = Eye::new(config.fov_range, config.fov_angle, config.eye_cells);
        let brain = Brain::random(rng, &eye);
        let body = Body::random(rng, config);

        Creature::new(eye, brain, body, rng)
    }

    fn new(eye: Eye, brain: Brain, body: Body, rng: &mut dyn RngCore) -> Creature {
        Creature {
            position: rng.gen(),
            eye,
            brain,
            body,
            satiation: 0,
            generation: 0,
        }
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        self.brain
            .as_chromosome()
            .into_iter()
            .chain(self.body.as_chromosome().into_iter())
            .collect()
    }

    pub(crate) fn from_chromosome(
        chromosome: ga::Chromosome,
        rng: &mut dyn RngCore,
        config: &Config,
    ) -> Creature {
        let brain_chromosome_length = (config.eye_cells * 2 * config.eye_cells) // Input Layer
            + (2 * config.eye_cells * 2) // Hidden Layer
            + (config.eye_cells * 2 + 2); // Biases
        let [brain_chromosome, body_chromosome] = chromosome.split_at(brain_chromosome_length);

        let eye = Eye::new(config.fov_range, config.fov_angle, config.eye_cells);
        let brain = Brain::from_chromosome(brain_chromosome, &eye);
        let body = Body::from_chromosome(body_chromosome, rng, config);

        Creature::new(eye, brain, body, rng)
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.body.rotation
    }

    pub fn generation(&self) -> usize {
        self.generation
    }

    pub fn fitness(&self) -> f32 {
        if self.body.energy <= 0.0 {
            0.0
        } else {
            self.satiation as f32
        }
    }

    pub fn energy(&self) -> f32 {
        self.body.energy
    }

    pub fn size(&self) -> f32 {
        self.body.size
    }

    pub fn color(&self) -> f32 {
        self.body.color
    }
}
