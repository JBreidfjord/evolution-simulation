use crate::*;

#[derive(Debug)]
pub struct World {
    crate creatures: Vec<Creature>,
    crate foods: Vec<Food>,
}

impl World {
    pub fn random(rng: &mut dyn RngCore, config: &Config) -> World {
        let creatures = (0..config.population_count)
            .map(|_| Creature::random(rng, &config))
            .collect();

        let foods = (0..config.food_count).map(|_| Food::random(rng)).collect();

        World { creatures, foods }
    }

    pub fn creatures(&self) -> &Vec<Creature> {
        &self.creatures
    }

    pub fn foods(&self) -> &Vec<Food> {
        &self.foods
    }
}
