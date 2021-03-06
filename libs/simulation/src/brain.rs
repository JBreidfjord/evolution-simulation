use lib_neural_network::Activation;

use crate::*;

#[derive(Debug, Clone)]
pub struct Brain {
    pub(crate) nn: nn::Network,
}

impl Brain {
    pub fn random(rng: &mut dyn RngCore, eye: &Eye) -> Brain {
        Brain {
            nn: nn::Network::random(rng, &Self::topology(eye)),
        }
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        self.nn.weights().collect()
    }

    pub(crate) fn from_chromosome(chromosome: ga::Chromosome, eye: &Eye) -> Brain {
        Brain {
            nn: nn::Network::from_weights(&Self::topology(eye), chromosome),
        }
    }

    fn topology(eye: &Eye) -> [nn::LayerTopology; 3] {
        [
            nn::LayerTopology {
                neurons: eye.cells(),
                activation: Activation::ReLU,
            },
            nn::LayerTopology {
                neurons: 2 * eye.cells(),
                activation: Activation::ReLU,
            },
            nn::LayerTopology {
                neurons: 2,
                activation: Activation::Tanh,
            },
        ]
    }
}
