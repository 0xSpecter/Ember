use crate::prelude::*;

pub struct App<'a> {
    pub window: Option<Arc<Window>>,
    pub state: Option<State<'a>>,
    pub input: Input,
    pub last_render_time: instant::Instant
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());

            let _ = window.set_cursor_grab(CursorGrabMode::Locked);
            window.set_cursor_visible(false);

            window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));

            self.window = Some(window.clone());

            let state = block_on(State::new(window.clone()));
            self.state = Some(state);
        }
    }

    fn device_event(
            &mut self,
            _event_loop: &ActiveEventLoop,
            _device_id: DeviceId,
            event: DeviceEvent,
        ) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                self.input.mouse_delta = Vec2::new(delta.0 as f32, delta.1 as f32);
            },
            _ => (),
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();

                if let Some(state) = &mut self.state {
                    let now = Instant::now();
                    let delta = now - self.last_render_time;
                    self.last_render_time = now;

                    state.events(&self.input);
                    state.update(delta);
                    let _ = state.render();


                    self.input.update();
                }
            },
            WindowEvent::KeyboardInput {device_id: _, event, is_synthetic: _} => {
                if let PhysicalKey::Code(key) = event.physical_key {
                    match &key {
                        KeyCode::Escape => event_loop.exit(),
                        KeyCode::KeyO => event_loop.exit(),
                        _ => ()
                    } 
                }

                self.input.add_key(event);
            },
            WindowEvent::CursorMoved { device_id: _, position } => {
                self.input.mouse_position = Vec2::new(position.x as f32, position.y as f32);
            },
            WindowEvent::Resized(physical_size) => {
                if let Some(state) = &mut self.state {
                    state.resize(physical_size);
                }
            }
            _ => (),
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        println!("App suspended");
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        println!("App exiting");
    }
}

impl App<'_> {
    pub fn new() -> Self {
        App {
            last_render_time: Instant::now(),
            window: Default::default(),
            state: Default::default(),
            input: Default::default(),
        }
    }
}
