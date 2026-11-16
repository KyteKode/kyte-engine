mod app;
mod graphics;
mod physics;

use winit::event_loop::{ControlFlow, EventLoop};

use app::App;

pub struct World {
    app: App,
}

impl World {
    pub fn new() -> Self {
        Self { app: App::init() }
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut self.app).unwrap();
    }
}
