#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use lumen::{
    address::Address,
    fixture_set::ResolvedFixtureMap,
    parameter::{Param, Parameter},
    patch::FixtureProfile,
    timecode::{time::Time, Source},
    Environment, Patch,
};
use lux::{evaluator::Evaluator, parser::parse};
use std::{fmt::Write, sync::Mutex, thread, time::Duration};
use tauri::{State, Window};

#[tauri::command]
fn on_text_change(
    source: String,
    lockable_environment: State<LockableEnvironment>,
    time_source: State<Mutex<Source>>,
) -> String {
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

    let mut environment = lockable_environment.env.lock().unwrap();
    let mut evaluator = Evaluator::new(&mut environment);
    let evaluation_result = evaluator.evaluate(result.unwrap());

    match evaluation_result {
        Ok(()) => {
            time_source.lock().unwrap().seek(Time::at(0, 0, 0, 0));
            environment.run_to_time(Time::at(0, 0, 0, 0));
            writeln!(console_text, "{:#?}", environment.fixtures).unwrap();
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

    dbg!(&source);
    source.time().tc_string(source.fps())
}

#[tauri::command]
fn pause_time(source: State<Mutex<Source>>) -> String {
    let mut source = source.lock().unwrap();
    source.pause();
    dbg!(&source);
    source.time().tc_string(source.fps())
}

#[tauri::command]
fn stop_time(source: State<Mutex<Source>>) -> String {
    let mut source = source.lock().unwrap();
    source.stop();
    dbg!(&source);
    source.time().tc_string(source.fps())
}

#[tauri::command]
fn init_tick(window: Window) {
    std::thread::spawn(move || loop {
        window.emit("tick", 0).unwrap();
        thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    });
}

#[tauri::command]
fn resolve(
    lockable_environment: State<LockableEnvironment>,
    source: State<Mutex<Source>>,
) -> ResolvedFixtureMap {
    let mut env = lockable_environment.env.lock().unwrap();
    let source = source.lock().unwrap();

    let mut dimmer = FixtureProfile::new();
    dimmer.set_parameter(Param::Intensity, Parameter::new(0, 0.0, 100.0));
    let mut patch = Patch::new();

    for n in 1..=10 {
        patch.patch(n, Address::new(1, n as u16), &dimmer);
    }

    let t = source.time();
    env.run_to_time(t);
    env.fixtures.resolve(t, &patch)
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
            init_tick,
            on_text_change,
            get_current_time,
            start_time,
            pause_time,
            stop_time,
            resolve
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
