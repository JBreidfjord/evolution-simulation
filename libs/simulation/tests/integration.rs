use lib_simulation as sim;

#[test]
fn test_simulation() {
    let mut rng = rand::thread_rng();
    let mut config = sim::Config::default();
    config.energy_loss_factor = 2.0;
    config.reproduction_cost = 25.0;
    config.reproduction_threshold = 50.0;
    config.population_count = 80;
    config.food_count = 50;

    let mut simulation = sim::Simulation::random(&mut rng, Some(config));
    for i in 0..5000 {
        simulation.step(&mut rng);
        if i % 100 == 0 {
            println!("Step {} Pop = {}", i, simulation.world().creatures().len());
        }
        if simulation.world().creatures().len() == 0 {
            break;
        }
    }
}
