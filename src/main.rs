use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
pub mod components;
pub mod application;
pub mod color;
pub mod window;

fn main() {
    let fps: u16 = 30;
    pollster::block_on(run(fps));
}

// If planning to support WASM, don't use block_on inside of an async function
// -- Futures have to be run using the browser's executor
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run(fps: u16) {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = application::Application::new(window).await;

    let frame_time = std::time::Duration::from_secs_f64(1.0 / fps as f64);
    let mut last_frame = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(last_frame + frame_time);

        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window.id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        },
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.resize(**new_inner_size);
                        },
                        _ => {}
                    }
                }
            },
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                state.update();
                match state.render() {
                    Ok(_) => {},
                    // Reconfigure surface if lost
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        state.resize(state.size)
                    },
                    // System is out of memory
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by next frame
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }

                last_frame = std::time::Instant::now();
                *control_flow = ControlFlow::WaitUntil(last_frame + frame_time);
            },
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once unless manually requested
                state.window().request_redraw();
            }
            _ => {}
        }
    });
}