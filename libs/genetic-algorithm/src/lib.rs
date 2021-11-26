#![feature(type_alias_impl_trait)]

use rand::seq::SliceRandom;
use rand::Rng;
use std::iter::FromIterator;
use std::ops::Index;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

pub struct RouletteWheelSelection;

pub struct UniformCrossover;

pub struct GaussianMutation {
    // Probability of changing a gene
    rate: f32,
    // Magnitude of change
    factor: f32,
}

#[derive(Clone, Debug)]
pub struct Chromosome {
    genes: Vec<f32>,
}

pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
}

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn rand::RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}

pub trait CrossoverMethod {
    fn crossover(
        &self,
        rng: &mut dyn rand::RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
}

pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn rand::RngCore, child: &mut Chromosome);
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static,
    ) -> GeneticAlgorithm<S> {
        GeneticAlgorithm {
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
        }
    }

    pub fn step<I>(&self, rng: &mut dyn rand::RngCore, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                // Selection
                let parent_a = self.selection_method.select(rng, population).chromosome();
                let parent_b = self.selection_method.select(rng, population).chromosome();

                // Crossover
                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);

                // Mutation
                self.mutation_method.mutate(rng, &mut child);

                // TODO convert Chromosome to Individual
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

impl UniformCrossover {
    pub fn new() -> UniformCrossover {
        UniformCrossover
    }
}

impl CrossoverMethod for UniformCrossover {
    fn crossover(
        &self,
        rng: &mut dyn rand::RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());

        let parent_a = parent_a.iter();
        let parent_b = parent_b.iter();

        parent_a
            .zip(parent_b)
            .map(|(&a, &b)| if rng.gen_bool(0.5) { a } else { b })
            .collect()
    }
}

impl GaussianMutation {
    pub fn new(rate: f32, factor: f32) -> GaussianMutation {
        assert!(rate >= 0.0 && rate <= 1.0);

        GaussianMutation { rate, factor }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn rand::RngCore, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            let sign = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };

            if rng.gen_bool(self.rate as _) {
                *gene += sign * self.factor * rng.gen::<f32>();
            }
        }
    }
}

impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}

impl Index<usize> for Chromosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Chromosome {
        Chromosome {
            genes: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = impl Iterator<Item = f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
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

    fn chromosome(&self) -> &Chromosome {
        panic!("Not supported for TestIndividual")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod selection {
        use super::*;

        mod roulettewheel {
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

    mod crossover {
        use super::*;

        mod uniform {
            use super::*;
            use rand::SeedableRng;
            use rand_chacha::ChaCha8Rng;

            #[test]
            fn test() {
                let mut rng = ChaCha8Rng::from_seed(Default::default());
                let parent_a: Chromosome = (1..=100).map(|n| n as f32).collect();
                let parent_b: Chromosome = (1..=100).map(|n| -n as f32).collect();
                let child = UniformCrossover::new().crossover(&mut rng, &parent_a, &parent_b);

                // Number of genes different between 'child' and 'parent_a'
                let diff_a = child.iter().zip(parent_a).filter(|(c, p)| *c != p).count();

                // Number of genes different between 'child' and 'parent_b'
                let diff_b = child.iter().zip(parent_b).filter(|(c, p)| *c != p).count();

                assert_eq!(diff_a, 49);
                assert_eq!(diff_b, 51);
            }
        }
    }

    mod mutation {
        use super::*;

        mod gaussian {
            use super::*;
            use rand::SeedableRng;
            use rand_chacha::ChaCha8Rng;

            fn actual(rate: f32, factor: f32) -> Vec<f32> {
                let mut child = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();

                let mut rng = ChaCha8Rng::from_seed(Default::default());

                GaussianMutation::new(rate, factor).mutate(&mut rng, &mut child);

                child.into_iter().collect()
            }

            mod given_zero_rate {
                use approx::assert_relative_eq;

                fn actual(factor: f32) -> Vec<f32> {
                    super::actual(0.0, factor)
                }

                mod and_zero_factor {
                    use super::*;

                    #[test]
                    fn does_not_change_original_chromosome() {
                        let actual = actual(0.0);
                        let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                        assert_relative_eq!(actual.as_slice(), expected.as_slice());
                    }
                }

                mod and_nonzero_factor {
                    use super::*;

                    #[test]
                    fn does_not_change_original_chromosome() {
                        let actual = actual(0.5);
                        let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                        assert_relative_eq!(actual.as_slice(), expected.as_slice());
                    }
                }
            }
            mod given_half_rate {
                use approx::assert_relative_eq;

                fn actual(factor: f32) -> Vec<f32> {
                    super::actual(0.5, factor)
                }

                mod and_zero_factor {
                    use super::*;

                    #[test]
                    fn does_not_change_original_chromosome() {
                        let actual = actual(0.0);
                        let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                        assert_relative_eq!(actual.as_slice(), expected.as_slice());
                    }
                }

                mod and_nonzero_factor {
                    use super::*;

                    #[test]
                    fn slightly_changes_original_chromosome() {
                        let actual = actual(0.5);
                        let expected = vec![1.0, 1.7756249, 3.0, 4.1596804, 5.0];

                        assert_relative_eq!(actual.as_slice(), expected.as_slice());
                    }
                }
            }
            mod given_max_rate {
                use approx::assert_relative_eq;

                fn actual(factor: f32) -> Vec<f32> {
                    super::actual(1.0, factor)
                }

                mod and_zero_factor {
                    use super::*;

                    #[test]
                    fn does_not_change_original_chromosome() {
                        let actual = actual(0.0);
                        let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                        assert_relative_eq!(actual.as_slice(), expected.as_slice());
                    }
                }

                mod and_nonzero_factor {
                    use super::*;

                    #[test]
                    fn changes_original_chromosome() {
                        let actual = actual(0.5);
                        let expected = vec![1.4545316, 2.1162078, 2.7756248, 3.9505124, 4.638691];

                        assert_relative_eq!(actual.as_slice(), expected.as_slice());
                    }
                }
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
