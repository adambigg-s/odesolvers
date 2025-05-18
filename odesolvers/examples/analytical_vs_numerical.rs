use odesolvers::plot::Plot;
use odesolvers::runge_kutta::Integrator;

const PLOT_WIDTH: usize = 220;
const PLOT_HEIGHT: usize = 70;

fn main() {
    let dt = 0.1;
    let final_time = 17.;
    let initial_state = [10., -3.];
    let mut integrator = Integrator::build(initial_state, dt, oscillator_dynamics);

    let mut states = Vec::new();
    let mut times = Vec::new();
    let mut plot = Plot::build(PLOT_HEIGHT, PLOT_WIDTH);
    plot.xbounds(-5., 20.).ybounds(-7., 7.).set_settings().subtick(true).subtick_spacing(3.);
    plot.apply_settings();

    // numerical solution
    while integrator.curr_time() < final_time {
        states.push(integrator.dynamic_step());
        times.push(integrator.curr_time());
        states.windows(2).zip(times.windows(2)).for_each(|(window, time)| {
            let (start, end) = (window[0], window[1]);
            let (t0, t1) = (time[0], time[1]);
            plot.set_brush().front_color(255, 0, 0);
            plot.plot_line(t0, start[0], t1, end[0]);
        });
        plot.display();
        plot.clear_string();
    }

    // analytical solution plotted against
    let points = 500;
    (0..points).for_each(|i| {
        let (points, i) = (points as f64, i as f64);
        let t0 = final_time / points * i;
        let t1 = final_time / points * (i + 1.);
        let (x0, x1) = (oscillator_analytical(t0, &initial_state), oscillator_analytical(t1, &initial_state));
        plot.set_brush().front_color(0, 0, 255);
        plot.plot_line(t0, x0, t1, x1);
        plot.display();
        plot.clear_string();
    });

    println!("analytical vs numerical solution for under-damped harmonic oscialltor");
}

const C: f64 = 0.55;
const K: f64 = 3.;
const M: f64 = 1.;

#[rustfmt::skip]
fn oscillator_dynamics(state: &[f64; 2]) -> [f64; 2] {
    let [x, v] = *state;
    [
        v,
        - K / M * x + -C / M * v,
    ]
}

#[rustfmt::skip]
fn oscillator_analytical(t: f64, initial_state: &[f64; 2]) -> f64 {
    let [x0, v0] = *initial_state;

    let gamma = C / (2. * M);
    let omega0 = (K / M).sqrt();
    let omegad = (omega0 * omega0 - gamma * gamma).sqrt();

    let a = x0;
    let phi = (v0 + gamma * x0) / omegad;

    a * (-gamma * t).exp() * (omegad * t).cos() + phi * (-gamma * t).exp() * (omegad * t).sin()
}
