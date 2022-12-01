use std::{
    thread,
    time::{Duration, Instant},
};

use lumen::{parameter::Param, value::Literal, value_generator::Fade, Environment};

fn main() {
    let mut environment = Environment::new();

    environment.fixtures.create_with_id(1);

    for (_, fixture) in environment.fixtures.all() {
        fixture.set(
            Param::Intensity,
            Box::new(Fade::new(
                Literal::new(0.0),
                Literal::new(100.0),
                Duration::new(10, 0),
            )),
        );
    }

    let now = Instant::now();
    for n in 0..=10 {
        for (_, fixture) in environment.fixtures.resolve(now.elapsed()) {
            println!("@{n}s {:?}", fixture);
        }

        thread::sleep(Duration::new(1, 0));
    }
}
