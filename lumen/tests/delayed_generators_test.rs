use std::time::Duration;

use lumen::{
    action::Apply,
    fixture::Fixture,
    parameter::{Param, Parameter},
    patch::FixtureProfile,
    timecode::time::Time,
    value::{
        generator::{Delay, Static},
        Values,
    },
};

// TODO: Some macro's could tidy up this code a lot

#[test]
fn multiple_delayed_static_values() {
    let mut fixture = Fixture::new(1);

    fixture.apply(&Apply::new(
        Param::Intensity,
        Box::new(Static::new(Values::make_literal(0.0))),
    ));

    fixture.apply(&Apply::new(
        Param::Intensity,
        Box::new(Delay::new(
            Duration::new(1, 0),
            Box::new(Static::new(Values::make_literal(10.0))),
        )),
    ));

    fixture.apply(&Apply::new(
        Param::Intensity,
        Box::new(Delay::new(
            Duration::new(2, 0),
            Box::new(Static::new(Values::make_literal(20.0))),
        )),
    ));

    let mut profile = FixtureProfile::new();
    profile.set_parameter(Param::Intensity, Parameter::new(0, 0.0, 100.0));

    assert_eq!(
        *fixture
            .resolve(&Time::at(0, 0, 0, 0), &profile)
            .get_value(&Param::Intensity)
            .unwrap(),
        Values::make_literal(0.0)
    );

    assert_eq!(
        *fixture
            .resolve(&Time::at(0, 0, 1, 0), &profile)
            .get_value(&Param::Intensity)
            .unwrap(),
        Values::make_literal(10.0)
    );

    assert_eq!(
        *fixture
            .resolve(&Time::at(0, 0, 2, 0), &profile)
            .get_value(&Param::Intensity)
            .unwrap(),
        Values::make_literal(20.0)
    );
}

#[test]
fn latest_takes_priority() {
    let mut fixture = Fixture::new(1);

    // Delays are out of order in the value stack

    fixture.apply(&Apply::new(
        Param::Intensity,
        Box::new(Delay::new(
            Duration::new(2, 0),
            Box::new(Static::new(Values::make_literal(20.0))),
        )),
    ));

    fixture.apply(&Apply::new(
        Param::Intensity,
        Box::new(Delay::new(
            Duration::new(1, 0),
            Box::new(Static::new(Values::make_literal(10.0))),
        )),
    ));

    let mut profile = FixtureProfile::new();
    profile.set_parameter(Param::Intensity, Parameter::new(0, 0.0, 100.0));

    assert_eq!(
        *fixture
            .resolve(&Time::at(0, 0, 1, 0), &profile)
            .get_value(&Param::Intensity)
            .unwrap(),
        Values::make_literal(10.0)
    );

    assert_eq!(
        *fixture
            .resolve(&Time::at(0, 0, 2, 0), &profile)
            .get_value(&Param::Intensity)
            .unwrap(),
        Values::make_literal(20.0)
    );
}

// TODO: Test to confirm a delayed fade generates the correct values, respecting
//       it's offset.
