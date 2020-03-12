mod agent;
mod environment;

use game_loop::game_loop;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::{cmp::min, collections::HashMap};
use environment::{Environment, Action, SpecialState};
use agent::Agent;

fn main() {
    let environment = Environment::new(5, 5, (1, 0), vec![
      SpecialState { coordinate: (1, 0), transitions_to: (1, 4), reward: 10. },
      SpecialState { coordinate: (3, 0), transitions_to: (3, 2), reward: 5. },
    ]);

    let agent = Agent::new();

    game_loop((environment, agent), 5, 1., |g| {
        let (environment, agent) = &mut g.game;

        let state = environment.current_state;
        let actions = environment.available_actions(state);

        let selected = agent.select_action(state, &actions);
        let reward = environment.take_action(selected);

        agent.receive_reward(reward);
    }, |_g| {

    });

    println!("Hello, world!");
}
