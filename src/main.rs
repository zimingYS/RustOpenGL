extern crate glium;

use std::fmt::Debug;
use glium::winit;

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
    let window = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
}
