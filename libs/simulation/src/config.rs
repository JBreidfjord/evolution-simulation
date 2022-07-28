use std::f32::consts::{FRAC_PI_4, FRAC_PI_8, PI};

pub struct Config {
    pub population_count: usize, // Number of individuals in the population
    pub target_population: usize,
    pub food_count: usize,           // Number of food in the world
    pub creature_size: f32,          // Size of the creatures
    pub food_size: f32,              // Size of the food
    pub starting_energy: f32,        // Starting energy of the creatures
    pub food_energy: f32,            // Energy gained from each food
    pub energy_loss_factor: f32,     // Energy lost each tick * Creature speed
    pub reproduction_cost: f32,      // Energy cost to reproduce
    pub reproduction_threshold: f32, // Threshold for allowing reproduction
    pub speed_min: f32,              // Minimum Creature speed
    pub speed_max: f32,              // Maximum Creature speed
    pub speed_accel: f32,            // Change in speed per update
    pub rotation_accel: f32,         // Change in rotation per update
    pub mutation_rate: f32,          // Probability of mutation [0, 1]
    pub mutation_strength: f32,      // Multiplied factor of mutation
    pub fov_range: f32,
    pub fov_angle: f32,
    pub eye_cells: usize,
}

impl Config {
    pub fn new(
        population_count: usize,
        target_population: usize,
        food_count: usize,
        creature_size: f32,
        food_size: f32,
        starting_energy: f32,
        food_energy: f32,
        energy_loss_factor: f32,
        reproduction_cost: f32,
        reproduction_threshold: f32,
        speed_min: f32,
        speed_max: f32,
        speed_accel: f32,
        rotation_accel: f32,
        mutation_rate: f32,
        mutation_strength: f32,
        fov_range: f32,
        fov_angle: f32,
        eye_cells: usize,
    ) -> Config {
        Config {
            population_count,
            target_population,
            food_count,
            creature_size,
            food_size,
            starting_energy,
            food_energy,
            energy_loss_factor,
            reproduction_cost,
            reproduction_threshold,
            speed_min,
            speed_max,
            speed_accel,
            rotation_accel,
            mutation_rate,
            mutation_strength,
            fov_range,
            fov_angle,
            eye_cells,
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            population_count: 200,
            target_population: 100,
            food_count: 15,
            creature_size: 0.005,
            food_size: 0.005,
            starting_energy: 100.0,
            food_energy: 75.0,
            energy_loss_factor: 5.0,
            reproduction_cost: 50.0,
            reproduction_threshold: 100.0,
            speed_min: 0.0001,
            speed_max: 0.0025,
            speed_accel: 0.2,
            rotation_accel: FRAC_PI_8 / 2.0,
            mutation_rate: 0.15,
            mutation_strength: 0.3,
            fov_range: 0.25,
            fov_angle: PI + FRAC_PI_4,
            eye_cells: 9,
        }
    }
}
