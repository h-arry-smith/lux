use lumen::Environment;
use lumen::{
    action::{Action, Apply, ApplyGroup},
    parameter::Param,
    value::{generator::Static, Values},
    QueryBuilder,
};

// TIME MOVING FORWARD - SINGLE TRACK
mod single_track_moving_forward {
    use lumen::{parameter::Param, timecode::time::Time, track::Track, value::Values};

    use crate::{action, build_environment};

    // PLAYHEAD     *
    //    TRACK ----O----
    //     TIME     1
    //  HISTORY     1
    #[test]
    fn one_action_at_t1_generates_one_history() {
        let mut environment = build_environment(1);
        let mut track = Track::new();
        track.add_action(Time::at(0, 0, 1, 0), action(10.0));
        environment.add_track(track);

        environment.run_to_time(Time::at(0, 0, 1, 0));

        assert_eq!(environment.history.len(), 1);
        assert_eq!(
            environment
                .fixtures
                .get(&1)
                .unwrap()
                .param(Param::Intensity)
                .unwrap()
                .value(),
            Values::make_literal(10.0)
        )
    }

    // PLAYHEAD          *
    //    TRACK ----O----O----
    //     TIME     1    2
    //  HISTORY     1    2
    #[test]
    fn two_action_at_t1_generates_one_history() {
        let mut environment = build_environment(1);
        let mut track = Track::new();
        track.add_action(Time::at(0, 0, 1, 0), action(10.0));
        track.add_action(Time::at(0, 0, 1, 0), action(20.0));
        environment.add_track(track);

        environment.run_to_time(Time::at(0, 0, 1, 0));

        assert_eq!(environment.history.len(), 1);
        assert_eq!(
            environment
                .fixtures
                .get(&1)
                .unwrap()
                .param(Param::Intensity)
                .unwrap()
                .value(),
            Values::make_literal(20.0)
        )
    }

    // PLAYHEAD          *
    //    TRACK ----O----O----
    //          ----O----O----
    //     TIME     1    2
    //  HISTORY     1    2
    #[test]
    fn two_action_at_t1_and_two_action_at_t2_generates_two_history() {
        let mut environment = build_environment(1);
        let mut track = Track::new();
        track.add_action(Time::at(0, 0, 1, 0), action(10.0));
        track.add_action(Time::at(0, 0, 1, 0), action(20.0));
        track.add_action(Time::at(0, 0, 2, 0), action(30.0));
        track.add_action(Time::at(0, 0, 2, 0), action(40.0));
        environment.add_track(track);

        environment.run_to_time(Time::at(0, 0, 2, 0));

        assert_eq!(environment.history.len(), 2);
        assert_eq!(
            environment
                .fixtures
                .get(&1)
                .unwrap()
                .param(Param::Intensity)
                .unwrap()
                .value(),
            Values::make_literal(40.0)
        )
    }

    // PLAYHEAD     >    *
    //    TRACK ----O----O----
    //     TIME     1    2
    //  HISTORY     1    2
    #[test]
    fn one_action_at_t1_the_one_action_at_t2_generates_two_history() {
        let mut environment = build_environment(1);
        let mut track = Track::new();
        track.add_action(Time::at(0, 0, 1, 0), action(10.0));
        track.add_action(Time::at(0, 0, 2, 0), action(20.0));
        environment.add_track(track);

        environment.run_to_time(Time::at(0, 0, 1, 0));
        environment.run_to_time(Time::at(0, 0, 2, 0));

        assert_eq!(environment.history.len(), 2);
        assert_eq!(
            environment
                .fixtures
                .get(&1)
                .unwrap()
                .param(Param::Intensity)
                .unwrap()
                .value(),
            Values::make_literal(20.0)
        )
    }
}

// TIME MOVES BACKWARDS - SINGLE TRACK
// Run one action at T1, go back to start, return to initial state
// Run one action at T1, one at T3, go to T2 and check state is H1

// TIME MOVES FORWARD - MULTI TRACK
// Run two seperate track with 1 action at T1, and generate one history
// Run two seperate track with 2 action at T2, and generate one history
// Run two seperate track with two sets of action, and generate two history
// Run two tracks to T1, and then to T2, and generate two history

// TIME MOVES BACKWARD - MULTI TRACK
// Run two tracks with one action at T1, go back to start, and return to initial state
// Run two tracks, with two action at T1, two action at T3, go to T2 and check state is H1
// Run three tracks, with multiple actions all at different times, return to the middle and check history

// TIME MOFES BACKWARD, AND THEN FORWARD AGAIN
// Run two tracks, with two action at T1 and T3, run all the way, return to T2,
// then all the way and check state is fine.

#[cfg(test)]
fn build_environment(n_fixtures: usize) -> Environment {
    let mut environment = Environment::new();
    for n in 1..=n_fixtures {
        environment.fixtures.create_with_id(n);
    }

    environment
}

#[cfg(test)]
fn action(value: f32) -> Action {
    let mut action = Action::new();
    let query = QueryBuilder::new().all().build();
    let value = Static::new(Values::make_literal(value));
    let apply = Apply::new(Param::Intensity, Box::new(value));
    let mut apply_group = ApplyGroup::new(query);
    apply_group.add_apply(apply);
    action.add_group(apply_group);

    action
}