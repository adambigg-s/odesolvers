use odesolvers::plot::color_gradient;
use odesolvers::plot::Plot;
use odesolvers::runge_kutta::Integrator;

const PLOT_WIDTH: usize = 220;
const PLOT_HEIGHT: usize = 70;

fn main() {
    let dt = 0.05;
    let final_time = 120.;
    let initial_state = [10., -10.];
    let mut integrator = Integrator::build(initial_state, dt, harmonic_oscillator_dynamics);

    let mut states = Vec::new();
    let mut times = Vec::new();
    let mut plot = Plot::build(PLOT_HEIGHT, PLOT_WIDTH);
    plot.xbounds(-10., 120.).ybounds(-10., 10.).set_settings().subtick(true).subtick_spacing(2.);

    while integrator.curr_time() < final_time {
        states.push(integrator.dynamic_step());
        times.push(integrator.curr_time());
        states.windows(2).zip(times.windows(2)).for_each(|(window, time)| {
            let (start, end) = (window[0], window[1]);
            let (t0, t1) = (time[0], time[1]);
            let (red, green, blue) = color_gradient(0.1 * t0);
            plot.set_brush().front_color(red, green, blue);
            plot.plot_line(t0, start[0], t1, end[0]);
        });
        plot.display();
        plot.clear();
    }

    println!("harmonic oscillator example");
}

const C: f32 = 0.1;
const K: f32 = 7.;
const M: f32 = 1.5;

#[rustfmt::skip]
fn harmonic_oscillator_dynamics(state: &[f32; 2]) -> [f32; 2] {
    let [x, v] = *state;
    [
        v,
        -K / M * x + -C / M * v,
    ]
}
