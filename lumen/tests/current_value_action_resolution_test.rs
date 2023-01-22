use std::time::Duration;

use lumen::{
    action::{Action, Apply, ApplyGroup},
    address::Address,
    parameter::{Param, Parameter},
    patch::FixtureProfile,
    timecode::time::Time,
    track::Track,
    value::{
        generator::{CurrentValue, Fade, Static},
        Values,
    },
    Environment, Patch, QueryBuilder,
};

#[test]
fn simple() {
    let mut environment = Environment::new();
    environment.fixtures.create_with_id(1);
    let mut patch = Patch::new();
    let mut dimmer = FixtureProfile::new();
    dimmer.set_parameter(Param::Intensity, Parameter::new(0, 0.0, 100.0));
    patch.patch(1, Address::new(1, 1), &dimmer);

    let mut track = Track::new();

    track.add_action(Time::at(0, 0, 0, 0), fading_action());
    track.add_action(Time::at(0, 0, 2, 0), capture());

    environment.add_track(track);

    environment.run_to_time(Time::at(0, 0, 4, 0), &patch);

    let resolved = environment.fixtures.resolve(Time::at(0, 0, 4, 0), &patch);

    assert_eq!(
        *resolved
            .get(&1)
            .unwrap()
            .get_value(&Param::Intensity)
            .unwrap(),
        Values::make_literal(50.0)
    )
}

//  0 - 100
// 50 - 100
// 75 - 100
//    *     = 87.50
#[test]
fn multi_fade() {
    let mut environment = Environment::new();
    environment.fixtures.create_with_id(1);
    let mut patch = Patch::new();
    let mut dimmer = FixtureProfile::new();
    dimmer.set_parameter(Param::Intensity, Parameter::new(0, 0.0, 100.0));
    patch.patch(1, Address::new(1, 1), &dimmer);

    let mut track = Track::new();

    track.add_action(Time::at(0, 0, 0, 0), fading_action());
    track.add_action(Time::at(0, 0, 2, 0), capture_fading_action());
    track.add_action(Time::at(0, 0, 4, 0), capture_fading_action());

    environment.add_track(track);

    // Time Fuzzing
    environment.run_to_time(Time::at(0, 0, 6, 0), &patch);
    environment.run_to_time(Time::at(0, 0, 1, 0), &patch);
    environment.run_to_time(Time::at(0, 0, 3, 0), &patch);
    environment.run_to_time(Time::at(0, 0, 2, 0), &patch);
    environment.run_to_time(Time::at(0, 0, 6, 0), &patch);

    let resolved = environment.fixtures.resolve(Time::at(0, 0, 6, 0), &patch);

    assert_eq!(
        *resolved
            .get(&1)
            .unwrap()
            .get_value(&Param::Intensity)
            .unwrap(),
        Values::make_literal(87.50)
    )
}

// TODO: This action boilerplate code is only differnetiated by value across
//       many tests, and should be abstracted to a helper method

fn fading_action() -> Action {
    let mut action = Action::new();
    let query = QueryBuilder::new().all().build();
    let value = Fade::new(
        Box::new(Static::new(Values::make_literal(0.0))),
        Box::new(Static::new(Values::make_literal(100.0))),
        Duration::new(4, 0),
    );
    let apply = Apply::new(Param::Intensity, Box::new(value));
    let mut apply_group = ApplyGroup::new(query);
    apply_group.add_apply(apply);
    action.add_group(apply_group);

    action
}

fn capture_fading_action() -> Action {
    let mut action = Action::new();
    let query = QueryBuilder::new().all().build();
    let value = Fade::new(
        Box::new(CurrentValue::new()),
        Box::new(Static::new(Values::make_literal(100.0))),
        Duration::new(4, 0),
    );
    let apply = Apply::new(Param::Intensity, Box::new(value));
    let mut apply_group = ApplyGroup::new(query);
    apply_group.add_apply(apply);
    action.add_group(apply_group);

    action
}

fn capture() -> Action {
    let mut action = Action::new();
    let query = QueryBuilder::new().all().build();
    let value = CurrentValue::new();
    let apply = Apply::new(Param::Intensity, Box::new(value));
    let mut apply_group = ApplyGroup::new(query);
    apply_group.add_apply(apply);
    action.add_group(apply_group);

    action
}
