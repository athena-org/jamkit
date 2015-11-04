use time::{Duration, SteadyTime};
use {Key, KeyState};

pub struct DeterminismTimer {
    elapsed: Duration,
    target: Duration,
    last_tick: SteadyTime
}

impl DeterminismTimer {
    pub fn at_interval(milliseconds: i64) -> Self {
        DeterminismTimer {
            elapsed: Duration::zero(),
            target: Duration::milliseconds(milliseconds),
            last_tick: SteadyTime::now()
        }
    }

    pub fn update(&mut self, tick_closure: &mut FnMut(Duration)) {
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
            lookup: (0..Key::Unknown as usize).map(|_| KeyState::Released).collect()
        }
    }

    pub fn process_keyboard(&mut self, state: &KeyState, key: &Key) {
        if let &Key::Unknown = key { return; }

        self.lookup[key.clone() as usize] = state.clone();
    }

    pub fn get(&self, key: Key) -> KeyState {
        if let Key::Unknown = key {
            KeyState::Released
        } else {
            self.lookup[key as usize].clone()
        }
    }
}
