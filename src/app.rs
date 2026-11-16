use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use crate::graphics::Graphics;
use crate::physics::Physics;

pub struct App {
    graphics: Option<Graphics>,
    physics: Option<Physics>
}

impl App {
    pub fn init() -> Self {
        Self {
            graphics: None,
            physics: None
        }
    }

    fn setup_graphics(&mut self, event_loop: &ActiveEventLoop) {
        if self.graphics.is_some() {
            return;
        }

        let window_attrs = Window::default_attributes().with_title("Kyte Engine");
        let window = Arc::new(event_loop.create_window(window_attrs).unwrap());
        self.graphics = Some(Graphics::init(window));
    }

    fn setup_physics(&mut self) {
        if self.physics.is_some() {
            return;
        }

        self.physics = Some(Physics::init(240))
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.setup_graphics(event_loop);
        self.setup_physics()
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(graphics) = self.graphics.as_mut() else {
            return;
        };

        let Some(physics) = self.physics.as_mut() else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => graphics.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                graphics.render();

                // step, make screen redder, when it is completely red, close the program
                // for test purposes only
                let steps_taken = physics.step();
                graphics.clear_color.r += (steps_taken as f64) / 3600.0;
                if graphics.clear_color.r >= 1.0 {
                    event_loop.exit();
                }

                graphics.request_redraw();
            }
            _ => {}
        }
    }
}
