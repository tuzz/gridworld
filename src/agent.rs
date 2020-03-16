use crate::*;

#[derive(Default)]
pub struct Agent {
    pub state_value_function: HashMap<(u32, u32), f32>,
    pub discount_factor: f32,

    pub last_state: Option<(u32, u32)>,

    pub random: ThreadRng,
}

impl Agent {
    pub fn new(discount_factor: f32) -> Self {
        Self { discount_factor, ..Self::default() }
    }

    pub fn select_action(&mut self, state: (u32, u32), actions: &[Action]) -> Action {
        let action = *actions.choose(&mut self.random).unwrap();

        self.last_state = Some(state);

        action
    }

    pub fn receive_reward(&mut self, reward: f32, new_state: (u32, u32)) {
        let prev_state = self.last_state.unwrap();

        let prev_value = *self.state_value_function.entry(prev_state).or_insert(0.);
        let this_value = *self.state_value_function.entry(new_state).or_insert(0.);

        let new_value = reward + self.discount_factor * this_value;

        if new_value > prev_value {
            self.state_value_function.insert(prev_state, new_value);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_selects_a_random_action_from_those_available() {
        let mut agent = Agent::new(1.);

        for _ in 0..100 {
            let action = agent.select_action((0, 0), &[Action::North]);
            assert_eq!(action, Action::North);

            let action = agent.select_action((0, 0), &[Action::South]);
            assert_eq!(action, Action::South);

            let action = agent.select_action((0, 0), &[Action::North, Action::South]);
            assert!(action == Action::North || action == Action::South);
        }
    }

    #[test]
    fn it_sets_last_state_after_selecting_one() {
        let mut agent = Agent::new(1.);

        agent.select_action((1, 2), &[Action::North]);

        assert_eq!(agent.last_state, Some((1, 2)));
    }
}
