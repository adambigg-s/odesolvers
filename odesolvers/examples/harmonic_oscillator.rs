use odesolvers::runge_kutta::Integrator;

const PLOT_WIDTH: usize = 220;
const PLOT_HEIGHT: usize = 70;

fn main() {
    println!("\x1b[2J");
    let dt = 0.001;
    let final_time = 120.;
    let initial_state = [1., -10.];
    let mut integrator = Integrator::build(initial_state, dt, harmonic_oscillator_dynamics);

    let mut states = Vec::new();
    while integrator.curr_time() < final_time {
        states.push(integrator.step());
    }

    println!("harmonic oscillator example");
}

const C: f32 = 0.5;
const K: f32 = 3.;
const M: f32 = 1.;

#[rustfmt::skip]
fn harmonic_oscillator_dynamics(state: &[f32; 2]) -> [f32; 2] {
    let [x, v] = state;
    [
        *v,
        -K / M * x + -C / M * v,
    ]
}
