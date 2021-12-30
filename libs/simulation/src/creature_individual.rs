use crate::*;

pub struct CreatureIndividual {
    fitness: f32,
    chromosome: ga::Chromosome,
}

impl ga::Individual for CreatureIndividual {
    fn create(chromosome: ga::Chromosome) -> CreatureIndividual {
        CreatureIndividual {
            fitness: 0.0,
            chromosome,
        }
    }

    fn chromosome(&self) -> &ga::Chromosome {
        &self.chromosome
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}

impl CreatureIndividual {
    pub fn from_creature(creature: &Creature) -> CreatureIndividual {
        CreatureIndividual {
            fitness: creature.fitness(),
            chromosome: creature.as_chromosome(),
        }
    }

    pub fn into_creature(self, rng: &mut dyn RngCore, config: &Config) -> Creature {
        Creature::from_chromosome(self.chromosome, rng, &config)
    }
}
