mod agent;
mod environment;
mod render;
mod trajectory;

use game_loop::game_loop;
use itertools::izip;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::{cmp::min, collections::HashMap, thread::sleep, time::Duration, env::args};
use environment::{Environment, Action, SpecialState};
use agent::Agent;
use render::render;
use trajectory::Trajectory;

pub struct Game(Environment, Agent, Trajectory);

fn main() {
    let args = args().collect::<Vec<String>>();

    let (environment, updates_per_second) = match args.get(1) {
        Some(s) if s == "massive" => {
            let jumps = (0..22).map(|i| {
              SpecialState { coordinate: (2 * i + 2, i),  transitions_to: (0, i), reward: 3. * i as f32 }
            }).collect();

            (Environment::new(50, 25, (2, 2), jumps), 100_000)
        },

        Some(s) if s == "large" => {
            (Environment::new(23, 11, (2, 2), vec![
              SpecialState { coordinate: (2, 0),  transitions_to: (0, 2),  reward: 2. },
              SpecialState { coordinate: (3, 0),  transitions_to: (2, 3),  reward: 3. },
              SpecialState { coordinate: (5, 0),  transitions_to: (3, 5),  reward: 5. },
              SpecialState { coordinate: (7, 0),  transitions_to: (5, 7),  reward: 7. },
              SpecialState { coordinate: (11, 0), transitions_to: (7, 2),  reward: 11. },
              SpecialState { coordinate: (13, 0), transitions_to: (11, 3), reward: 13. },
              SpecialState { coordinate: (17, 0), transitions_to: (13, 5), reward: 17. },
              SpecialState { coordinate: (19, 0), transitions_to: (17, 7), reward: 19. },
            ]), 10_000)
        },

        _ => { // Defaults to the gridworld environment from page 65 of the book:
            (Environment::new(5, 5, (2, 2), vec![
              SpecialState { coordinate: (1, 0), transitions_to: (1, 4), reward: 10. },
              SpecialState { coordinate: (3, 0), transitions_to: (3, 2), reward: 5. },
            ]), 1000)
        }
    };

    let game = Game(environment, Agent::new(0.9), Trajectory::new());
    render(&game);

    game_loop(game, updates_per_second, 1., |g| {
        let Game(environment, agent, trajectory) = &mut g.game;

        let state = environment.current_state;
        let actions = environment.available_actions(state);

        let selected = agent.select_action(state, &actions);
        let reward = environment.take_action(selected);

        let new_state = environment.current_state;
        agent.receive_reward(reward, new_state);

        trajectory.add(state, selected, reward);
    }, |g| {
        render(&g.game);
        sleep(Duration::from_millis(250));
    });
}
