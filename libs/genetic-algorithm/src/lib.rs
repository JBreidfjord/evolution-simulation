use rand::seq::SliceRandom;

pub struct GeneticAlgorithm;

pub struct RouletteWheelSelection;

pub trait Individual {
    fn fitness(&self) -> f32;
}

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn rand::RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}

impl GeneticAlgorithm {
    pub fn new() -> GeneticAlgorithm {
        GeneticAlgorithm
    }

    pub fn step<I>(&self, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                // TODO selection
                // TODO crossover
                // TODO mutation
                todo!()
            })
            .collect()
    }
}

impl RouletteWheelSelection {
    pub fn new() -> RouletteWheelSelection {
        RouletteWheelSelection
    }
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn rand::RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |i| i.fitness())
            .expect("Received empty population")
    }
}

#[cfg(test)]
#[derive(Clone, Debug)]
pub struct TestIndividual {
    fitness: f32,
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> TestIndividual {
        TestIndividual { fitness }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn fitness(&self) -> f32 {
        self.fitness
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use std::collections::BTreeMap;

    #[test]
    fn test() {
        let method = RouletteWheelSelection::new();
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let population = vec![
            TestIndividual::new(1.0),
            TestIndividual::new(2.0),
            TestIndividual::new(3.0),
            TestIndividual::new(4.0),
        ];

        let actual_histogram: BTreeMap<i32, _> = (0..1000)
            .map(|_| method.select(&mut rng, &population))
            .fold(Default::default(), |mut histogram, individual| {
                *histogram.entry(individual.fitness() as _).or_default() += 1;
                histogram
            });

        let expected_histogram = maplit::btreemap! {
            // fitness => selection count
            1 => 102,
            2 => 198,
            3 => 301,
            4 => 399,
        };

        assert_eq!(actual_histogram, expected_histogram);
    }
}
