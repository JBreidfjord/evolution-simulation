use rand::prelude::*;
use serde::Serialize;
use wasm_bindgen::prelude::*;

use lib_simulation as sim;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[derive(Debug, Clone, Serialize)]
pub struct World {
    pub creatures: Vec<Creature>,
    pub foods: Vec<Food>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Creature {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct Food {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Simulation {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);

        Simulation { rng, sim }
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());
        JsValue::from_serde(&world).unwrap()
    }

    pub fn step(&mut self) {
        self.sim.step(&mut self.rng);
    }

    pub fn train(&mut self) -> String {
        let stats = self.sim.train(&mut self.rng);

        format!(
            "min={:.2}, max={:.2}, avg={:.2}",
            stats.min_fitness(),
            stats.max_fitness(),
            stats.avg_fitness()
        )
    }
}

impl From<&sim::World> for World {
    fn from(world: &sim::World) -> World {
        let creatures = world.creatures().iter().map(Creature::from).collect();
        let foods = world.foods().iter().map(Food::from).collect();

        World { creatures, foods }
    }
}

impl From<&sim::Creature> for Creature {
    fn from(creature: &sim::Creature) -> Creature {
        Creature {
            x: creature.position().x,
            y: creature.position().y,
            rotation: creature.rotation().angle(),
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
