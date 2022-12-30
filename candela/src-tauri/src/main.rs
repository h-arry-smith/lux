#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use lumen::Environment;
use lux::{evaluator::Evaluator, parser::parse};
use std::{fmt::Write, sync::Mutex};
use tauri::State;

#[tauri::command]
fn on_text_change(source: String, lockable_environment: State<LockableEnvironment>) -> String {
    let result = parse(&source);
    let mut console_text = String::new();

    match result {
        Ok(_) => {
            writeln!(console_text, "parse ok...").unwrap();
        }
        Err(err) => {
            writeln!(console_text, "parse error: {}", err).unwrap();
            return console_text;
        }
    }

    let environment = lockable_environment.env.lock().unwrap();
    let mut evaluator = Evaluator::new(environment.to_owned());

    match evaluator.evaluate(result.unwrap()) {
        Ok(()) => {
            writeln!(console_text, "{:#?}", evaluator.env.fixtures).unwrap();
        }
        Err(err) => {
            writeln!(console_text, "evaluation error: {}", err).unwrap();
        }
    }

    console_text
}

struct LockableEnvironment {
    env: Mutex<Environment>,
}

fn main() {
    let mut environment = Environment::new();

    for n in 1..=10 {
        environment.fixtures.create_with_id(n);
    }

    tauri::Builder::default()
        .manage(LockableEnvironment {
            env: Mutex::new(environment),
        })
        .invoke_handler(tauri::generate_handler![on_text_change])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
