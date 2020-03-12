use crate::*;

#[derive(Default)]
pub struct Trajectory {
    pub states: Vec<(u32, u32)>,
    pub actions: Vec<Action>,
    pub rewards: Vec<f32>,
}

impl Trajectory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, state: (u32, u32), action: Action, reward: f32) {
        self.states.push(state);
        self.actions.push(action);
        self.rewards.push(reward);
    }
}
