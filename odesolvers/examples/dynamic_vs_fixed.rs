use odesolvers::plot::Plot;
use odesolvers::runge_kutta::Integrator;

const PLOT_WIDTH: usize = 220;
const PLOT_HEIGHT: usize = 70;

fn main() {
    let dt = 0.05;
    let final_time = 120.;
    let initial_state = [0., -25.];
    let mut fixedegrator = Integrator::build(initial_state, dt, harmonic_oscillator_dynamics);
    let mut dyanmicgrator = Integrator::build(initial_state, dt, harmonic_oscillator_dynamics);

    let mut plot = Plot::build(PLOT_HEIGHT, PLOT_WIDTH);
    plot.xbounds(-5., 120.).set_settings().subtick(true).subtick_spacing(10.);

    let mut fixed = Vec::new();
    let mut fixed_times = Vec::new();
    let mut dynamic = Vec::new();
    let mut dynamic_times = Vec::new();
    while fixedegrator.curr_time() < final_time || dyanmicgrator.curr_time() < final_time {
        fixed.push(fixedegrator.step());
        fixed_times.push(fixedegrator.curr_time());
        dynamic.push(dyanmicgrator.dynamic_step());
        dynamic_times.push(dyanmicgrator.curr_time());

        plot.set_brush().front_color(10, 10, 255);
        fixed.windows(2).zip(fixed_times.windows(2)).for_each(|(window, time)| {
            let (start, end) = (window[0], window[1]);
            let (t0, t1) = (time[0], time[1]);
            plot.plot_line(t0, start[0], t1, end[0]);
        });
        plot.set_brush().front_color(255, 10, 10);
        dynamic.windows(2).zip(dynamic_times.windows(2)).for_each(|(window, time)| {
            let (start, end) = (window[0], window[1]);
            let (t0, t1) = (time[0], time[1]);
            plot.plot_line(t0, start[0], t1, end[0]);
        });

        plot.display();
        plot.clear();
    }

    println!("fixed vs dynamic step size comparison");
}

const C: f32 = 0.15;
const K: f32 = 7.;
const M: f32 = 3.;

#[rustfmt::skip]
fn harmonic_oscillator_dynamics(state: &[f32; 2]) -> [f32; 2] {
    let [x, v] = state;
    [
        *v,
        -K / M * x + -C / M * v,
    ]
}
