use logfather::*;
mod core;
use core::application;

fn main() {
    let _logger = Logger::new();
    
    application::run("KoZy \\o/", 30.0);
}




// // If planning to support WASM, don't use block_on inside of an async function
// // -- Futures have to be run using the browser's executor
// #[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
// pub async fn run(fps: u16) {
//     env_logger::init();
//     let event_loop = winit::EventLoopWrapper::new();
    
//     let window = winit::window::WindowBuilder::new().build(&event_loop).unwrap();
//     let mut state = application::Application::new(std::sync::Arc::new(window)).await;

//     let frame_time = std::time::Duration::from_secs_f64(1.0 / fps as f64);
//     let mut last_frame = std::time::Instant::now();

//     event_loop.run(move |event, event_loop_window_target| {
//         event_loop_window_target.set_control_flow(ControlFlow::WaitUntil(last_frame + frame_time));

//         match event {
//             Event::WindowEvent {
//                 ref event,
//                 window_id,
//             } if window_id == window.id() => {
//                 if state.input(event) {
//                     match event {
//                         WindowEvent::ActivationTokenDone { serial, token } => todo!(),
//                         WindowEvent::Resized(physical_size) => {
//                             state.resize(*physical_size);
//                         },
//                         WindowEvent::Moved(_) => todo!(),
//                         WindowEvent::CloseRequested => todo!(),
//                         WindowEvent::Destroyed => todo!(),
//                         WindowEvent::DroppedFile(_) => todo!(),
//                         WindowEvent::HoveredFile(_) => todo!(),
//                         WindowEvent::HoveredFileCancelled => todo!(),
//                         WindowEvent::Focused(_) => todo!(),
//                         WindowEvent::KeyboardInput { device_id, event, is_synthetic } => todo!(),
//                         WindowEvent::ModifiersChanged(_) => todo!(),
//                         WindowEvent::Ime(_) => todo!(),
//                         WindowEvent::CursorMoved { device_id, position } => todo!(),
//                         WindowEvent::CursorEntered { device_id } => todo!(),
//                         WindowEvent::CursorLeft { device_id } => todo!(),
//                         WindowEvent::MouseWheel { device_id, delta, phase } => todo!(),
//                         WindowEvent::MouseInput { device_id, state, button } => todo!(),
//                         WindowEvent::TouchpadMagnify { device_id, delta, phase } => todo!(),
//                         WindowEvent::SmartMagnify { device_id } => todo!(),
//                         WindowEvent::TouchpadRotate { device_id, delta, phase } => todo!(),
//                         WindowEvent::TouchpadPressure { device_id, pressure, stage } => todo!(),
//                         WindowEvent::AxisMotion { device_id, axis, value } => todo!(),
//                         WindowEvent::Touch(_) => todo!(),
//                         WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer } => {},
//                         WindowEvent::ThemeChanged(_) => todo!(),
//                         WindowEvent::Occluded(_) => todo!(),
//                         WindowEvent::RedrawRequested => todo!(),
//                         // WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
//                         //     input:
//                         //         KeyboardInput {
//                         //             state: ElementState::Pressed,
//                         //             virtual_keycode: Some(VirtualKeyCode::Escape),
//                         //             ..
//                         //         },
//                         //     ..
//                         // } => *control_flow = ControlFlow::Exit,
//                         // WindowEvent::Resized(physical_size) => {
//                         //     state.resize(*physical_size);
//                         // },
//                         // WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
//                         //     state.resize(**new_inner_size);
//                         // },
//                         // _ => {}
//                     }
//                 }
//             },
//             Event::NewEvents(_) => todo!(),
//             Event::DeviceEvent { device_id, event } => todo!(),
//             Event::UserEvent(_) => todo!(),
//             Event::Suspended => todo!(),
//             Event::Resumed => todo!(),
//             Event::AboutToWait => todo!(),
//             Event::LoopExiting => todo!(),
//             Event::MemoryWarning => todo!(),
//             // Event::RedrawRequested(window_id) if window_id == state.window().id() => {
//             //     state.update();
//             //     match state.render() {
//             //         Ok(_) => {},
//             //         // Reconfigure surface if lost
//             //         Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
//             //             state.resize(state.size)
//             //         },
//             //         // System is out of memory
//             //         Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
//             //         // All other errors (Outdated, Timeout) should be resolved by next frame
//             //         Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
//             //     }

//             //     last_frame = std::time::Instant::now();
//             //     *control_flow = ControlFlow::WaitUntil(last_frame + frame_time);
//             // },
//             // Event::MainEventsCleared => {
//             //     // RedrawRequested will only trigger once unless manually requested
//             //     state.window().request_redraw();
//             // }
//             // _ => {}
//         }
//     });
// }