use std::f32::consts::FRAC_PI_2;

pub struct Config {
    pub population_count: usize,  // Number of individuals in the population
    pub food_count: usize,        // Number of food in the world
    pub creature_size: f32,       // Size of the creatures
    pub food_size: f32,           // Size of the food
    pub starting_energy: f32,     // Starting energy of the creatures
    pub food_energy: f32,         // Energy gained from each food
    pub energy_loss_factor: f32,  // Energy lost each tick * Creature speed
    pub generation_length: usize, // Steps before evolution
    pub speed_min: f32,           // Minimum Creature speed
    pub speed_max: f32,           // Maximum Creature speed
    pub speed_accel: f32,         // Change in speed per update
    pub rotation_accel: f32,      // Change in rotation per update
    pub mutation_rate: f32,       // Probability of mutation [0, 1]
    pub mutation_strength: f32,   // Multiplied factor of mutation
}

impl Config {
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
}

impl Default for Config {
    fn default() -> Config {
        Config {
            population_count: 20,
            food_count: 40,
            creature_size: 0.01,
            food_size: 0.01,
            starting_energy: 100.0,
            food_energy: 25.0,
            energy_loss_factor: 50.0,
            generation_length: 2500,
            speed_min: 0.001,
            speed_max: 0.005,
            speed_accel: 0.2,
            rotation_accel: FRAC_PI_2,
            mutation_rate: 0.01,
            mutation_strength: 0.3,
        }
    }
}
