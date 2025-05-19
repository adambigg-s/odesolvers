use std::f32::consts::PI;

use odesolvers::plot::color_gradient;
use odesolvers::plot::wait;
use odesolvers::plot::Plot;
use odesolvers::plot::StateTracker;
use odesolvers::runge_kutta::Integrator;

const PLOT_WIDTH: usize = 220;
const PLOT_HEIGHT: usize = 70;

fn main() {
    let dt = 0.1;
    let final_time = 10000.;
    let initial_state: [f32; 4] = [PI * 10. / 11., PI * 10. / 11., 0., 0.];
    let mut integrator = Integrator::build(initial_state, dt, double_pendulum_dynamics);

    let mut states = StateTracker::build(30);
    let mut plot = Plot::build(PLOT_HEIGHT, PLOT_WIDTH);
    plot.xbounds(-30., 30.)
        .ybounds(-22., 22.)
        .set_settings()
        .subtick(true)
        .subtick_spacing(5.)
        .axis_color(90, 30, 30);

    while integrator.curr_time() < final_time {
        integrator.step();
        states.push((integrator.state(), integrator.curr_time()));
        states.state_pairs().for_each(|(&(start, t0), &(end, t1))| {
            let start = pendulum_state_to_cartesian(&start);
            let end = pendulum_state_to_cartesian(&end);

            let (red, green, blue) = color_gradient(0.1 * (t0 + t1));
            plot.set_brush().front_color(red, green, blue);
            plot.plot_line(start[0], start[1], end[0], end[1]);

            let (red, green, blue) = color_gradient(0.1 * (t0 + t1) + PI);
            plot.set_brush().front_color(red, green, blue);
            plot.plot_line(start[2], start[3], end[2], end[3]);
        });

        // plot the cable connecting the pendulum
        if let Some(final_state) = states.states.back() {
            plot.set_brush().front_color(0, 0, 0);

            let state = pendulum_state_to_cartesian(&final_state.0);
            plot.plot_line(0., 0., state[0], state[1]);
            plot.plot_line(state[0], state[1], state[2], state[3]);
        }

        plot.display();
        plot.clear();

        wait(25);
    }

    println!("double pendulum dynamics");
}

const M1: f32 = 3.;
const M2: f32 = 2.5;
const L1: f32 = 12.;
const L2: f32 = 8.;
const G: f32 = 9.81;
const C1: f32 = 0.001;
const C2: f32 = 0.01;

fn double_pendulum_dynamics(state: &[f32; 4]) -> [f32; 4] {
    let [t1, t2, w1, w2] = *state;

    let t1ddt = w1;
    let t2ddt = w2;

    let den_partial = 2. * M1 + M2 - M2 * (2. * t1 - 2. * t2).cos();

    let num1 = -G * (2. * M1 + M2) * t1.sin()
        - M2 * G * (t1 - 2. * t2).sin()
        - 2. * (t1 - t2).sin() * M2 * (w2 * w2 * L2 + w1 * w1 * L1 * (t1 - t2).cos());
    let mut w1ddt = num1 / (L1 * den_partial);
    // linear damping model
    w1ddt -= C1 * w1;

    let num2 = 2.
        * (t1 - t2).sin()
        * (w1 * w1 * L1 * (M1 + M2) + G * (M1 + M2) * t1.cos() + w2 * w2 * L2 * M2 * (t1 - t2).cos());
    let mut w2ddt = num2 / (L2 * den_partial);
    // linear damping model
    w2ddt -= C2 * w2;

    [t1ddt, t2ddt, w1ddt, w2ddt]
}

#[rustfmt::skip]
fn pendulum_state_to_cartesian(state: &[f32; 4]) -> [f32; 4] {
    let [t1, t2, ..] = *state;
    [
        L1 * t1.sin(),
        L1 * t1.cos() * -1.,
        L1 * t1.sin() + L2 * t2.sin(),
        L1 * t1.cos() * -1. - L2 * t2.cos(),
    ]
}
