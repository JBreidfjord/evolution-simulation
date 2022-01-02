use lib_simulation as sim;

#[test]
fn test_simulation() {
    let mut rng = rand::thread_rng();
    let config = sim::Config::default();

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
