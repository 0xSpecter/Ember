use rsgl::prelude::*;

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();

    let _ = event_loop.run_app(&mut app);
}
