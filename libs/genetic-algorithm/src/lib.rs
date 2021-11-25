use rand::seq::SliceRandom;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
}

pub struct RouletteWheelSelection;

#[derive(Clone, Debug)]
pub struct Chromosome {
    genes: Vec<f32>,
}

pub trait Individual {
    fn fitness(&self) -> f32;
}

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn rand::RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(selection_method: S) -> GeneticAlgorithm<S> {
        GeneticAlgorithm { selection_method }
    }

    pub fn step<I>(&self, rng: &mut dyn rand::RngCore, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                let parent_a = self.selection_method.select(rng, population);
                let parent_b = self.selection_method.select(rng, population);
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

    mod selection {
        use super::*;

        mod roulettewheelselection {
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
    }

    mod chromosome {
        use super::*;

        mod index {
            use super::*;

            #[test]
            fn test() {
                let chromosome = Chromosome {
                    genes: vec![1.0, 2.0, 3.0],
                };

                assert_eq!(chromosome[0], 1.0);
                assert_eq!(chromosome[1], 2.0);
                assert_eq!(chromosome[2], 3.0);
            }
        }

        mod from_iterator {
            use super::*;

            #[test]
            fn test() {
                let chromosome: Chromosome = vec![1.0, 2.0, 3.0].into_iter().collect();

                assert_eq!(chromosome[0], 1.0);
                assert_eq!(chromosome[1], 2.0);
                assert_eq!(chromosome[2], 3.0);
            }
        }

        mod into_iterator {
            use super::*;

            #[test]
            fn test() {
                let chromosome = Chromosome {
                    genes: vec![1.0, 2.0, 3.0],
                };

                let genes: Vec<_> = chromosome.into_iter().collect();

                assert_eq!(genes.len(), 3);
                assert_eq!(genes[0], 1.0);
                assert_eq!(genes[1], 2.0);
                assert_eq!(genes[2], 3.0);
            }
        }
    }
}
