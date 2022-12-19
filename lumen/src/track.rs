use crate::{action::Action, timecode::time::Time, Environment};

#[derive(Debug)]
pub struct Track {
    actions: Vec<TrackAction>,
    offset: Time,
    last_time: Option<Time>,
}

impl Track {
    pub fn new(offset: Time) -> Self {
        Self {
            actions: Vec::new(),
            offset,
            last_time: None,
        }
    }

    pub fn add_action(&mut self, time: Time, action: Action) {
        self.actions.push(TrackAction::new(time, action));
        self.actions.sort();
    }

    pub fn apply_actions(&mut self, current_time: Time, environment: &mut Environment) {
        if current_time < self.offset {
            return;
        }

        let current_time = current_time - self.offset;

        match self.last_time {
            Some(last_time) => {
                if current_time >= last_time {
                    // time has progressed lineraly, so just apply unapplied actions
                    self.apply_unapplied_actions_to_time(current_time, environment);
                    return;
                }
            }
            None => {
                // never seen a time before, so set and continue assuming we could
                // be anywhere
                self.last_time = Some(current_time);
            }
        }

        for track_action in self.actions_after_time(current_time) {
            track_action.clear_history()
        }

        // Given the sorted actions by time, find the most recent actions to the
        // current time with a history
        match self.most_recent_history_to_time(current_time) {
            Some(history_id) => {
                // and revert the environment to the earliest history in the set.
                environment.revert(history_id)
            }
            None => {
                // If not found, get the first action, run that, mark it with a history
                // index
                if let Some(track_action) = self.actions.first_mut() {
                    let history_id =
                        environment.generate_history_and_run_action(&track_action.action);
                    track_action.set_history(history_id);
                }
                return;
            }
        }

        // get any unapplied actions till the time, and apply them.
        self.apply_unapplied_actions_to_time(current_time, environment);
    }

    fn apply_unapplied_actions_to_time(
        &mut self,
        current_time: Time,
        environment: &mut Environment,
    ) {
        for track_action in self
            .actions
            .iter_mut()
            .filter(|action| !action.has_history() && action.time <= current_time)
        {
            let id = environment.generate_history();
            environment.run_action(&track_action.action);
            track_action.set_history(id);
        }
    }

    fn most_recent_history_to_time(&self, time: Time) -> Option<usize> {
        // find the latest history that has been applied at this time and return
        // it
        let mut history = None;
        for action in self.actions.iter() {
            if action.has_history() && action.time <= time {
                history = Some(action.history())
            }
        }
        history
    }

    fn actions_after_time(&mut self, time: Time) -> impl Iterator<Item = &mut TrackAction> {
        self.actions
            .iter_mut()
            .filter(move |action| action.time > time)
    }
}

#[derive(Debug)]
pub struct TrackAction {
    time: Time,
    action: Action,
    history: Option<usize>,
}

impl TrackAction {
    fn new(time: Time, action: Action) -> Self {
        Self {
            time,
            action,
            history: None,
        }
    }

    fn has_history(&self) -> bool {
        self.history.is_some()
    }

    fn clear_history(&mut self) {
        self.history = None
    }

    fn history(&self) -> usize {
        self.history.unwrap()
    }

    fn set_history(&mut self, history_id: usize) {
        self.history = Some(history_id)
    }
}

impl PartialEq for TrackAction {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for TrackAction {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialOrd for TrackAction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TrackAction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time)
    }
}
