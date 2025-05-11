use odesolvers::plot::Plot;
use odesolvers::runge_kutta::Integrator;

const PLOT_WIDTH: usize = 220;
const PLOT_HEIGHT: usize = 70;

fn main() {
    println!("\x1b[2J");
    let dt = 0.001;
    let final_time = 120.;
    let initial_state = [0.01, 0., -0.01];
    let mut integrator = Integrator::build(initial_state, dt, lorenz_dynamics);

    let mut states = Vec::new();
    let mut plot = Plot::build(PLOT_HEIGHT, PLOT_WIDTH);
    while integrator.curr_time() < final_time {
        states.push(integrator.step());

        for state in &states {
            plot.set_point(state[0] as f32, state[1] as f32);
        }
        plot.display();
    }

    println!("lorenz attractor example with new plot");
}

const SIGMA: f64 = 10.;
const RHO: f64 = 28.;
const BETA: f64 = 8. / 3.;

#[rustfmt::skip]
fn lorenz_dynamics(state: &[f64; 3]) -> [f64; 3] {
    let [x, y, z] = state;
    [
        SIGMA * (y - x),
        x * (RHO - z) - y,
        x * y - BETA * z,
    ]
}
