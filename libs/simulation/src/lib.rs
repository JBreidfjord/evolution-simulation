#![feature(crate_visibility_modifier)]

use nalgebra as na;
use rand::{Rng, RngCore};
use std::f32::consts::FRAC_PI_2;

use lib_neural_network as nn;

pub use self::{brain::*, creature::*, eye::*, food::*, world::*};

mod brain;
mod creature;
mod eye;
mod food;
mod world;

const SPEED_MIN: f32 = 0.001; // Minimum Creature speed
const SPEED_MAX: f32 = 0.005; // Maximum Creature speed
const SPEED_ACCEL: f32 = 0.2; // Change in speed per update
const ROTATION_ACCEL: f32 = FRAC_PI_2; // Change in rotation per update

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
        self.process_brains();
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

    fn process_brains(&mut self) {
        for creature in &mut self.world.creatures {
            let vision = creature.eye.process_vision(
                creature.position,
                creature.rotation,
                &self.world.foods,
            );

            let update = creature.brain.propagate(vision);
            let speed = update[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation = update[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

            creature.speed = (creature.speed + speed).clamp(SPEED_MIN, SPEED_MAX);
            creature.rotation = na::Rotation2::new(creature.rotation.angle() + rotation);
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
