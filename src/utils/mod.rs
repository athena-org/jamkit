use time::{Duration, SteadyTime};
use {Key, KeyState};
use std::ops::Index;

pub struct TickTimer {
    elapsed: Duration,
    target: Duration,
    last_tick: SteadyTime
}

impl TickTimer {
    pub fn at_interval(milliseconds: i64) -> Self {
        TickTimer {
            elapsed: Duration::zero(),
            target: Duration::milliseconds(milliseconds),
            last_tick: SteadyTime::now()
        }
    }

    pub fn update<F: FnMut(Duration)>(&mut self, mut tick_closure: F) {
        let new_tick = SteadyTime::now();
        self.elapsed = self.elapsed + (new_tick - self.last_tick);
        self.last_tick = new_tick;

        while self.elapsed > self.target {
            self.elapsed = self.elapsed - self.target;
            tick_closure(self.target.clone());
        }
    }
}

pub struct InputState {
    lookup: Vec<KeyState>
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            lookup: vec![KeyState::Released; Key::Unknown as usize + 1],
        }
    }

    pub fn process_keyboard(&mut self, state: KeyState, key: Key) {
        if let Key::Unknown = key { return; }

        self.lookup[key as usize] = state;
    }
}

impl Index<Key> for InputState {
    type Output = KeyState;
    fn index(&self, key: Key) -> &KeyState {
        &self.lookup[key as usize]
    }
}
