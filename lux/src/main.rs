use std::fmt::Write;
use std::fs::{self, DirEntry, File};
use std::io::BufRead;
use std::io::BufReader;

use lumen::{timecode::time::Time, Environment};
use lux::{evaluator::Evaluator, parser::parse};

fn main() {
    // Gather all the examples in the example directory
    let examples = fs::read_dir("./examples").expect("could not read examples directory");
    let examples: Vec<DirEntry> = examples
        .filter(|file| file.as_ref().unwrap().path().extension().unwrap() == "lux")
        .map(|file| file.unwrap())
        .collect();

    println!("Running {} examples...", examples.len());

    for example in examples {
        run_example(example);
    }

    // Parse and evaluate each example and check the output
}

fn run_example(example: DirEntry) {
    let input = fs::read_to_string(example.path())
        .unwrap_or_else(|_| panic!("failed to load file: {}", example.path().to_string_lossy()));

    let result = parse(&input);

    if let Ok(program) = result {
        // TODO: This environment is temporary
        let mut environment = Environment::new();

        for n in 1..=10 {
            environment.fixtures.create_with_id(n);
        }

        let mut evaluator = Evaluator::new(&mut environment);

        match evaluator.evaluate(program) {
            Ok(()) => {
                environment.run_to_time(Time::at(0, 0, 0, 0));

                // Compare environment state to expected output defined in example file
                let test_output = environment_test_output(&environment);
                let expected_output = get_expected_output(&example);

                if test_output == expected_output {
                    display_test_result(example, true);
                } else {
                    println!("=== RESULT ===");
                    println!("{}", test_output);
                    println!("=== EXPECTED ===");
                    println!("{}", expected_output);
                    display_test_result(example, false);
                }
            }
            Err(err) => {
                display_test_result(example, false);
                eprintln!("error: {}", err);
            }
        }
    } else {
        eprintln!("error: {}", result.err().unwrap());
        display_test_result(example, false);
    }
}

fn display_test_result(example: DirEntry, result: bool) {
    let right = "✅";
    let wrong = "❌";
    if result {
        println!("{:<75} [{}]", example.path().display(), right);
    } else {
        println!("{:<75} [{}]", example.path().display(), wrong);
    }
}

fn environment_test_output(environment: &Environment) -> String {
    let mut output = String::new();

    let mut sorted_ids: Vec<usize> = environment.fixtures.ids().iter().cloned().collect();
    sorted_ids.sort();

    let mut empty_fixtures = Vec::new();

    for id in sorted_ids {
        let fixture = environment.fixtures.get(&id).unwrap();

        if fixture.parameters().is_empty() {
            empty_fixtures.push(id);
            continue;
        }

        writeln!(output, "FIXTURE {}", id).unwrap();

        for (param, generators) in fixture.parameters() {
            writeln!(output, "  {}", param).unwrap();

            for generator in generators {
                writeln!(output, "    {}", generator).unwrap();
            }
        }
    }

    if !empty_fixtures.is_empty() {
        empty_fixtures.sort();
        writeln!(
            output,
            "FIXTURES {}",
            empty_fixtures
                .iter()
                .map(|id| format!("{}", id))
                .collect::<Vec<String>>()
                .join(" ")
        )
        .unwrap();
        writeln!(output, "  NONE").unwrap();
    }

    output
}

fn get_expected_output(example: &DirEntry) -> String {
    let mut output = String::new();
    let f = File::open(example.path()).unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap();

        if let Some(out_str) = line.strip_prefix("/// ") {
            writeln!(output, "{}", out_str).unwrap();
        }
    }

    output
}
