#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use lumen::{timecode::Source, Environment};
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

#[tauri::command]
fn get_current_time(source: State<Mutex<Source>>) -> String {
    let source = source.lock().unwrap();
    source.time().tc_string(source.fps())
}

#[tauri::command]
fn start_time(source: State<Mutex<Source>>) -> String {
    let mut source = source.lock().unwrap();

    if source.paused() {
        source.resume();
    } else {
        source.start();
    }

    source.time().tc_string(source.fps())
}

#[tauri::command]
fn pause_time(source: State<Mutex<Source>>) -> String {
    let mut source = source.lock().unwrap();
    source.pause();
    source.time().tc_string(source.fps())
}

#[tauri::command]
fn stop_time(source: State<Mutex<Source>>) -> String {
    let mut source = source.lock().unwrap();
    source.stop();
    source.time().tc_string(source.fps())
}
struct LockableEnvironment {
    env: Mutex<Environment>,
}

fn main() {
    let mut environment = Environment::new();
    let source = Source::new(lumen::timecode::FrameRate::Thirty);

    for n in 1..=10 {
        environment.fixtures.create_with_id(n);
    }

    tauri::Builder::default()
        .manage(LockableEnvironment {
            env: Mutex::new(environment),
        })
        .manage(Mutex::new(source))
        .invoke_handler(tauri::generate_handler![
            on_text_change,
            get_current_time,
            start_time,
            pause_time,
            stop_time,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
