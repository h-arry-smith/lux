use std::{thread, time::Duration};

use lumen::{
    address::Address,
    parameter::{Param, Parameter},
    patch::FixtureProfile,
    timecode::{time::Time, FrameRate, Source},
    track::{Action, Applicator, Selection, Track},
    universe::Multiverse,
    value::{
        generator::{Fade, Static},
        Values,
    },
    Environment, Patch, QueryBuilder,
};

// TODO: This is fine for testing purposes but we need to think about the right
//       architecture for this.
//
//       After constructing an environment of fixtures, and a patch for that
//       environemnt, we need to resolve it for a given, time and then apply
//       it to the multiverse.
//
//       It is very important that this interface is right, as it is the most
//       outward facing part of the whole thing.

fn main() {
    let mut environment = Environment::new();
    let mut dimmer = FixtureProfile::new();
    dimmer.set_parameter(Param::Intensity, Parameter::new(0, 0.0, 100.0));
    let mut patch = Patch::new();

    for n in 1..=10 {
        environment.fixtures.create_with_id(n);
        patch.patch(n, Address::new(1, n as u16), &dimmer);
    }

    // In reality you wouldn't manually build it up like this, but this is just
    // for testing, better interfacts to come!
    let mut track = Track::new(0);
    let mut action = Action::new(Time::at(0, 0, 1, 0, environment.timecode(0).fps()));
    let mut selection = Selection::new(QueryBuilder::new().all().build());
    selection.add_applicator(Applicator::new(
        Param::Intensity,
        Box::new(Fade::new(
            Static::new(Values::make_literal(0.0)),
            Static::new(Values::make_literal(100.0)),
            Duration::new(2, 0),
        )),
    ));

    action.add_selection(selection);
    track.add_action(action);

    environment.set_track(track);
    environment.timecode(0).start();

    for _ in 0..=10 {
        let time = environment.timecode(0).time();
        println!("@{:?}", time);

        environment.tick();

        for (_, resolved_fixture) in environment.fixtures.resolve(time, &patch) {
            println!("    {:?}", resolved_fixture);
        }

        thread::sleep(Duration::new(0, 2_000_000_000 / 10));
    }

    println!("=== FIXTURE DMX ===");
    let mut multiverse = Multiverse::new();
    let time = environment.timecode(0).time();

    for (id, resolved_fixture) in environment.fixtures.resolve(time, &patch) {
        let dmx_string = patch.get_profile(&id).to_dmx(&resolved_fixture);
        println!("{}: {:?}", id, dmx_string);

        multiverse.map_string(patch.get_address(&id), &dmx_string);
    }

    println!("=== FIXTURE DMX ===");
    println!("{:?}", multiverse)
}
