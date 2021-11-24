#[cfg(test)]
mod tests {
    use super::*;

    mod neuron {
        use super::*;

        mod random {
            use super::*;
            use approx::assert_relative_eq;
            use rand::SeedableRng;
            use rand_chacha::ChaCha8Rng;

            #[test]
            fn test() {
                let mut rng = ChaCha8Rng::from_seed(Default::default());
                let neuron = Neuron::random(&mut rng, 2);

                assert_relative_eq!(neuron.bias, 0.8181262);
                assert_relative_eq!(neuron.weights.as_slice(), [-0.6255188, 0.67383957].as_ref());
            }
        }

        mod propagate {
            use super::*;
            use approx::assert_relative_eq;

            #[test]
            fn test() {
                let neuron = Neuron {
                    weights: vec![0.5, 0.5],
                    bias: 0.5,
                };

                // Ensures ReLU activation function is used
                assert_relative_eq!(neuron.propagate(&vec![-10.0, -10.0]), 0.0);

                assert_relative_eq!(neuron.propagate(&vec![1.0, 0.5]), 1.25);
            }
        }
    }

    mod layer {
        use super::*;

        mod random {
            use super::*;
            use approx::assert_relative_eq;
            use rand::SeedableRng;
            use rand_chacha::ChaCha8Rng;

            #[test]
            fn test() {
                let mut rng = ChaCha8Rng::from_seed(Default::default());
                let layer = Layer::random(&mut rng, 2, 2);

                assert_relative_eq!(layer.neurons[0].bias, 0.8181262);
                assert_relative_eq!(
                    layer.neurons[0].weights.as_slice(),
                    [-0.6255188, 0.67383957].as_ref()
                );

                assert_relative_eq!(layer.neurons[1].bias, -0.53516835);
                assert_relative_eq!(
                    layer.neurons[1].weights.as_slice(),
                    [0.26284897, 0.5238807].as_ref()
                );
            }
        }

        mod propagate {
            use super::*;
            use approx::assert_relative_eq;

            #[test]
            fn test() {
                let layer = Layer {
                    neurons: vec![
                        Neuron {
                            weights: vec![0.25, 0.75],
                            bias: 0.0,
                        },
                        Neuron {
                            weights: vec![0.5, 0.5],
                            bias: 0.5,
                        },
                    ],
                };

                let prop = layer.propagate(vec![0.3, 0.6]);
                assert_relative_eq!(prop.as_slice(), [0.525, 0.95].as_ref());
            }
        }
    }

    mod network {
        use super::*;

        mod random {
            use super::*;
            use approx::assert_relative_eq;
            use rand::SeedableRng;
            use rand_chacha::ChaCha8Rng;

            #[test]
            fn test() {
                let mut rng = ChaCha8Rng::from_seed(Default::default());
                let network = Network::random(
                    &mut rng,
                    &vec![LayerTopology { neurons: 2 }, LayerTopology { neurons: 2 }],
                );

                assert_eq!(network.layers.len(), 1);
                assert_eq!(network.layers[0].neurons.len(), 2);

                assert_relative_eq!(network.layers[0].neurons[0].bias, 0.8181262);
                assert_relative_eq!(
                    network.layers[0].neurons[0].weights.as_slice(),
                    [-0.6255188, 0.67383957].as_ref()
                );

                assert_relative_eq!(network.layers[0].neurons[1].bias, -0.53516835);
                assert_relative_eq!(
                    network.layers[0].neurons[1].weights.as_slice(),
                    [0.26284897, 0.5238807].as_ref()
                );
            }
        }

        mod propagate {
            use super::*;
            use approx::assert_relative_eq;

            #[test]
            fn test() {
                let network = Network {
                    layers: vec![
                        Layer {
                            neurons: vec![
                                Neuron {
                                    weights: vec![0.25, 0.75],
                                    bias: 0.0,
                                },
                                Neuron {
                                    weights: vec![0.5, 0.5],
                                    bias: 0.5,
                                },
                            ],
                        },
                        Layer {
                            neurons: vec![
                                Neuron {
                                    weights: vec![0.25, 0.75],
                                    bias: 0.0,
                                },
                                Neuron {
                                    weights: vec![0.5, 0.5],
                                    bias: 0.5,
                                },
                            ],
                        },
                    ],
                };

                let prop = network.propagate(vec![0.3, 0.6]);
                assert_relative_eq!(prop.as_slice(), [0.84375, 1.2375].as_ref());
            }
        }
    }
}
