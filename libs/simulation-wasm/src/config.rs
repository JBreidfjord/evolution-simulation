use crate::*;

#[derive(Debug, Clone, Copy, Serialize)]
#[wasm_bindgen]
pub struct Config {
    pub population_count: usize,
    pub food_count: usize,
    pub creature_size: f32,
    pub food_size: f32,
    pub starting_energy: f32,
    pub food_energy: f32,
    pub energy_loss_factor: f32,
    pub generation_length: usize,
    pub speed_min: f32,
    pub speed_max: f32,
    pub speed_accel: f32,
    pub rotation_accel: f32,
    pub mutation_rate: f32,
    pub mutation_strength: f32,
}

#[wasm_bindgen]
impl Config {
    #[wasm_bindgen(constructor)]
    pub fn new(
        population_count: usize,
        food_count: usize,
        creature_size: f32,
        food_size: f32,
        starting_energy: f32,
        food_energy: f32,
        energy_loss_factor: f32,
        generation_length: usize,
        speed_min: f32,
        speed_max: f32,
        speed_accel: f32,
        rotation_accel: f32,
        mutation_rate: f32,
        mutation_strength: f32,
    ) -> Config {
        Config {
            population_count,
            food_count,
            creature_size,
            food_size,
            starting_energy,
            food_energy,
            energy_loss_factor,
            generation_length,
            speed_min,
            speed_max,
            speed_accel,
            rotation_accel,
            mutation_rate,
            mutation_strength,
        }
    }

    pub fn default() -> Config {
        Config::from(sim::Config::default())
    }
}

impl From<sim::Config> for Config {
    fn from(config: sim::Config) -> Config {
        Config {
            population_count: config.population_count,
            food_count: config.food_count,
            creature_size: config.creature_size,
            food_size: config.food_size,
            starting_energy: config.starting_energy,
            food_energy: config.food_energy,
            energy_loss_factor: config.energy_loss_factor,
            generation_length: config.generation_length,
            speed_min: config.speed_min,
            speed_max: config.speed_max,
            speed_accel: config.speed_accel,
            rotation_accel: config.rotation_accel,
            mutation_rate: config.mutation_rate,
            mutation_strength: config.mutation_strength,
        }
    }
}

impl Into<sim::Config> for Config {
    fn into(self) -> sim::Config {
        sim::Config {
            population_count: self.population_count,
            food_count: self.food_count,
            creature_size: self.creature_size,
            food_size: self.food_size,
            starting_energy: self.starting_energy,
            food_energy: self.food_energy,
            energy_loss_factor: self.energy_loss_factor,
            generation_length: self.generation_length,
            speed_min: self.speed_min,
            speed_max: self.speed_max,
            speed_accel: self.speed_accel,
            rotation_accel: self.rotation_accel,
            mutation_rate: self.mutation_rate,
            mutation_strength: self.mutation_strength,
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config::from(sim::Config::default())
    }
}
