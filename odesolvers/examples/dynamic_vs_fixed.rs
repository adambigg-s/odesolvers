use odesolvers::plot::Plot;
use odesolvers::runge_kutta::Integrator;

const PLOT_WIDTH: usize = 220;
const PLOT_HEIGHT: usize = 70;

fn main() {
    println!("\x1b[2J");
    let dt = 0.01;
    let final_time = 120.;
    let initial_state = [10., -10.];
    let mut fixedegrator = Integrator::build(initial_state, dt, harmonic_oscillator_dynamics);
    let mut dyanmicgrator = Integrator::build(initial_state, dt, harmonic_oscillator_dynamics);

    let mut plot = Plot::build(PLOT_HEIGHT, PLOT_WIDTH);
    plot.xbounds(0., 100.);

    let fixed = fixedegrator.solve_with_time(final_time);
    let dynamic = dyanmicgrator.solve_dynamic_with_time(final_time);

    fixed.windows(2).for_each(|window| {
        let (start, end) = (window[0], window[1]);
        plot.set_brush().front_color(255, 50, 10);
        plot.plot_line(start.0, start.1[0], end.0, end.1[0]);
    });
    dynamic.windows(2).for_each(|window| {
        let (start, end) = (window[0], window[1]);
        plot.set_brush().front_color(10, 50, 255);
        plot.plot_line(start.0, start.1[0], end.0, end.1[0]);
    });
    plot.display();

    println!("fixed vs dynamic step size comparison");
}

const C: f32 = 0.05;
const K: f32 = 3.;
const M: f32 = 1.5;

#[rustfmt::skip]
fn harmonic_oscillator_dynamics(state: &[f32; 2]) -> [f32; 2] {
    let [x, v] = state;
    [
        *v,
        -K / M * x + -C / M * v,
    ]
}
