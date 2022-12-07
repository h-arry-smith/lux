use std::{
    thread,
    time::{Duration, Instant},
};

use lumen::{
    address::Address,
    parameter::{Param, Parameter},
    patch::FixtureProfile,
    universe::Multiverse,
    value::generator::Fade,
    value::{generator::Static, Values},
    Environment, Patch,
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

    environment.fixtures.create_with_id(1);

    for (_, fixture) in environment.fixtures.all() {
        fixture.set(
            Param::Intensity,
            Box::new(Fade::new(
                Static::new(Values::make_percentage(10.0)),
                Static::new(Values::make_percentage(100.0)),
                Duration::new(2, 0),
            )),
        );
    }

    let mut dimmer = FixtureProfile::new();
    dimmer.set_parameter(Param::Intensity, Parameter::new(0, 0.0, 75.0));

    let mut patch = Patch::new();
    patch.patch(1, Address::new(1, 1), &dimmer);

    let start_time = Instant::now();

    for _ in 0..=10 {
        for (_, resolved_fixture) in environment.fixtures.resolve(start_time.elapsed(), &patch) {
            println!("@{:?} {:?}", start_time.elapsed(), resolved_fixture);
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
