use nalgebra as na;
use rand::Rng;
use rand::RngCore;

pub struct Simulation {
    world: World,
}

#[derive(Debug)]
pub struct World {
    creatures: Vec<Creature>,
    foods: Vec<Food>,
}

#[derive(Debug)]
pub struct Creature {
    position: na::Point2<f32>,
    // Could replace rotation and speed with velocity Vector2
    rotation: na::Rotation2<f32>,
    speed: f32,
}

#[derive(Debug)]
pub struct Food {
    position: na::Point2<f32>,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Simulation {
        Simulation {
            world: World::random(rng),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> World {
        let creatures = (0..40).map(|_| Creature::random(rng)).collect();

        let foods = (0..40).map(|_| Food::random(rng)).collect();

        World { creatures, foods }
    }

    pub fn creatures(&self) -> &Vec<Creature> {
        &self.creatures
    }

    pub fn foods(&self) -> &Vec<Food> {
        &self.foods
    }
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
