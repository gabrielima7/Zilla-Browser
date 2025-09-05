use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn create_and_run_gui() {
    let event_loop = EventLoop::new();
    let _window = WindowBuilder::new()
        .with_title("Zilla Browser")
        .with_inner_size(tao::dpi::LogicalSize::new(1280, 720))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
