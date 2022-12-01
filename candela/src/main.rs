use std::{
    thread,
    time::{Duration, Instant},
};

use lumen::{parameter::Param, value::Literal, value_generator::StaticGenerator, Environment};

fn main() {
    let mut environment = Environment::new();

    environment.fixtures.create_with_id(1);

    for (_, fixture) in environment.fixtures.all() {
        fixture.set(Param::Intensity, StaticGenerator::new(Literal::new(50)));
        println!("{:?}", fixture);
    }

    let now = Instant::now();
    for n in 0..10 {
        for (_, fixture) in environment.fixtures.resolve(now.elapsed()) {
            println!("@{n}s {:?}", fixture);
        }

        thread::sleep(Duration::new(1, 0));
    }
}
