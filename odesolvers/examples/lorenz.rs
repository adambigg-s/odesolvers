use odesolvers::plot::color_gradient;
use odesolvers::plot::Plot;
use odesolvers::runge_kutta::Integrator;

const PLOT_WIDTH: usize = 220;
const PLOT_HEIGHT: usize = 70;

fn main() {
    let dt = 0.1;
    let final_time = 120.;
    let initial_state = [0.001, 0.001, 0.001];
    let mut integrator = Integrator::build(initial_state, dt, lorenz_dynamics);

    let mut states = Vec::new();
    let mut plot = Plot::build(PLOT_HEIGHT, PLOT_WIDTH);
    plot.xbounds(-30., 30.).ybounds(-3., 60.).set_settings().subtick(true).subtick_spacing(3.);

    while integrator.curr_time() < final_time {
        states.push(integrator.dynamic_step());
        states.windows(2).enumerate().for_each(|(time, window)| {
            let (start, end) = (window[0], window[1]);
            let (red, green, blue) = color_gradient(0.001 * time as f32);
            plot.set_brush().front_color(red, green, blue);
            plot.plot_line(start[0], start[2], end[0], end[2]);
        });
        plot.display();
        plot.clear();
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
