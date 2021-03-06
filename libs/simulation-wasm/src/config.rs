use crate::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen(inspectable)]
pub struct Config {
    pub population_count: usize,
    pub target_population: usize,
    pub food_count: usize,
    pub creature_size: f32,
    pub food_size: f32,
    pub starting_energy: f32,
    pub food_energy: f32,
    pub energy_loss_factor: f32,
    pub reproduction_cost: f32,
    pub reproduction_threshold: f32,
    pub speed_min: f32,
    pub speed_max: f32,
    pub speed_accel: f32,
    pub rotation_accel: f32,
    pub mutation_rate: f32,
    pub mutation_strength: f32,
    pub fov_range: f32,
    pub fov_angle: f32,
    pub eye_cells: usize,
}

#[wasm_bindgen]
impl Config {
    #[wasm_bindgen(constructor)]
    pub fn from_object(obj: &JsValue) -> Config {
        obj.into_serde().unwrap_or_default()
    }
}

impl From<sim::Config> for Config {
    fn from(config: sim::Config) -> Config {
        Config {
            population_count: config.population_count,
            target_population: config.target_population,
            food_count: config.food_count,
            creature_size: config.creature_size,
            food_size: config.food_size,
            starting_energy: config.starting_energy,
            food_energy: config.food_energy,
            energy_loss_factor: config.energy_loss_factor,
            reproduction_cost: config.reproduction_cost,
            reproduction_threshold: config.reproduction_threshold,
            speed_min: config.speed_min,
            speed_max: config.speed_max,
            speed_accel: config.speed_accel,
            rotation_accel: config.rotation_accel,
            mutation_rate: config.mutation_rate,
            mutation_strength: config.mutation_strength,
            fov_range: config.fov_range,
            fov_angle: config.fov_angle,
            eye_cells: config.eye_cells,
        }
    }
}

impl Into<sim::Config> for Config {
    fn into(self) -> sim::Config {
        sim::Config {
            population_count: self.population_count,
            target_population: self.target_population,
            food_count: self.food_count,
            creature_size: self.creature_size,
            food_size: self.food_size,
            starting_energy: self.starting_energy,
            food_energy: self.food_energy,
            energy_loss_factor: self.energy_loss_factor,
            reproduction_cost: self.reproduction_cost,
            reproduction_threshold: self.reproduction_threshold,
            speed_min: self.speed_min,
            speed_max: self.speed_max,
            speed_accel: self.speed_accel,
            rotation_accel: self.rotation_accel,
            mutation_rate: self.mutation_rate,
            mutation_strength: self.mutation_strength,
            fov_range: self.fov_range,
            fov_angle: self.fov_angle,
            eye_cells: self.eye_cells,
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config::from(sim::Config::default())
    }
}
