use crate::*;

#[derive(Debug)]
pub struct World {
    crate creatures: Vec<Creature>,
    crate foods: Vec<Food>,
}

impl World {
    pub fn random(rng: &mut dyn RngCore, population_size: usize, food_size: usize) -> World {
        let creatures = (0..population_size)
            .map(|_| Creature::random(rng))
            .collect();

        let foods = (0..food_size).map(|_| Food::random(rng)).collect();

        World { creatures, foods }
    }

    pub fn creatures(&self) -> &Vec<Creature> {
        &self.creatures
    }

    pub fn foods(&self) -> &Vec<Food> {
        &self.foods
    }
}
