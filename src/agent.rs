use crate::*;

#[derive(Default)]
pub struct Agent {
    state_value_function: HashMap<(u32, u32), f32>,

    last_state: Option<(u32, u32)>,
    last_action: Option<Action>,

    random: ThreadRng,
}

impl Agent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn select_action(&mut self, state: (u32, u32), actions: &[Action]) -> Action {
        let action = *actions.choose(&mut self.random).unwrap();

        self.last_state = Some(state);
        self.last_action = Some(action);

        action
    }

    pub fn receive_reward(&mut self, _reward: f32) {
        // TODO
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_selects_a_random_action_from_those_available() {
        let mut agent = Agent::new();

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
    fn it_sets_last_state_and_action_after_selecting_one() {
        let mut agent = Agent::new();

        agent.select_action((1, 2), &[Action::North]);

        assert_eq!(agent.last_state, Some((1, 2)));
        assert_eq!(agent.last_action, Some(Action::North));
    }
}
