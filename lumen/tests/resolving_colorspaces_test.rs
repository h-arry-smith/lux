use lumen::{
    action::Apply,
    color::Colorspace,
    fixture::Fixture,
    parameter::{Param, Parameter},
    patch::FixtureProfile,
    timecode::time::Time,
    value::{generator::Static, Values},
};

#[test]
fn resolve_rgb() {
    let profile = rgb_profile();
    let mut fixture = rgb_fixture(10.0, 20.0, 30.0);

    let resolved_fixture = fixture.resolve(&Time::at(0, 0, 0, 0), &profile);

    assert_eq!(
        *resolved_fixture.get_value(&Param::Red).unwrap(),
        Values::make_literal(10.0)
    );
    assert_eq!(
        *resolved_fixture.get_value(&Param::Green).unwrap(),
        Values::make_literal(20.0)
    );
    assert_eq!(
        *resolved_fixture.get_value(&Param::Blue).unwrap(),
        Values::make_literal(30.0)
    );
}

fn rgb_profile() -> FixtureProfile {
    let mut profile = FixtureProfile::new();
    profile.set_colorspace(Colorspace::RGB);
    profile.set_parameter(Param::Red, Parameter::simple(0));
    profile.set_parameter(Param::Blue, Parameter::simple(1));
    profile.set_parameter(Param::Green, Parameter::simple(2));
    profile
}

fn rgb_fixture(r: f64, g: f64, b: f64) -> Fixture {
    let mut fixture = Fixture::new(1);

    fixture.apply(&Apply::new(
        Param::Red,
        Box::new(Static::new(Values::make_literal(r))),
    ));
    fixture.apply(&Apply::new(
        Param::Green,
        Box::new(Static::new(Values::make_literal(g))),
    ));
    fixture.apply(&Apply::new(
        Param::Blue,
        Box::new(Static::new(Values::make_literal(b))),
    ));

    fixture
}
