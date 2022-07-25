use crate::*;

#[derive(Debug, Clone)]
pub struct Body {
    pub(crate) size: f32,
    pub(crate) color: f32,
    // Could replace rotation and speed with velocity Vector2
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) energy: f32,
}

impl Body {
    pub fn new(rng: &mut dyn RngCore, size: f32, color: f32, config: &Config) -> Body {
        assert!(size > 0.0);
        assert!(color >= 0.0);

        Body {
            size,
            color,
            rotation: rng.gen(),
            speed: config.speed_min,
            energy: config.starting_energy,
        }
    }

    pub(crate) fn random(rng: &mut dyn RngCore, config: &Config) -> Body {
        let size = rng.gen_range((config.creature_size / 2.0)..=(config.creature_size * 1.5));
        let color = rng.gen();

        Body::new(rng, size, color, config)
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        ga::Chromosome::from_iter(vec![self.size * 10.0, self.color * 5.0])
    }

    pub(crate) fn from_chromosome(
        chromosome: ga::Chromosome,
        rng: &mut dyn RngCore,
        config: &Config,
    ) -> Body {
        let size =
            (chromosome[0] / 10.0).clamp(config.creature_size / 5.0, config.creature_size * 5.0);
        let color = (chromosome[1] / 5.0).clamp(0.0, 1.0);

        Body::new(rng, size, color, config)
    }

    pub(crate) fn process_energy(&mut self, rotation: f32, config: &Config) {
        self.energy -= config.energy_loss_factor
            * (rotation.abs() + self.speed + (10.0 * self.size)).powf(2.0);
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
