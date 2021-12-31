use rand::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use lib_simulation as sim;

use crate::config::*;

mod config;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
    pub generation: usize,
    pub config: Config,
}

#[derive(Debug, Clone, Serialize)]
pub struct World {
    pub creatures: Vec<Creature>,
    pub foods: Vec<Food>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Creature {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub fitness: f32,
    pub energy: f32,
    pub size: f32,
    pub generation: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct Food {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(config: Option<Config>) -> Simulation {
        let mut rng = thread_rng();
        let config = config.unwrap_or_default();
        let sim = sim::Simulation::random(&mut rng, Some(config.into()));

        Simulation {
            rng,
            sim,
            generation: 0,
            config,
        }
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());
        JsValue::from_serde(&world).unwrap()
    }

    pub fn age(&self) -> usize {
        *self.sim.age()
    }

    pub fn step(&mut self) {
        self.sim.step(&mut self.rng)
    }

    pub fn train(&mut self) {
        self.sim.train();
        self.generation += 1;
    }
}

impl From<&sim::World> for World {
    fn from(world: &sim::World) -> World {
        let creatures: Vec<Creature> = world.creatures().iter().map(Creature::from).collect();
        let creatures = creatures
            .into_iter()
            .enumerate()
            .map(|(i, creature)| Creature {
                id: i,
                x: creature.x,
                y: creature.y,
                rotation: creature.rotation,
                fitness: creature.fitness,
                energy: creature.energy,
                size: creature.size,
                generation: creature.generation,
            })
            .collect();
        let foods = world.foods().iter().map(Food::from).collect();

        World { creatures, foods }
    }
}

impl From<&sim::Creature> for Creature {
    fn from(creature: &sim::Creature) -> Creature {
        Creature {
            id: 0,
            x: creature.position().x,
            y: creature.position().y,
            rotation: creature.rotation().angle(),
            fitness: creature.fitness(),
            energy: creature.energy(),
            size: creature.size(),
            generation: creature.generation(),
        }
    }
}

impl From<&sim::Food> for Food {
    fn from(food: &sim::Food) -> Food {
        Food {
            x: food.position().x,
            y: food.position().y,
        }
    }
}
