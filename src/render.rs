use crate::*;

pub fn render(Game(environment, agent, trajectory): &Game) {
    print!("\x1B[2J"); // Clear the terminal

    let jumps = environment.special_states.len();
    println!("State-Value function for the random policy on a grid with {} jumps:", jumps);
    println!();

    for y in 0..environment.grid_height {
        print_divider(y, environment);
        print_spacer(y, environment);
        print!("|");
        for x in 0..environment.grid_width {
            let state = (x, y);
            let value = agent.state_value_function.get(&state).unwrap_or(&0.);

            print!("  {:.1}  |", value);
        }
        println!();
        print_spacer(y, environment);
    }
    print_divider(999, environment);

    println!();
    println!("Trajectory at time={} (state, value, reward):", trajectory.states.len());
    println!();

    let zipped = izip!(&trajectory.states, &trajectory.actions, &trajectory.rewards)
        .collect::<Vec<_>>();

    for (time, (state, action, reward)) in zipped.iter().enumerate() {
        match time {
            t if t < 2 || t >= zipped.len() - 2 => {
                print!("{:?}, {:?}, {:.1}, ", state, action, reward);
            }
            2 => {
                print!("... ({} omitted) ... ", zipped.len() - 4)
            }
            _ => {},
        }
    }
    println!();
}

fn print_divider(y: u32, environment: &Environment) {
    let mut parts = vec!["--------".to_string(); environment.grid_width as usize];

    for (id, special_state) in environment.special_states.iter().enumerate() {
        if y == special_state.coordinate.1 {
            let x = special_state.coordinate.0;

            parts[x as usize] = format!(" Jump {} ", id + 1);
        }

        if y == special_state.transitions_to.1 {
            let x = special_state.transitions_to.0;

            parts[x as usize] = format!(" >> {} << ", id + 1);
        }
    }

    println!("{}-", parts.join(""));
}

fn print_spacer(y: u32, environment: &Environment) {
    let mut parts = vec!["|       "; environment.grid_width as usize];

    if y == environment.current_state.1 {
        let x = environment.current_state.0;

        parts[x as usize] = "| xxxxx ";
    }

    println!("{}|", parts.join(""));
}
