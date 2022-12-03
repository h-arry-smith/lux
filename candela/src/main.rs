use std::{
    thread,
    time::{Duration, Instant},
};

use lumen::{parameter::Param, value::Percentage, value_generator::Fade, Environment};

fn main() {
    let mut environment = Environment::new();

    environment.fixtures.create_with_id(1);

    for (_, fixture) in environment.fixtures.all() {
        fixture.set(
            Param::Intensity,
            Box::new(Fade::new(
                Percentage::new(0.0),
                Percentage::new(100.0),
                Duration::new(10, 0),
            )),
        );
    }

    let now = Instant::now();
    for _ in 0..=10 {
        let elapsed = now.elapsed();

        for (_, fixture) in environment.fixtures.resolve(elapsed) {
            println!("@{:?} {:?}", elapsed, fixture);
        }

        thread::sleep(Duration::new(1, 0));
    }
}
