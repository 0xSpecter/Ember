use crate::prelude::*;

#[derive(Default)]
pub struct App<'a> {
    pub window: Option<Arc<Window>>,
    pub state: Option<State<'a>>,
    pub input: Input,
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());

            window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));

            self.window = Some(window.clone());

            let state = block_on(State::new(window.clone()));
            self.state = Some(state);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();

                if let Some(state) = &mut self.state {

                    state.events(&self.input);
                    state.update();
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
            WindowEvent::Resized(physical_size) => {
                if let Some(state) = &mut self.state {
                    state.resize(physical_size);
                }
            }
            _ => (),
        }
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        println!("App suspended");
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        println!("App exiting");
    }
}
impl App<'_> {
    pub fn new() -> Self {
        App::default()
    }
}
