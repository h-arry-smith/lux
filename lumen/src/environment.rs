use std::collections::BTreeMap;

use crate::{
    fixture_set::FixtureSet,
    history::History,
    timecode::time::Time,
    track::{Track, Tracks},
    Patch,
};

// TODO: This struct splitting technique is succesful for dealing with mutable
//       borrows, but shouldn't live in this file, and all these interfaces need
//       tidying up.

#[derive(Clone)]
pub struct Environment {
    pub fixtures: FixtureSet,
    pub history: History,
    tracks: Tracks,
    last_time: Option<Time>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            fixtures: FixtureSet::new(),
            history: History::new(),
            tracks: Tracks::new(),
            last_time: None,
        }
    }

    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track)
    }

    pub fn reset(&mut self) {
        self.fixtures = self.fixtures.clean_clone();
        self.history.clear();
        self.tracks.clear();
        self.last_time = None;
        self.revert_to_time(Time::at(0, 0, 0, 0));
    }

    pub fn run_to_time(&mut self, time: Time, patch: &Patch) {
        if let Some(last_time) = self.last_time {
            if time < last_time {
                self.revert_to_time(time);
            }
        }

        self.last_time = Some(time);

        let mut all_unrun_actions = BTreeMap::new();
        // for each active track
        for track in self.tracks.active() {
            // get all the unrun actions and merge them into time groups
            all_unrun_actions.extend(track.unrun_actions_at_time(time));
        }

        let mut histories = Vec::new();

        // apply the actions at each time generating a history
        for (time_frame, track_actions) in all_unrun_actions.into_iter() {
            let history_id = self.history.record(self.fixtures.clone());

            // collect a history id for each time group
            histories.push((time_frame, history_id));

            for track_action in track_actions {
                self.fixtures
                    .apply_action(track_action.action(), time_frame, patch);
            }
        }

        // apply those history id's to those time groups in all the active tracks
        for (time_frame, history_id) in histories {
            for track in self.tracks.active_mut() {
                track.set_action_history_for_time(time_frame, history_id);
            }
        }
    }

    fn revert_to_time(&mut self, time: Time) {
        if time.is_zero() {
            let default_fixture_state = self.fixtures.clean_clone();
            self.fixtures = default_fixture_state;
            self.history.clear();

            for track in self.tracks.active_mut() {
                track.clear_history();
            }

            return;
        }

        let mut closest_track_actions = Vec::new();
        for track in self.tracks.active() {
            if let Some(track_action) = track.get_closest_action_to_time_with_history(time) {
                closest_track_actions.push(track_action);
            }
        }

        let closest_track_action = closest_track_actions.iter().reduce(|closest, action| {
            if closest.time() >= action.time() {
                closest
            } else {
                action
            }
        });

        match closest_track_action {
            Some(track_action) => {
                self.fixtures = self
                    .history
                    .revert(track_action.history())
                    .expect("Tried to go to an invalid history ID");

                let reset_time = *track_action.time();

                for track in self.tracks.active_mut() {
                    track.clear_history_after_time(reset_time);
                }
            }
            None => {
                // If there isn't a history to revert to, we can revert to the
                // base state
                self.revert_to_time(Time::at(0, 0, 0, 0));
            }
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
