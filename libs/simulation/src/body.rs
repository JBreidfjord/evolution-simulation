use crate::*;

#[derive(Debug, Clone)]
pub struct Body {
    crate size: f32,
    crate color: f32,
    // Could replace rotation and speed with velocity Vector2
    crate rotation: na::Rotation2<f32>,
    crate speed: f32,
    crate energy: f32,
}

impl Body {
    pub fn new(rng: &mut dyn RngCore, size: f32, color: f32, config: &Config) -> Body {
        assert!(size > 0.0);
        assert!(color > 0.0);

        Body {
            size,
            color,
            rotation: rng.gen(),
            speed: config.speed_min,
            energy: config.starting_energy,
        }
    }

    crate fn random(rng: &mut dyn RngCore, config: &Config) -> Body {
        let size = rng.gen_range((config.creature_size / 2.0)..=(config.creature_size * 1.5));
        let color = rng.gen();

        Body::new(rng, size, color, config)
    }

    crate fn as_chromosome(&self) -> ga::Chromosome {
        ga::Chromosome::from_iter(vec![self.size, self.color])
    }

    crate fn from_chromosome(
        chromosome: ga::Chromosome,
        rng: &mut dyn RngCore,
        config: &Config,
    ) -> Body {
        let size = chromosome[0];
        let color = chromosome[1];

        Body::new(rng, size, color, config)
    }

    crate fn process_energy(&mut self, rotation: f32, config: &Config) {
        self.energy -=
            config.energy_loss_factor * (rotation.abs() + self.speed + (5.0 * self.size));
    }

    pub fn size(&self) -> f32 {
        self.size
    }

    pub fn color(&self) -> f32 {
        self.color
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }

    pub fn energy(&self) -> f32 {
        self.energy
    }
}
