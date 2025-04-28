# odesolvers
a basic numerical ODE solver using Runge-Kutta 4 and RK5 in Rust as an
alternative to ode45

in addition, i am building a terminal plotting library in here. think like
matplotlib, but all in the terminal using ansi escape codes and ascii braille
characters primarily for plotting. eventually, if this project actually starts
working very well, i will break the terminal plotting out into a separate
project, because it is seeming pretty cool so far for quick plotting.

view the 'examples' folder for some cool plots using it, and how it is used. it
is a little more cumbersome than like a matplotlib because it has to follow Rust
typing rules, but it is fairly simple.

![alt text](https://github.com/adambigg-s/odesolvers/blob/main/demos/lorenz_attractor.gif)
