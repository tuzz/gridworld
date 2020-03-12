mod agent;
mod environment;
mod render;
mod trajectory;

use game_loop::game_loop;
use itertools::izip;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::{cmp::min, collections::HashMap, thread::sleep, time::Duration};
use environment::{Environment, Action, SpecialState};
use agent::Agent;
use render::render;
use trajectory::Trajectory;

pub struct Game(Environment, Agent, Trajectory);

fn main() {
    let environment = Environment::new(5, 5, (2, 2), vec![
      SpecialState { coordinate: (1, 0), transitions_to: (1, 4), reward: 10. },
      SpecialState { coordinate: (3, 0), transitions_to: (3, 2), reward: 5. },
    ]);

    let game = Game(environment, Agent::new(0.9), Trajectory::new());
    render(&game);

    game_loop(game, 1, 1., |g| {
        let Game(environment, agent, trajectory) = &mut g.game;

        let state = environment.current_state;
        let actions = environment.available_actions(state);

        let selected = agent.select_action(state, &actions);
        let reward = environment.take_action(selected);

        agent.receive_reward(reward);
        trajectory.add(state, selected, reward);
    }, |g| {
        render(&g.game);
        sleep(Duration::from_millis(1000));
    });
}
