#![feature(crate_visibility_modifier)]

use nalgebra as na;
use rand::{Rng, RngCore};

pub use self::{creature::*, food::*, world::*};

mod creature;
mod food;
mod world;

pub struct Simulation {
    world: World,
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

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_movement();
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for creature in &mut self.world.creatures {
            for food in &mut self.world.foods {
                let distance = na::distance(&creature.position, &food.position);

                if distance <= 0.01 {
                    food.position = rng.gen();
                }
            }
        }
    }

    fn process_movement(&mut self) {
        for creature in &mut self.world.creatures {
            creature.position += creature.rotation * na::Vector2::new(creature.speed, 0.0);

            creature.position.x = na::wrap(creature.position.x, 0.0, 1.0);
            creature.position.y = na::wrap(creature.position.y, 0.0, 1.0);
        }
    }
}
