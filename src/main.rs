use simrs_benchmark::simulation;
use simrs_benchmark::test_simulation;

use std::time::Duration;

fn main() {
    simulation(50000);
}

#[allow(dead_code)]
fn testing_simulation() {
    let (mut simulation, count_key) = test_simulation();
    let limit = Duration::from_secs(1002);
    let clock = simulation.scheduler.clock();
    while simulation.step() && clock.time() < limit {}

    let state = &mut simulation.state;
    let count = *state.get(count_key).unwrap();
    println!("Final Count = {}", count);
    println!("Final Time = {:?}", clock.time());
}
