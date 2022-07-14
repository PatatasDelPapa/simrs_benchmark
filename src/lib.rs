use std::time::Duration;

use simrs::{Component, ComponentId, Key, Simulation};

#[derive(PartialEq, Clone, Copy)]
enum Passivated {
    True,
    False,
    Warned,
}

#[derive(Debug)]
struct Product;

struct Producer {
    count: Key<usize>,
    consumer: ComponentId<ConsumerEvent>,
    thresh_hold: usize,
    interval: Duration,
    produce_ammount: usize,
    // 0 for producer | 1 for consumer
    passivated_key: Key<[Passivated; 2]>,
}

#[derive(Debug)]
struct ProducerEvent;

impl Component for Producer {
    type Event = ProducerEvent;

    fn process_event(
        &self,
        self_id: simrs::ComponentId<Self::Event>,
        _event: &Self::Event,
        scheduler: &mut simrs::Scheduler,
        state: &mut simrs::State,
    ) {
        let passivated_list = state.get_mut(self.passivated_key).unwrap();
        passivated_list[0] = Passivated::False;

        if passivated_list[1] == Passivated::True {
            passivated_list[1] = Passivated::Warned;
            scheduler.schedule_now(self.consumer, ConsumerEvent);
        }

        let count = state.get_mut(self.count).unwrap();
        if *count < self.thresh_hold {
            *count += self.produce_ammount;
            // println!("PRODUCED - Before: {} | After: {} | At: {:?}", *count - self.produce_ammount, count, scheduler.time());
            scheduler.schedule(self.interval, self_id, ProducerEvent);
        } else {
            let passivated_list = state.get_mut(self.passivated_key).unwrap();
            passivated_list[0] = Passivated::True;
        }
    }
}

struct Consumer {
    count: Key<usize>,
    producer: Key<Option<ComponentId<ProducerEvent>>>,
    interval: Duration,
    // 0 for producer | 1 for consumer
    passivated_key: Key<[Passivated; 2]>,
    consume_ammount: usize,
}

#[derive(Debug)]
struct ConsumerEvent;

impl Component for Consumer {
    type Event = ConsumerEvent;

    fn process_event(
        &self,
        self_id: ComponentId<Self::Event>,
        _event: &Self::Event,
        scheduler: &mut simrs::Scheduler,
        state: &mut simrs::State,
    ) {
        let producer = state.get(self.producer).unwrap().unwrap();

        let passivated_list = state.get_mut(self.passivated_key).unwrap();
        passivated_list[1] = Passivated::False;
        
        if passivated_list[0] == Passivated::True {
            passivated_list[0] = Passivated::Warned;
            scheduler.schedule_now(producer, ProducerEvent);
        }

        let count = state.get_mut(self.count).unwrap();
        if *count >= self.consume_ammount {
            *count -= self.consume_ammount;
            // println!("CONSUMED - Before: {} | After: {} | At: {:?}", *count + self.consume_ammount, count, scheduler.time());
            scheduler.schedule(self.interval, self_id, ConsumerEvent);
        } else {
            let passivated_list = state.get_mut(self.passivated_key).unwrap();
            passivated_list[1] = Passivated::True;
        }
    }
}

pub fn set_simulation() -> Simulation {
    let mut simulation = Simulation::default();
    let state = &mut simulation.state;
    let count = state.insert(0);
    let producer_key = state.insert(None);
    let passivated_key = state.insert([Passivated::False; 2]);

    let consumer = simulation.add_component(Consumer {
        count,
        producer: producer_key,
        interval: Duration::from_secs(8),
        passivated_key,
        consume_ammount: 8,
    });
    let producer = simulation.add_component(Producer {
        consumer,
        count,
        thresh_hold: 15,
        interval: Duration::from_secs(2),
        produce_ammount: 1,
        passivated_key,
    });

    let state = &mut simulation.state;
    let producer_key = state.get_mut(producer_key).unwrap();
    *producer_key = Some(producer);

    simulation.schedule(Duration::ZERO, producer, ProducerEvent);
    simulation.schedule(Duration::ZERO, consumer, ConsumerEvent);
    simulation
}

pub fn test_simulation() -> (Simulation, Key<usize>) {
    let mut simulation = Simulation::default();
    let state = &mut simulation.state;
    let count = state.insert(0);
    let producer_key = state.insert(None);
    let passivated_key = state.insert([Passivated::False; 2]);

    let consumer = simulation.add_component(Consumer {
        count,
        producer: producer_key,
        interval: Duration::from_secs(1),
        passivated_key,
        consume_ammount: 8,
    });
    let producer = simulation.add_component(Producer {
        consumer,
        count,
        thresh_hold: 15,
        interval: Duration::from_secs(2),
        produce_ammount: 1,
        passivated_key,
    });

    let state = &mut simulation.state;
    let producer_key = state.get_mut(producer_key).unwrap();
    *producer_key = Some(producer);

    simulation.schedule(Duration::ZERO, producer, ProducerEvent);
    simulation.schedule(Duration::ZERO, consumer, ConsumerEvent);

    (simulation, count)
}

pub fn simulation(limit: u64) {
    let limit = Duration::from_secs(limit);
    let mut simulation = set_simulation();
    let clock = simulation.scheduler.clock();
    while simulation.step() && clock.time() <= limit {}
}
