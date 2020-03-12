use crate::*;

#[derive(Debug)]
pub struct Environment {
    pub grid_width: u32,
    pub grid_height: u32,
    pub current_state: (u32, u32),
    pub special_states: Vec<SpecialState>,
}

#[derive(Debug)]
pub struct SpecialState {
    pub coordinate: (u32, u32),
    pub transitions_to: (u32, u32),
    pub reward: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action { North, South, East, West }

impl Environment {
    pub fn new(grid_width: u32, grid_height: u32, start_state: (u32, u32), special_states: Vec<SpecialState>) -> Self {
        Self { grid_width, grid_height, current_state: start_state, special_states }
    }

    pub fn available_actions(&self, _state: (u32, u32)) -> Vec<Action> {
        vec![Action::North, Action::South, Action::East, Action::West]
    }

    pub fn take_action(&mut self, action: Action) -> f32 {
        let (mut x, mut y) = self.current_state;
        let mut reward = 0.;

        let special_state = self.special_states.iter()
            .find(|s| s.coordinate == self.current_state);

        if let Some(s) = special_state {
            self.current_state = s.transitions_to;
            return s.reward;
        }

        match action {
            Action::North => y = y.saturating_sub(1),
            Action::South => y = min(y + 1, self.grid_height - 1),
            Action::East  => x = min(x + 1, self.grid_width - 1),
            Action::West  => x = x.saturating_sub(1),
        }

        if (x, y) == self.current_state {
            reward = -1.;
        } else {
            self.current_state = (x, y);
        }

        reward
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_a_vec_of_available_actions_for_a_given_state() {
        let environment = Environment::new(5, 5, (0, 0), vec![]);
        let actions = environment.available_actions((0, 0));

        assert_eq!(actions.len(), 4);
    }

    #[test]
    fn it_transitions_to_a_new_state_after_taking_an_action() {
        let mut environment = Environment::new(5, 5, (0, 0), vec![]);

        environment.take_action(Action::East);
        assert_eq!(environment.current_state, (1, 0));

        environment.take_action(Action::East);
        assert_eq!(environment.current_state, (2, 0));

        environment.take_action(Action::North);
        assert_eq!(environment.current_state, (2, 0));

        environment.take_action(Action::South);
        assert_eq!(environment.current_state, (2, 1));

        environment.take_action(Action::West);
        assert_eq!(environment.current_state, (1, 1));
    }

    #[test]
    fn it_returns_a_reward_of_minus_one_for_actions_that_take_the_agent_off_the_grid() {
        let mut environment = Environment::new(5, 5, (0, 0), vec![]);

        assert_eq!(environment.take_action(Action::North), -1.);
        assert_eq!(environment.take_action(Action::West), -1.);

        for _ in 0..4 { environment.take_action(Action::South); }
        assert_eq!(environment.take_action(Action::South), -1.);

        for _ in 0..4 { environment.take_action(Action::East); }
        assert_eq!(environment.take_action(Action::East), -1.);
    }

    #[test]
    fn it_returns_positive_rewards_for_transitions_from_special_states() {
        let mut environment = Environment::new(5, 5, (1, 0), vec![
          SpecialState { coordinate: (1, 0), transitions_to: (1, 4), reward: 10. },
          SpecialState { coordinate: (3, 0), transitions_to: (3, 2), reward: 5. },
        ]);

        assert_eq!(environment.take_action(Action::East), 10.);
        assert_eq!(environment.current_state, (1, 4));

        environment.take_action(Action::East);
        environment.take_action(Action::East);
        environment.take_action(Action::North);
        environment.take_action(Action::North);
        environment.take_action(Action::North);
        environment.take_action(Action::North);

        assert_eq!(environment.current_state, (3, 0));
        assert_eq!(environment.take_action(Action::North), 5.);
    }

    #[test]
    fn it_returns_a_zero_reward_for_other_actions() {
        let mut environment = Environment::new(5, 5, (0, 0), vec![]);

        assert_eq!(environment.take_action(Action::East), 0.);
        assert_eq!(environment.take_action(Action::South), 0.);
        assert_eq!(environment.take_action(Action::West), 0.);
        assert_eq!(environment.take_action(Action::North), 0.);
    }
}
