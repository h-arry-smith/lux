use lumen::{
    action::{Action, Apply, ApplyGroup},
    address::Address,
    parameter::{Param, Parameter},
    patch::FixtureProfile,
    timecode::{time::Time, FrameRate, Source},
    track::Track,
    universe::Multiverse,
    value::{
        generator::{Fade, Static},
        Values,
    },
    Environment, Patch, QueryBuilder,
};
use std::{thread, time::Duration};

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

    let mut action1 = Action::new();
    let query = QueryBuilder::new().all().build();
    let fade = Fade::new(
        Static::new(Values::make_percentage(0.0)),
        Static::new(Values::make_percentage(100.0)),
        Duration::new(2, 0),
    );
    let apply = Apply::new(Param::Intensity, Box::new(fade));
    let mut apply_group = ApplyGroup::new(query);
    apply_group.add_apply(apply);

    action1.add_group(apply_group);

    let mut action2 = Action::new();
    let query = QueryBuilder::new().all().even().build();
    let fade = Fade::new(
        Static::new(Values::make_percentage(100.0)),
        Static::new(Values::make_percentage(50.0)),
        Duration::new(2, 0),
    );
    let apply = Apply::new(Param::Intensity, Box::new(fade));
    let mut apply_group = ApplyGroup::new(query);
    apply_group.add_apply(apply);

    action2.add_group(apply_group);

    let mut timer = Source::new(FrameRate::Thirty);
    let mut track = Track::new();
    track.add_action(Time::at(0, 0, 0, 0), action1);
    track.add_action(Time::at(0, 0, 2, 0), action2);

    environment.generate_history();

    for _ in 0..2 {
        timer.start();
        for _ in 0..=4 {
            println!("@{}", timer.time().tc_string(FrameRate::Thirty));

            track.apply_actions(timer.time(), &mut environment);

            for (_, resolved_fixture) in environment.fixtures.resolve(timer.time(), &patch) {
                println!("    {:?}", resolved_fixture);
            }

            thread::sleep(Duration::new(1, 0));
        }
    }

    println!("=== FIXTURE DMX ===");
    let mut multiverse = Multiverse::new();

    for (id, resolved_fixture) in environment.fixtures.resolve(timer.time(), &patch) {
        let dmx_string = patch.get_profile(&id).to_dmx(&resolved_fixture);
        println!("{}: {:?}", id, dmx_string);

        multiverse.map_string(patch.get_address(&id), &dmx_string);
    }

    println!("=== FIXTURE DMX ===");
    println!("{:?}", multiverse)
}
