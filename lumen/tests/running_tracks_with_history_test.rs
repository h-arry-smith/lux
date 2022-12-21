use lumen::fixture::FixtureID;
use lumen::Environment;
use lumen::{
    action::{Action, Apply, ApplyGroup},
    parameter::Param,
    value::{generator::Static, Values},
    QueryBuilder,
};

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

mod single_track_moving_backwards {
    use lumen::{parameter::Param, timecode::time::Time, track::Track, value::Values};

    use crate::{action, action_for, build_environment};

    // PLAYHEAD     >    *
    //    TRACK ----O-----
    //     TIME     1    0
    //  HISTORY     1    0
    #[test]
    fn one_action_at_t1_go_back_to_start_initial_state_restored() {
        let mut environment = build_environment(1);
        let mut track = Track::new();
        track.add_action(Time::at(0, 0, 1, 0), action(10.0));
        environment.add_track(track);

        environment.run_to_time(Time::at(0, 0, 1, 0));
        environment.run_to_time(Time::at(0, 0, 0, 0));

        assert_eq!(environment.history.len(), 0);
        assert!(environment
            .fixtures
            .get(&1)
            .unwrap()
            .param(Param::Intensity)
            .is_none())
    }

    // PLAYHEAD     >    >    *
    //    TRACK ----O----O-----
    //     TIME     1    3    2
    //  HISTORY     1    2    1
    #[test]
    fn one_action_at_t1_one_at_t3_go_back_t2_and_check_state_is_h1() {
        let mut environment = build_environment(1);
        let mut track = Track::new();
        track.add_action(Time::at(0, 0, 1, 0), action(10.0));
        track.add_action(Time::at(0, 0, 3, 0), action(30.0));
        environment.add_track(track);

        environment.run_to_time(Time::at(0, 0, 3, 0));
        environment.run_to_time(Time::at(0, 0, 2, 0));

        dbg!(&environment.history);

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

    // PLAYHEAD >    >    *
    //    TRACK O----O-----
    //     TIME 0    1    0
    //  HISTORY 1    2    1
    #[test]
    fn one_action_at_t0_one_action_at_t1_go_back_to_t0_and_no_h2_remains() {
        let mut environment = build_environment(2);
        let mut track = Track::new();
        track.add_action(Time::at(0, 0, 0, 0), action(10.0));
        track.add_action(Time::at(0, 0, 1, 0), action_for(20.0, 2));
        environment.add_track(track);

        environment.run_to_time(Time::at(0, 0, 1, 0));
        environment.run_to_time(Time::at(0, 0, 0, 0));

        assert_eq!(environment.history.len(), 1);
        assert_eq!(
            environment
                .fixtures
                .get(&2)
                .unwrap()
                .param(Param::Intensity)
                .unwrap()
                .value(),
            Values::make_literal(10.0)
        )
    }
}

mod multi_track_moving_forward {
    use lumen::{timecode::time::Time, track::Track};

    use crate::{action, build_environment};

    // PLAYHEAD     *
    //  TRACK 1 ----O----
    //  TRACK 2 ----O----
    //     TIME     1
    //  HISTORY     1
    #[test]
    fn two_track_with_one_action_at_t1_create_one_history() {
        let mut environment = build_environment(1);
        let mut track1 = Track::new();
        let mut track2 = Track::new();
        track1.add_action(Time::at(0, 0, 1, 0), action(10.0));
        track2.add_action(Time::at(0, 0, 1, 0), action(20.0));
        environment.add_track(track1);
        environment.add_track(track2);

        environment.run_to_time(Time::at(0, 0, 1, 0));

        assert_eq!(environment.history.len(), 1);
    }

    // PLAYHEAD     *
    //  TRACK 1 ----O----
    //          ----O----
    //  TRACK 2 ----O----
    //          ----O----
    //     TIME     1
    //  HISTORY     1
    #[test]
    fn two_track_with_two_actions_at_t1_create_one_history() {
        let mut environment = build_environment(1);
        let mut track1 = Track::new();
        let mut track2 = Track::new();
        track1.add_action(Time::at(0, 0, 1, 0), action(10.0));
        track1.add_action(Time::at(0, 0, 1, 0), action(20.0));
        track2.add_action(Time::at(0, 0, 1, 0), action(30.0));
        track2.add_action(Time::at(0, 0, 1, 0), action(40.0));
        environment.add_track(track1);
        environment.add_track(track2);

        environment.run_to_time(Time::at(0, 0, 1, 0));

        assert_eq!(environment.history.len(), 1);
    }

    // PLAYHEAD          *
    //  TRACK 1 ----O----O----
    //          ----O----O----
    //  TRACK 2 ----O----O----
    //          ----O----O----
    //     TIME     1    2
    //  HISTORY     1    2
    #[test]
    fn two_track_with_two_actions_at_t1_and_two_actions_at_t2_create_two_history() {
        let mut environment = build_environment(1);
        let mut track1 = Track::new();
        let mut track2 = Track::new();
        track1.add_action(Time::at(0, 0, 1, 0), action(10.0));
        track1.add_action(Time::at(0, 0, 1, 0), action(20.0));
        track2.add_action(Time::at(0, 0, 1, 0), action(30.0));
        track2.add_action(Time::at(0, 0, 1, 0), action(40.0));
        track1.add_action(Time::at(0, 0, 2, 0), action(10.0));
        track1.add_action(Time::at(0, 0, 2, 0), action(20.0));
        track2.add_action(Time::at(0, 0, 2, 0), action(30.0));
        track2.add_action(Time::at(0, 0, 2, 0), action(40.0));
        environment.add_track(track1);
        environment.add_track(track2);

        environment.run_to_time(Time::at(0, 0, 2, 0));

        assert_eq!(environment.history.len(), 2);
    }

    // PLAYHEAD     >    *
    //  TRACK 1 ----O----O----
    //          ----O----O----
    //  TRACK 2 ----O----O----
    //          ----O----O----
    //     TIME     1    2
    //  HISTORY     1    2
    #[test]
    fn two_track_with_two_actions_to_t1_and_two_actions_to_t2_create_two_history() {
        let mut environment = build_environment(1);
        let mut track1 = Track::new();
        let mut track2 = Track::new();
        track1.add_action(Time::at(0, 0, 1, 0), action(10.0));
        track1.add_action(Time::at(0, 0, 1, 0), action(20.0));
        track2.add_action(Time::at(0, 0, 1, 0), action(30.0));
        track2.add_action(Time::at(0, 0, 1, 0), action(40.0));
        track1.add_action(Time::at(0, 0, 2, 0), action(10.0));
        track1.add_action(Time::at(0, 0, 2, 0), action(20.0));
        track2.add_action(Time::at(0, 0, 2, 0), action(30.0));
        track2.add_action(Time::at(0, 0, 2, 0), action(40.0));
        environment.add_track(track1);
        environment.add_track(track2);

        environment.run_to_time(Time::at(0, 0, 1, 0));
        environment.run_to_time(Time::at(0, 0, 2, 0));

        assert_eq!(environment.history.len(), 2);
    }
}

// TIME MOVES BACKWARD - MULTI TRACK
mod multi_track_moving_backward {
    use lumen::{parameter::Param, timecode::time::Time, track::Track, value::Values};

    use crate::{action, build_environment};

    // PLAYHEAD     >    *
    //  TRACK 1 ----0-----
    //  TRACK 2 ----0-----
    //     TIME     1    0
    //  HISTORY     1    0
    #[test]
    fn two_tracks_one_action_at_t1_go_to_end_and_then_return_to_start() {
        let mut environment = build_environment(1);
        let mut track1 = Track::new();
        let mut track2 = Track::new();
        track1.add_action(Time::at(0, 0, 1, 0), action(10.0));
        track2.add_action(Time::at(0, 0, 1, 0), action(20.0));
        environment.add_track(track1);
        environment.add_track(track2);

        environment.run_to_time(Time::at(0, 0, 1, 0));
        environment.run_to_time(Time::at(0, 0, 0, 0));

        assert!(environment
            .fixtures
            .get(&1)
            .unwrap()
            .param(Param::Intensity)
            .is_none())
    }

    // PLAYHEAD          >    *
    //  TRACK 1 ----0----0-----
    //          ----0----0-----
    //  TRACK 2 ----0----0-----
    //          ----0----0-----
    //     TIME     1    3    2
    //  HISTORY     1    2    1
    #[test]
    fn two_tracks_with_actions_at_t1_and_t3_return_to_t2() {
        let mut environment = build_environment(1);
        let mut track1 = Track::new();
        let mut track2 = Track::new();
        track1.add_action(Time::at(0, 0, 1, 0), action(10.0));
        track1.add_action(Time::at(0, 0, 1, 0), action(10.0));
        track2.add_action(Time::at(0, 0, 1, 0), action(20.0));
        track2.add_action(Time::at(0, 0, 1, 0), action(20.0));
        track1.add_action(Time::at(0, 0, 3, 0), action(30.0));
        track1.add_action(Time::at(0, 0, 3, 0), action(30.0));
        track2.add_action(Time::at(0, 0, 3, 0), action(40.0));
        track2.add_action(Time::at(0, 0, 3, 0), action(40.0));
        environment.add_track(track1);
        environment.add_track(track2);

        environment.run_to_time(Time::at(0, 0, 3, 0));
        environment.run_to_time(Time::at(0, 0, 2, 0));

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

    // Run three tracks, with multiple actions all at different times, return to the middle and check history
    // PLAYHEAD                              >    *
    //  TRACK 1 ----0----0----0----0----0---------0
    //  TRACK 2 ---------0----0-------------------0
    //  TRACK 3 ---------0---------0---------0----0
    //     TIME     1    2    3    4    5    6    3.5
    //  HISTORY     1    2    3    4    5    6    3
    #[test]
    fn multiple_actions_lots_of_actions() {
        let mut environment = build_environment(1);
        let mut track1 = Track::new();
        let mut track2 = Track::new();
        let mut track3 = Track::new();

        track1.add_action(Time::at(0, 0, 1, 0), action(10.0));

        track1.add_action(Time::at(0, 0, 2, 0), action(20.0));
        track2.add_action(Time::at(0, 0, 2, 0), action(20.0));
        track3.add_action(Time::at(0, 0, 2, 0), action(20.0));

        track1.add_action(Time::at(0, 0, 3, 0), action(30.0));
        track2.add_action(Time::at(0, 0, 3, 0), action(30.0));

        track1.add_action(Time::at(0, 0, 4, 0), action(40.0));
        track3.add_action(Time::at(0, 0, 4, 0), action(40.0));

        track1.add_action(Time::at(0, 0, 5, 0), action(50.0));

        track3.add_action(Time::at(0, 0, 6, 0), action(60.0));

        environment.add_track(track1);
        environment.add_track(track2);
        environment.add_track(track3);

        environment.run_to_time(Time::at(0, 0, 6, 0));
        environment.run_to_time(Time::at(0, 0, 3, 500));

        assert_eq!(
            environment
                .fixtures
                .get(&1)
                .unwrap()
                .param(Param::Intensity)
                .unwrap()
                .value(),
            Values::make_literal(30.0)
        )
    }
}

// TIME MOVES BACKWARD, AND THEN FORWARD AGAIN
// Run two tracks, with two action at T1 and T3, run all the way, return to T2,
// then all the way and check state is fine.

mod time_moving_both_directions {
    use lumen::{parameter::Param, timecode::time::Time, track::Track, value::Values};

    use crate::{action, build_environment};

    // PLAYHEAD
    //  TRACK 1 ----0----0---------0
    //  TRACK 2 ----0----0---------0
    //     TIME     1    3    2    3
    //  HISTORY     1    2    1    2
    #[test]
    fn two_tracks_two_directions() {
        let mut environment = build_environment(1);
        let mut track1 = Track::new();
        let mut track2 = Track::new();
        track1.add_action(Time::at(0, 0, 1, 0), action(10.0));
        track2.add_action(Time::at(0, 0, 1, 0), action(20.0));
        track1.add_action(Time::at(0, 0, 3, 0), action(30.0));
        track2.add_action(Time::at(0, 0, 3, 0), action(40.0));
        environment.add_track(track1);
        environment.add_track(track2);

        environment.run_to_time(Time::at(0, 0, 3, 0));
        environment.run_to_time(Time::at(0, 0, 2, 0));
        environment.run_to_time(Time::at(0, 0, 3, 0));
        environment.run_to_time(Time::at(0, 0, 1, 0));
        environment.run_to_time(Time::at(0, 0, 3, 0));

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
}

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
#[cfg(test)]
fn action_for(value: f32, id: FixtureID) -> Action {
    let mut action = Action::new();
    let query = QueryBuilder::new().id(id).build();
    let value = Static::new(Values::make_literal(value));
    let apply = Apply::new(Param::Intensity, Box::new(value));
    let mut apply_group = ApplyGroup::new(query);
    apply_group.add_apply(apply);
    action.add_group(apply_group);

    action
}
