use std::f64::consts::PI;

use odesolvers::plot::color_gradient;
use odesolvers::plot::wait;
use odesolvers::plot::Plot;
use odesolvers::plot::StateTracker;
use odesolvers::runge_kutta::Integrator;

const PLOT_WIDTH: usize = 220;
const PLOT_HEIGHT: usize = 70;

fn main() {
    let dt = 0.1;
    let final_time = 30.;
    let initial_state = [-PI * 5. / 11., 0., 0., 0.];
    let mut integrator = Integrator::build(initial_state, dt, pendulum_dynamics);

    let mut states = StateTracker::build(30);
    let mut plot = Plot::build(PLOT_HEIGHT, PLOT_WIDTH);
    plot.xbounds(-15., 15.).ybounds(-20., 5.).set_settings().subtick(true).subtick_spacing(3.);

    while integrator.curr_time() < final_time {
        integrator.step();
        states.push((integrator.state(), integrator.curr_time()));
        states.state_pairs().for_each(|(&(start, t0), &(end, t1))| {
            let (red, green, blue) = color_gradient((t0 + t1) as f32 / 2.);
            plot.set_brush().front_color(red, green, blue);

            let (x0, y0) = (L * start[0].sin(), -L * start[0].cos());
            let (x1, y1) = (L * end[0].sin(), -L * end[0].cos());
            plot.plot_line(x0, y0, x1, y1);
        });

        // plot the cable connecting the pendulum
        if let Some(final_state) = states.states.back() {
            plot.set_brush().front_color(0, 0, 0);
            plot.plot_line(0., 0., L * final_state.0[0].sin(), -L * final_state.0[0].cos());
        }

        plot.display();
        plot.clear();

        wait(25);
    }

    println!("pendulum dynamics");
}

const G: f64 = 9.8;
const L: f64 = 10.;
const C: f64 = 0.05;

#[rustfmt::skip]
fn pendulum_dynamics(state: &[f64; 4]) -> [f64; 4] {
    let [theta, theta_dot, ..] = *state;
    [
        theta_dot,
        -theta.sin() * G / L + -theta_dot * C,
        1.,
        1.,
    ]
}
