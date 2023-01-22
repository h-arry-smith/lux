use std::time::Duration;

use lumen::{
    action::Apply,
    fixture::Fixture,
    parameter::{Param, Parameter},
    patch::FixtureProfile,
    timecode::time::Time,
    value::{
        generator::{CurrentValue, Delay, Fade, Static},
        Generator, Values,
    },
};

#[test]
fn simple() {
    let mut fixture = Fixture::new(1);

    fixture.apply(&Apply::new(
        Param::Intensity,
        Box::new(Static::new(Values::make_literal(10.0))),
    ));

    fixture.apply(&Apply::new(Param::Intensity, Box::new(CurrentValue::new())));

    let mut profile = FixtureProfile::new();
    profile.set_parameter(Param::Intensity, Parameter::new(0, 0.0, 100.0));

    assert_eq!(
        *fixture
            .resolve(&Time::at(0, 0, 0, 0), &profile)
            .get_value(&Param::Intensity)
            .unwrap(),
        Values::make_literal(10.0)
    )
}

#[test]
fn fade() {
    let mut fixture = Fixture::new(1);

    fixture.apply(&Apply::new(
        Param::Intensity,
        Box::new(Static::new(Values::make_literal(10.0))),
    ));

    let mut fade = Fade::new(
        Box::new(CurrentValue::new()),
        Box::new(Static::new(Values::make_literal(50.0))),
        Duration::new(2, 0),
    );
    fade.set_start_time(Time::at(0, 0, 0, 0));

    fixture.apply(&Apply::new(Param::Intensity, Box::new(fade)));

    let mut profile = FixtureProfile::new();
    profile.set_parameter(Param::Intensity, Parameter::new(0, 0.0, 100.0));

    assert_eq!(
        *fixture
            .resolve(&Time::at(0, 0, 0, 0), &profile)
            .get_value(&Param::Intensity)
            .unwrap(),
        Values::make_literal(10.0)
    );

    assert_eq!(
        *fixture
            .resolve(&Time::at(0, 0, 1, 0), &profile)
            .get_value(&Param::Intensity)
            .unwrap(),
        Values::make_literal(30.0)
    );

    assert_eq!(
        *fixture
            .resolve(&Time::at(0, 0, 2, 0), &profile)
            .get_value(&Param::Intensity)
            .unwrap(),
        Values::make_literal(50.0)
    );
}

#[test]
fn current_value_from_running_fade() {
    let mut fixture = Fixture::new(1);

    fixture.apply(&Apply::new(
        Param::Intensity,
        Box::new(Static::new(Values::make_literal(10.0))),
    ));

    let mut fade = Fade::new(
        Box::new(CurrentValue::new()),
        Box::new(Static::new(Values::make_literal(50.0))),
        Duration::new(2, 0),
    );
    fade.set_start_time(Time::at(0, 0, 0, 0));

    fixture.apply(&Apply::new(Param::Intensity, Box::new(fade)));

    fixture.apply(&Apply::new(
        Param::Intensity,
        Box::new(Delay::new(
            Duration::new(1, 0),
            Box::new(CurrentValue::new()),
        )),
    ));

    let mut profile = FixtureProfile::new();
    profile.set_parameter(Param::Intensity, Parameter::new(0, 0.0, 100.0));

    assert_eq!(
        *fixture
            .resolve(&Time::at(0, 0, 0, 0), &profile)
            .get_value(&Param::Intensity)
            .unwrap(),
        Values::make_literal(10.0)
    );

    assert_eq!(
        *fixture
            .resolve(&Time::at(0, 0, 1, 0), &profile)
            .get_value(&Param::Intensity)
            .unwrap(),
        Values::make_literal(30.0)
    );

    assert_eq!(
        *fixture
            .resolve(&Time::at(0, 0, 2, 0), &profile)
            .get_value(&Param::Intensity)
            .unwrap(),
        Values::make_literal(30.0)
    );
}

// TODO: a running fade with a current value referenced fade
// TODO: Current Value from the previous Action
