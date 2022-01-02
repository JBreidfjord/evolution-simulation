use crate::*;

#[derive(Debug)]
pub struct World {
    crate creatures: Vec<Creature>,
    crate foods: Vec<Food>,
    crate food_count: usize,
}

impl World {
    pub fn random(rng: &mut dyn RngCore, config: &Config) -> World {
        let creatures: Vec<Creature> = (0..config.population_count)
            .map(|_| Creature::random(rng, &config))
            .collect();

        let k = ((config.target_population as isize - creatures.len() as isize) as f32
            / (config.target_population as f32 * 0.5))
            .exp();
        let food_count =
            k * config.food_count as f32 * (config.target_population / creatures.len()) as f32;
        let foods = (0..food_count as usize)
            .map(|_| Food::random(rng))
            .collect();

        World {
            creatures,
            foods,
            food_count: food_count as usize,
        }
    }

    pub fn creatures(&self) -> &Vec<Creature> {
        &self.creatures
    }

    pub fn foods(&self) -> &Vec<Food> {
        &self.foods
    }

    pub fn update_food(&mut self, config: &Config, rng: &mut dyn RngCore) {
        let k = ((config.target_population as isize - self.creatures.len() as isize) as f32
            / (config.target_population as f32 * 0.5))
            .exp();
        self.food_count =
            (k * config.food_count as f32
                * (config.target_population / self.creatures.len()) as f32) as usize;
        if self.foods.len() < self.food_count {
            self.foods
                .extend((0..self.food_count - self.foods.len()).map(|_| Food::random(rng)));
        } else if self.foods.len() > self.food_count {
            self.foods.truncate(self.food_count);
        }
    }
}
