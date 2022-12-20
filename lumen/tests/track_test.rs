use lumen::{
    action::{Action, Apply, ApplyGroup},
    parameter::Param,
    value::{generator::Static, Values},
    QueryBuilder,
};

mod unrun_actions_at_time {
    use lumen::{timecode::time::Time, track::Track};

    use crate::create_example_action;

    #[test]
    fn get_single_unrun_track_action_at_time() {
        let action = create_example_action();

        let mut track = Track::new();
        track.add_action(Time::at(0, 0, 1, 0), action.clone());
        track.add_action(Time::at(0, 0, 3, 0), action);

        let actions = track.unrun_actions_at_time(Time::at(0, 0, 2, 0));

        assert_eq!(actions.get(&Time::at(0, 0, 1, 0)).unwrap().len(), 1);
    }

    #[test]
    fn get_multiple_unrun_track_actions_at_single_time() {
        let action = create_example_action();

        let mut track = Track::new();
        track.add_action(Time::at(0, 0, 1, 0), action.clone());
        track.add_action(Time::at(0, 0, 1, 0), action);

        let actions = track.unrun_actions_at_time(Time::at(0, 0, 2, 0));

        assert_eq!(actions.get(&Time::at(0, 0, 1, 0)).unwrap().len(), 2);
    }

    #[test]
    fn get_multiple_unrun_track_actions_at_multiple_times() {
        let action = create_example_action();

        let mut track = Track::new();
        track.add_action(Time::at(0, 0, 1, 0), action.clone());
        track.add_action(Time::at(0, 0, 1, 0), action.clone());
        track.add_action(Time::at(0, 0, 2, 0), action.clone());
        track.add_action(Time::at(0, 0, 2, 0), action.clone());
        track.add_action(Time::at(0, 0, 4, 0), action.clone());
        track.add_action(Time::at(0, 0, 4, 0), action);

        let actions = track.unrun_actions_at_time(Time::at(0, 0, 2, 0));

        assert_eq!(actions.get(&Time::at(0, 0, 1, 0)).unwrap().len(), 2);
        assert_eq!(actions.get(&Time::at(0, 0, 2, 0)).unwrap().len(), 2);
    }
}

#[cfg(test)]
fn create_example_action() -> Action {
    let mut action = Action::new();
    let query = QueryBuilder::new().all().build();
    let value = Static::new(Values::make_literal(25.0));
    let apply = Apply::new(Param::Intensity, Box::new(value));
    let mut apply_group = ApplyGroup::new(query);
    apply_group.add_apply(apply);
    action.add_group(apply_group);

    action
}
