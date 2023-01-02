use std::{env, fs};

use lumen::{timecode::time::Time, Environment};
// TODO: Remove this file and build a proper test runner against real .lux files!
use lux::{evaluator::Evaluator, parser::parse};

fn main() {
    let filename = env::args().nth(1).expect("provide a filename");
    let input = fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("failed to load file: {}", filename));

    let result = parse(&input);

    eprintln!("{:#?}:", result);

    if let Ok(program) = result {
        // TODO: We only pass in an environment because we are going to build a
        //       patch by hand, but maybe our evalautor does this for us?
        //       Unless we seperate patch syntax out from Lux, and it has it's
        //       own parser and evaluator, in which case each evaluator forms
        //       it's own pass...

        let mut environment = Environment::new();

        for n in 1..=10 {
            environment.fixtures.create_with_id(n);
        }

        let mut evaluator = Evaluator::new(&mut environment);

        match evaluator.evaluate(program) {
            Ok(()) => {
                environment.run_to_time(Time::at(0, 0, 0, 0));
                println!("{:#?}", environment.fixtures);
            }
            Err(err) => eprintln!("error: {}", err),
        }
    }
}
