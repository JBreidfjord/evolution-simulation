#![feature(crate_visibility_modifier)]

use nalgebra as na;
use rand::{Rng, RngCore};

use lib_genetic_algorithm as ga;
use lib_neural_network as nn;

use self::creature_individual::*;
pub use self::{brain::*, config::*, creature::*, eye::*, food::*, world::*};

mod brain;
mod config;
mod creature;
mod creature_individual;
mod eye;
mod food;
mod world;

const UPPER_BOUND_X: f32 = 1.0; // Upper bound for Creature position
const UPPER_BOUND_Y: f32 = 1.0; // Upper bound for Creature position
const LOWER_BOUND_X: f32 = 0.0; // Lower bound for Creature position
const LOWER_BOUND_Y: f32 = 0.0; // Lower bound for Creature position

pub struct Simulation {
    world: World,
    ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    age: usize,
    config: Config,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore, config: Option<Config>) -> Simulation {
        let config = config.unwrap_or_default();
        let world = World::random(rng, &config);
        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection::new(),
            ga::UniformCrossover::new(),
            ga::GaussianMutation::new(0.01, 0.3),
        );

        Simulation {
            world,
            ga,
            age: 0,
            config,
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn age(&self) -> &usize {
        &self.age
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) -> bool {
        self.process_movement();
        self.process_collisions(rng);
        self.process_brains();

        self.age += 1;

        if self.age > self.config.generation_length {
            self.evolve(rng);
            true
        } else {
            false
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for creature in &mut self.world.creatures {
            if !creature.alive {
                continue;
            }

            for food in &mut self.world.foods {
                let distance = na::distance(&creature.position, &food.position);

                if distance <= (self.config.creature_size + self.config.food_size) / 2.0 {
                    creature.energy += self.config.food_energy;
                    creature.satiation += 1;
                    food.position = rng.gen();
                }
            }
        }
    }

    fn process_brains(&mut self) {
        for creature in &mut self.world.creatures {
            if !creature.alive {
                continue;
            }

            let vision = creature.eye.process_vision(
                creature.position,
                creature.rotation,
                &self.world.foods,
            );

            let update = creature.brain.nn.propagate(vision);
            let speed = update[0].clamp(-self.config.speed_accel, self.config.speed_accel);
            let rotation = update[1].clamp(-self.config.rotation_accel, self.config.rotation_accel);

            creature.speed =
                (creature.speed + speed).clamp(self.config.speed_min, self.config.speed_max);
            creature.rotation = na::Rotation2::new(creature.rotation.angle() + rotation);
        }
    }

    fn process_movement(&mut self) {
        for creature in &mut self.world.creatures {
            if !creature.alive {
                continue;
            }

            creature.position += creature.rotation * na::Vector2::new(creature.speed, 0.0);

            creature.position.x = creature.position.x.clamp(LOWER_BOUND_X, UPPER_BOUND_X);
            creature.position.y = creature.position.y.clamp(LOWER_BOUND_Y, UPPER_BOUND_Y);

            creature.energy -= self.config.energy_loss_factor * creature.speed;
            creature.energy = creature.energy.max(0.0);
            creature.alive = creature.energy > 0.0;
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) {
        self.age = 0;

        // Transform Vec<Creature> into Vec<CreatureIndividual>
        let current_population: Vec<_> = self
            .world
            .creatures
            .iter()
            .map(CreatureIndividual::from_creature)
            .collect();

        // Evolve the population
        let evolved_population = self.ga.step(rng, &current_population);

        // Transform Vec<CreatureIndividual> into Vec<Creature>
        self.world.creatures = evolved_population
            .into_iter()
            .map(|individual| individual.into_creature(rng, &self.config))
            .collect();

        // Reset food positions
        for food in &mut self.world.foods {
            food.position = rng.gen();
        }
    }

    /// Step until end of current generation
    pub fn train(&mut self, rng: &mut dyn RngCore) {
        loop {
            if self.step(rng) {
                return;
            }
        }
    }
}
