use odesolvers::plot::Plot;
use odesolvers::runge_kutta::Integrator;

const PLOT_WIDTH: usize = 220;
const PLOT_HEIGHT: usize = 70;

fn main() {
    let dt = 0.02;
    let final_time = 120.;
    let initial_state = [0.01, 0., -0.01];
    let mut integrator = Integrator::build(initial_state, dt, lorenz_dynamics);

    let mut states = Vec::new();
    let mut plot = Plot::build(PLOT_HEIGHT, PLOT_WIDTH);
    while integrator.curr_time() < final_time {
        states.push(integrator.step());
        states.windows(2).for_each(|window| {
            let (start, end) = (window[0], window[1]);
            plot.plot_line(start[0], start[1], end[0], end[1]);
        });
        plot.display();
        plot.clear();
        println!("size of string buffer: {}", plot.output_string.capacity());
    }

    println!("lorenz attractor example");
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
