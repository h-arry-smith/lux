use std::{
    thread,
    time::{Duration, Instant},
};

use lumen::{
    address::Address,
    parameter::{Param, Parameter},
    patch::FixtureProfile,
    value::generator::Fade,
    value::Values,
    Environment, Patch,
};

fn main() {
    let mut environment = Environment::new();

    environment.fixtures.create_with_id(1);

    for (_, fixture) in environment.fixtures.all() {
        fixture.set(
            Param::Intensity,
            Box::new(Fade::new(
                Values::make_literal(10.0),
                Values::make_percentage(100.0),
                Duration::new(10, 0),
            )),
        );
    }

    let mut dimmer = FixtureProfile::new();
    dimmer.set_parameter(Param::Intensity, Parameter::new(0.0, 75.0));

    let mut patch = Patch::new();
    patch.patch(1, Address::new(0, 1), &dimmer);

    let now = Instant::now();
    for _ in 0..=10 {
        let elapsed = now.elapsed();

        for (_, fixture) in environment.fixtures.resolve(elapsed, &patch) {
            println!("@{:?} {:?}", elapsed, fixture);
        }

        thread::sleep(Duration::new(1, 0));
    }
}
