use std::collections::BTreeMap;

use crate::{
    fixture_set::FixtureSet,
    history::History,
    timecode::time::Time,
    track::{Track, Tracks},
};

// TODO: This struct splitting technique is succesful for dealing with mutable
//       borrows, but shouldn't live in this file, and all these interfaces need
//       tidying up.

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

    pub fn run_to_time(&mut self, time: Time) {
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
                self.fixtures.apply_action(track_action.action());
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
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
