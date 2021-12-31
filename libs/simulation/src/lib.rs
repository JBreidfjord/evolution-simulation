#![feature(crate_visibility_modifier)]

use std::cmp::Ordering;

use nalgebra as na;
use rand::{Rng, RngCore};

use lib_genetic_algorithm as ga;
use lib_neural_network as nn;

use self::creature_individual::*;
pub use self::{body::*, brain::*, config::*, creature::*, eye::*, food::*, world::*};

mod body;
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
            ga::GaussianMutation::new(config.mutation_rate, config.mutation_strength),
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

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_brains();
        self.process_movement();
        self.process_collisions(rng);
        self.process_death();
        self.process_evolution(rng);
        self.process_death();

        self.age += 1;
    }

    fn process_brains(&mut self) {
        for creature in &mut self.world.creatures {
            let vision = creature.eye.process_vision(
                creature.position,
                creature.body.rotation,
                &self.world.foods,
            );

            let update = creature.brain.nn.propagate(vision);
            let speed = update[0].clamp(-self.config.speed_accel, self.config.speed_accel);
            let rotation = update[1].clamp(-self.config.rotation_accel, self.config.rotation_accel);

            creature.body.speed =
                (creature.body.speed + speed).clamp(self.config.speed_min, self.config.speed_max);
            creature.body.rotation = na::Rotation2::new(creature.body.rotation.angle() + rotation);

            creature.body.process_energy(rotation, &self.config);
        }
    }

    fn process_movement(&mut self) {
        for creature in &mut self.world.creatures {
            creature.position +=
                creature.body.rotation * na::Vector2::new(creature.body.speed, 0.0);

            creature.position.x = creature.position.x.clamp(LOWER_BOUND_X, UPPER_BOUND_X);
            creature.position.y = creature.position.y.clamp(LOWER_BOUND_Y, UPPER_BOUND_Y);
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for creature in &mut self.world.creatures {
            for food in &mut self.world.foods {
                let distance = na::distance(&creature.position, &food.position);

                if distance <= (self.config.creature_size + self.config.food_size) / 2.0 {
                    creature.body.energy += self.config.food_energy;
                    creature.satiation += 1;
                    food.position = rng.gen();
                }
            }
        }
    }

    fn process_evolution(&mut self, rng: &mut dyn RngCore) {
        let creatures = self.world.creatures.clone();
        let mut creatures_with_idx: Vec<(usize, Creature)> =
            creatures.into_iter().enumerate().collect();
        creatures_with_idx
            .retain(|(_, creature)| creature.body.energy >= self.config.reproduction_threshold);

        let mut reproduced_indices = Vec::new();
        let mut new_creatures = Vec::new();
        for (idx, creature) in self.world.creatures.iter().enumerate() {
            if creature.body.energy >= self.config.reproduction_threshold {
                // Prevent duplicated reproduction
                if reproduced_indices.contains(&idx) {
                    continue;
                }

                // Find nearest creature with enough energy
                let (nearest_creature_idx, nearest_creature) = creatures_with_idx
                    .iter()
                    .min_by(|(idx_a, a), (idx_b, b)| {
                        // Prevents self-comparing
                        if &idx == idx_a || reproduced_indices.contains(idx_a) {
                            Ordering::Greater
                        } else if &idx == idx_b || reproduced_indices.contains(idx_b) {
                            Ordering::Less
                        } else {
                            na::distance(&a.position, &creature.position)
                                .partial_cmp(&na::distance(&b.position, &creature.position))
                                .unwrap_or(Ordering::Equal)
                        }
                    })
                    .unwrap();

                // Prevent self-reproduction and duplicated reproduction
                if &idx == nearest_creature_idx || reproduced_indices.contains(nearest_creature_idx)
                {
                    continue;
                }

                let mut new_creature = self
                    .ga
                    .breed(
                        rng,
                        CreatureIndividual::from_creature(creature),
                        CreatureIndividual::from_creature(nearest_creature),
                    )
                    .into_creature(rng, &self.config);
                new_creature.body.energy = self.config.reproduction_cost * 2.0; // Energy from parents
                new_creature.position = na::center(&creature.position, &nearest_creature.position);
                new_creature.generation = creature.generation.max(nearest_creature.generation) + 1;
                new_creatures.push(new_creature);

                reproduced_indices.push(idx);
                reproduced_indices.push(*nearest_creature_idx);
            }
        }

        // Remove energy for reproduction
        for idx in reproduced_indices {
            self.world.creatures[idx].body.energy -= self.config.reproduction_cost;
        }
        self.world.creatures.extend(new_creatures);
    }

    fn process_death(&mut self) {
        self.world
            .creatures
            .retain(|creature| creature.body.energy > 0.0);
    }

    /// Step until end of current generation
    pub fn train(&mut self) {
        todo!()
    }
}
