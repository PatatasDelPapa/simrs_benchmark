use simrs_benchmark::test_simulation;

use std::time::Duration;

fn main() {
    let (mut simulation, count_key) = test_simulation();
    let limit = Duration::from_secs(1002);
    let clock = simulation.scheduler.clock();
    while simulation.step() && clock.time() < limit {}
    
    let state = &mut simulation.state;
    let count = *state.get(count_key).unwrap();
    println!("Final Count = {}", count);
    println!("Final Time = {:?}", clock.time());
}