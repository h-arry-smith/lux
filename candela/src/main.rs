use std::{
    thread,
    time::{Duration, Instant},
};

use lumen::{
    address::Address,
    parameter::{Param, Parameter},
    patch::FixtureProfile,
    universe::Multiverse,
    value::{generator::Static, Values},
    Environment, Patch, QueryBuilder,
};

// TODO: This is fine for testing purposes but we need to think about the right
//       architecture for this.
//
//       After constructing an environment of fixtures, and a patch for that
//       environemnt, we need to resolve it for a given, time and then apply
//       it to the multiverse.
//
//       It is very important that this interface is right, as it is the most
//       outward facing part of the whole thing.

fn main() {
    let mut environment = Environment::new();
    let mut dimmer = FixtureProfile::new();
    dimmer.set_parameter(Param::Intensity, Parameter::new(0, 0.0, 100.0));
    let mut patch = Patch::new();

    for n in 1..=10 {
        environment.fixtures.create_with_id(n);
        patch.patch(n, Address::new(1, n as u16), &dimmer);
    }

    for (_, fixture) in environment.fixtures.all() {
        fixture.set(
            Param::Intensity,
            Box::new(Static::new(Values::make_literal(50.0))),
        );
    }

    let query = QueryBuilder::new().all().even().id(7).build();
    let result = query.evaluate(&environment.fixtures);

    for (_, fixture) in environment.query_fixtures(&result) {
        fixture.set(
            Param::Intensity,
            Box::new(Static::new(Values::make_literal(100.0))),
        )
    }

    let start_time = Instant::now();

    for _ in 0..=10 {
        println!("@{:?}", start_time.elapsed());
        for (_, resolved_fixture) in environment.fixtures.resolve(start_time.elapsed(), &patch) {
            println!("    {:?}", resolved_fixture);
        }

        thread::sleep(Duration::new(0, 2_000_000_000 / 10));
    }

    println!("=== FIXTURE DMX ===");
    let mut multiverse = Multiverse::new();

    for (id, resolved_fixture) in environment.fixtures.resolve(Duration::new(10, 0), &patch) {
        let dmx_string = patch.get_profile(&id).to_dmx(&resolved_fixture);
        println!("{}: {:?}", id, dmx_string);

        multiverse.map_string(patch.get_address(&id), &dmx_string);
    }

    println!("=== FIXTURE DMX ===");
    println!("{:?}", multiverse)
}
