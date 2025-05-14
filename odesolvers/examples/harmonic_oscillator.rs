use odesolvers::plot::color_gradient;
use odesolvers::plot::Plot;
use odesolvers::runge_kutta::Integrator;

const PLOT_WIDTH: usize = 220;
const PLOT_HEIGHT: usize = 70;

fn main() {
    println!("\x1b[2J");
    let dt = 0.01;
    let final_time = 120.;
    let initial_state = [10., -10.];
    let mut integrator = Integrator::build(initial_state, dt, harmonic_oscillator_dynamics);

    let mut states = Vec::new();
    let mut times = Vec::new();
    let mut plot = Plot::build(PLOT_HEIGHT, PLOT_WIDTH);
    plot.xbounds(0., 25.);

    while integrator.curr_time() < final_time {
        states.push(integrator.step());
        times.push(integrator.curr_time());
        states.windows(2).enumerate().for_each(|(time, window)| {
            let (start, end) = (window[0], window[1]);
            let (red, green, blue) = color_gradient(0.01 * time as f32);
            plot.set_brush().front_color(red, green, blue);
            plot.plot_line(times[time], start[0], times[time], end[0]);
        });
        plot.display();
        plot.clear();
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
