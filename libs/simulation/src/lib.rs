#![feature(crate_visibility_modifier)]

use nalgebra as na;
use rand::{Rng, RngCore};
use std::f32::consts::FRAC_PI_2;

use lib_genetic_algorithm as ga;
use lib_neural_network as nn;

use self::creature_individual::*;
pub use self::{brain::*, creature::*, eye::*, food::*, world::*};

mod brain;
mod creature;
mod creature_individual;
mod eye;
mod food;
mod world;

const POPULATION_SIZE: usize = 40; // Number of Individuals in the population
const FOOD_SIZE: usize = 40; // Number of food in the world

const GENERATION_LENGTH: usize = 2500; // Steps before evolution

const SPEED_MIN: f32 = 0.001; // Minimum Creature speed
const SPEED_MAX: f32 = 0.005; // Maximum Creature speed
const SPEED_ACCEL: f32 = 0.2; // Change in speed per update
const ROTATION_ACCEL: f32 = FRAC_PI_2; // Change in rotation per update

const UPPER_BOUND_X: f32 = 1.0; // Upper bound for Creature position
const UPPER_BOUND_Y: f32 = 1.0; // Upper bound for Creature position
const LOWER_BOUND_X: f32 = 0.0; // Lower bound for Creature position
const LOWER_BOUND_Y: f32 = 0.0; // Lower bound for Creature position

pub struct Simulation {
    world: World,
    ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    age: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Simulation {
        let world = World::random(rng, POPULATION_SIZE, FOOD_SIZE);
        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection::new(),
            ga::UniformCrossover::new(),
            ga::GaussianMutation::new(0.01, 0.3),
        );

        Simulation { world, ga, age: 0 }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<ga::Statistics> {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movement();

        self.age += 1;

        if self.age > GENERATION_LENGTH {
            Some(self.evolve(rng))
        } else {
            None
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for creature in &mut self.world.creatures {
            for food in &mut self.world.foods {
                let distance = na::distance(&creature.position, &food.position);

                if distance <= 0.01 {
                    creature.satiation += 1;
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

            let update = creature.brain.nn.propagate(vision);
            let speed = update[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation = update[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

            creature.speed = (creature.speed + speed).clamp(SPEED_MIN, SPEED_MAX);
            creature.rotation = na::Rotation2::new(creature.rotation.angle() + rotation);
        }
    }

    fn process_movement(&mut self) {
        for creature in &mut self.world.creatures {
            creature.position += creature.rotation * na::Vector2::new(creature.speed, 0.0);

            creature.position.x = creature.position.x.clamp(LOWER_BOUND_X, UPPER_BOUND_X);
            creature.position.y = creature.position.y.clamp(LOWER_BOUND_Y, UPPER_BOUND_Y);
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) -> ga::Statistics {
        self.age = 0;

        // Transform Vec<Creature> into Vec<CreatureIndividual>
        let current_population: Vec<_> = self
            .world
            .creatures
            .iter()
            .map(CreatureIndividual::from_creature)
            .collect();

        // Evolve the population
        let (evolved_population, stats) = self.ga.step(rng, &current_population);

        // Transform Vec<CreatureIndividual> into Vec<Creature>
        self.world.creatures = evolved_population
            .into_iter()
            .map(|individual| individual.into_creature(rng))
            .collect();

        // Reset food positions
        for food in &mut self.world.foods {
            food.position = rng.gen();
        }

        stats
    }

    /// Step until end of current generation
    pub fn train(&mut self, rng: &mut dyn RngCore) -> ga::Statistics {
        loop {
            if let Some(summary) = self.step(rng) {
                return summary;
            }
        }
    }
}
