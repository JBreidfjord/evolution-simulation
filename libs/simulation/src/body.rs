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

    pub fn random(rng: &mut dyn RngCore, config: &Config) -> Body {
        let size = rng.gen_range((config.creature_size / 2.0)..=(config.creature_size * 1.5));
        let color = rng.gen();

        Body::new(rng, size, color, config)
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
