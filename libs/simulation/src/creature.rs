use crate::*;

#[derive(Debug, Clone)]
pub struct Creature {
    crate position: na::Point2<f32>,
    crate eye: Eye,
    crate brain: Brain,
    crate body: Body,
    crate satiation: usize,
    crate generation: usize,
}

impl Creature {
    pub fn random(rng: &mut dyn RngCore, config: &Config) -> Creature {
        let eye = Eye::new(config.fov_range, config.fov_angle, config.eye_cells);
        let brain = Brain::random(rng, &eye);
        let body = Body::random(rng, config);

        Creature::new(eye, brain, body, rng, &config)
    }

    fn new(eye: Eye, brain: Brain, body: Body, rng: &mut dyn RngCore, config: &Config) -> Creature {
        Creature {
            position: rng.gen(),
            eye,
            brain,
            body,
            satiation: 0,
            generation: 0,
        }
    }

    crate fn as_chromosome(&self) -> ga::Chromosome {
        self.brain
            .as_chromosome()
            .into_iter()
            .chain(self.body.as_chromosome().into_iter())
            .collect()
    }

    crate fn from_chromosome(
        chromosome: ga::Chromosome,
        rng: &mut dyn RngCore,
        config: &Config,
    ) -> Creature {
        let brain_chromosome_length = config.eye_cells * (2 * config.eye_cells + 4);
        let [brain_chromosome, body_chromosome] = chromosome.split_at(brain_chromosome_length);

        let eye = Eye::new(config.fov_range, config.fov_angle, config.eye_cells);
        let brain = Brain::from_chromosome(brain_chromosome, &eye);
        let body = Body::from_chromosome(body_chromosome, rng, config);

        Creature::new(eye, brain, body, rng, &config)
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
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
}
