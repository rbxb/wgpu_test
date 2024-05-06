use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize, event::*,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId}
};

pub struct State<'a> {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'a>,
}

impl<'a> State<'a> {
    pub async fn new(window: Arc<Window>) -> State<'a> {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();

        Self {
            instance,
            surface,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        println!("State resize")
    }

    pub fn draw(&self) {
        println!("State draw")
    }
}

#[derive(Default)]
pub struct App<'a> {
    window: Option<Arc<Window>>,
    state: Option<State<'a>>,
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("App resumed");
        if self.window.is_none() {
            let window = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());
            self.window = Some(window.clone());

            let state = pollster::block_on(State::new(window.clone()));
            self.state = Some(state);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if id != self.window.as_ref().unwrap().id() {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                println!("Close requested");
                event_loop.exit()
            },
            WindowEvent::Resized(physical_size) => {
                println!("Resize requested");
                self.state.as_mut().unwrap().resize(physical_size);
            },
            WindowEvent::RedrawRequested => {
                println!("Redraw requested");
                self.state.as_ref().unwrap().draw();
            },
            _ => {},
        }
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        println!("App suspended");
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        println!("App exiting");
    }
}

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    if let Err(e) = event_loop.run_app(&mut app) {
        eprintln!("Event loop error: {e}");
    }
}

