#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::plugins::network::Network;
use lumen::{
    address::Address,
    fixture_set::ResolvedFixtureMap,
    output::{sacn::ACN_SDT_MULTICAST_PORT, NetworkState},
    parameter::{Param, Parameter},
    patch::FixtureProfile,
    timecode::Source,
    universe::Multiverse,
    Environment, Patch,
};
use lux::{evaluator::Evaluator, parser::parse};
use std::{fmt::Write, sync::Mutex, thread, time::Duration};
use tauri::{State, Window};

mod plugins;

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

    let mut environment = lockable_environment.env.lock().unwrap();
    let mut evaluator = Evaluator::new(&mut environment);

    match evaluator.evaluate(result.unwrap()) {
        Ok(()) => {
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

#[tauri::command]
fn init_tick(window: Window) {
    std::thread::spawn(move || loop {
        window.emit("tick", 0).unwrap();
        thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    });
}

// TODO: This has become a very large general tick function, which it should
//       not be, and all this should be factored out.
#[tauri::command]
fn resolve(
    window: Window,
    lockable_environment: State<LockableEnvironment>,
    source: State<Mutex<Source>>,
    network: State<Mutex<Network>>,
) -> ResolvedFixtureMap {
    let mut env = lockable_environment.env.lock().unwrap();
    let source = source.lock().unwrap();
    let mut network = network.lock().unwrap();

    if network.state() == NetworkState::Bound {
        network.try_connect(format!("127.0.0.1:{}", ACN_SDT_MULTICAST_PORT));

        if network.state() == NetworkState::Connected {
            // TODO: Must be a way of the plugin to handle its emits when handling
            //       connection and reconnections.
            window.emit("network/connected", ()).unwrap();
        }
    }

    let mut dimmer = FixtureProfile::new();
    dimmer.set_parameter(Param::Intensity, Parameter::simple(0));
    let mut quad = FixtureProfile::new();
    quad.set_colorspace(lumen::color::Colorspace::RGBA);
    quad.set_parameter(Param::Red, Parameter::simple(0));
    quad.set_parameter(Param::Green, Parameter::simple(1));
    quad.set_parameter(Param::Blue, Parameter::simple(2));
    quad.set_parameter(Param::Amber, Parameter::simple(3));
    let mut patch = Patch::new();

    for n in 1..=10 {
        patch.patch(n, Address::new(1, n as u16), &dimmer);
    }

    for n in 1..=9 {
        patch.patch(100 + n, Address::new(1, (91 + (n * 10)) as u16), &quad);
    }

    let t = source.time();
    env.run_to_time(t, &patch);
    let resolved_map = env.fixtures.resolve(t, &patch);

    // TODO: Very temporary dmx generation of the multiverse, should really be
    //       under some much cleaner interface, when we rewrite the candela app
    //       to have much clearer responsibilities.
    let mut multiverse = Multiverse::new();
    for (id, resolved_fixture) in resolved_map.iter() {
        let dmx_string = patch.get_profile(id).to_dmx(resolved_fixture);
        multiverse.map_string(patch.get_address(id), &dmx_string);
    }

    if network.state() == NetworkState::Connected && network.output_multiverse(&multiverse).is_err()
    {
        window.emit("network/disconnected", ()).unwrap();
    }

    resolved_map
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

    for n in 101..=109 {
        environment.fixtures.create_with_id(n);
    }

    tauri::Builder::default()
        .plugin(plugins::network::init())
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
            resolve,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
