use winit::{event::*, keyboard::NamedKey};
use std::sync::Arc;
use logfather::*;
use crate::core::color;

#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

pub struct Mouse {
    pub x_pos: f64,
    pub y_pos: f64,
}

pub struct Application<'window> {
    pub instance: wgpu::Instance,
    pub window: std::sync::Arc<winit::window::Window>,
    pub adapter: wgpu::Adapter,
    pub surface: wgpu::Surface<'window>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub clear_color: wgpu::Color,
    pub render_pipeline: wgpu::RenderPipeline,

    pub mouse: Mouse,
    pub cursor_active: bool,

    pub frame_count: u32,
    pub last_frame_time: std::time::Instant,
    pub last_fps_update: std::time::Instant,
}


struct EventLoop {
    event_loop: winit::event_loop::EventLoop<()>,
    window: Arc<winit::window::Window>,
}

impl EventLoop {
    pub fn new(title: &str) -> Self {
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        let mut builder = winit::window::WindowBuilder::new().with_title(title);
        let window = Arc::new(builder.build(&event_loop).unwrap());

        Self { event_loop, window }
    }
}

struct Core {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl Core {
    pub async fn new(window: Arc<winit::window::Window>) -> Self {
        // Instance creates Adapters and Surfaces
        // BackendBit::Primary => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        // Handle for the GPU
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                // LowPower or HighPerformance - automatically prioritize
                // ::default() -> LowPower
                power_preference: wgpu::PowerPreference::HighPerformance,
                // Tells wgpu to find an adapter that can present to the supplied surface
                compatible_surface: Some(&surface),
                // Forces wgpu to pick an adapter that will work on all hardware
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                // Full list of features: https://docs.rs/wgpu/latest/wgpu/struct.Features.html
                required_features: wgpu::Features::empty(),
                // Full list of limits: https://docs.rs/wgpu/latest/wgpu/struct.Limits.html
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None,
        ).await.unwrap();

        Self { instance, adapter, device, queue }
    }
}



struct Surface {
    surface: Option<wgpu::Surface<'static>>,
    config: Option<wgpu::SurfaceConfiguration>,
}

impl Surface {
    fn new() -> Self {
        Self { surface: None, config: None }
    }

    fn start_condition(event: &Event<()>) -> bool {
        match event {
            Event::NewEvents(StartCause::Init) => !cfg!(target_os = "android"),
            Event::Resumed => cfg!(target_os = "android"),
            _ => false,
        }
    }

    fn init(&mut self, context: &Core, window: std::sync::Arc<winit::window::Window>, srgb: bool) {
        let window_size = window.inner_size();
        let width = window_size.width.max(1);
        let height = window_size.height.max(1);

        info!("Surface initiation: {window_size:?}");

        self.surface = Some(match context.instance.create_surface(window) {
            Ok(surface) => surface,
            Err(e) => panic!("Could not create surface: {e}"), // TODO: Handle this better
        });

        let mut config = match self.surface().get_default_config(&context.adapter, width, height) {
            Some(cfg) => cfg,
            None => panic!("Could not generate default configuration"), // TODO: Handle this better
        };

        let view_format = if srgb { 
            config.format.add_srgb_suffix() 
        } else { 
            let fmt = config.format.remove_srgb_suffix();
            config.format = fmt;
            fmt
        };
        config.view_formats.push(view_format);

        self.surface().configure(&context.device, &config);
        self.config = Some(config);
    }

    /// Resize the surface - 0 is not allowed
    fn resize(&mut self, context: &Core, size: winit::dpi::PhysicalSize<u32>) {
        info!("Surface resize {size:?}");

        let config = self.config.as_mut().unwrap();
        config.width = size.width.max(1);
        config.height = size.height.max(1);
        let surface = self.surface.as_ref().unwrap();
        surface.configure(&context.device, config);
    }

    /// Get the next surface texture
    fn acquire(&mut self, context: &Core) -> wgpu::SurfaceTexture {
        let surface = self.surface.as_ref().unwrap();

        match surface.get_current_texture() {
            Ok(frame) => frame,
            // If we timed out, just try again
            Err(wgpu::SurfaceError::Timeout) => surface
                .get_current_texture()
                .expect("Failed to acquire next surface texture!"),
            Err(
                // If the surface is outdated, or was lost, reconfigure it.
                wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost
                // If OutOfMemory happens, reconfiguring may not help, but we might as well try
                | wgpu::SurfaceError::OutOfMemory,
            ) => {
                surface.configure(&context.device, self.config());
                surface
                    .get_current_texture()
                    .expect("Failed to acquire next surface texture!")
            }
        }
    }


    fn get(&self) -> &wgpu::Surface {
        return self.surface.as_ref().unwrap();
    }

    fn surface(&self) -> &wgpu::Surface {
        return self.surface.as_ref().unwrap();
    }

    fn config(&self) -> &wgpu::SurfaceConfiguration {
        return self.config.as_ref().unwrap();
    }
}

struct FrameCounter {
    last_instant: std::time::SystemTime,
    frame_count: u32,
    /// Refresh rate in seconds
    refresh_rate: f32,
}

impl FrameCounter {
    fn new(refresh_rate: f32) -> Self {
        Self {
            last_instant: std::time::SystemTime::now(),
            frame_count: 0,
            refresh_rate,
        }
    }

    fn update(&mut self) {
        self.frame_count += 1;
        let elapsed = self.last_instant.elapsed().expect("Time went backwards").as_secs_f32();
        if elapsed >= self.refresh_rate {
            let fps =self.frame_count as f32 / self.refresh_rate;
            println!("FPS: {:.2}", fps);

            self.last_instant = std::time::SystemTime::now();
            self.frame_count = 0;
        }
    }
}

pub fn run(title: &'static str, target: f64) {
    pollster::block_on(start(title, target));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
async fn start(title: &str, target: f64) {
    let window_loop = EventLoop::new(title);
    let mut surface = Surface::new();
    let context = Core::new(window_loop.window.clone()).await;
    let mut frame_counter = FrameCounter::new(2.0);

    let frame_time = std::time::Duration::from_secs_f64(1.0 / target);
    let mut last_frame = std::time::Instant::now();

    info!("Entering event loop");
    let _ = window_loop.event_loop.run(move |event, window_target| {
        window_target.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(last_frame + frame_time));
        
        match event {
            ref e if Surface::start_condition(e) => {
                surface.init(&context, window_loop.window.clone(), true);
            }
            // Event::NewEvents(_) => todo!(),
            Event::WindowEvent { event, .. } => match event {
                // WindowEvent::ActivationTokenDone { serial, token } => todo!(),
                WindowEvent::Resized(size) => {
                    surface.resize(&context, size);
                    window_loop.window.request_redraw();
                },
                // WindowEvent::Moved(_) => todo!(),
                WindowEvent::CloseRequested => window_target.exit(),
                // WindowEvent::Destroyed => todo!(),
                // WindowEvent::DroppedFile(_) => todo!(),
                // WindowEvent::HoveredFile(_) => todo!(),
                // WindowEvent::HoveredFileCancelled => todo!(),
                // WindowEvent::Focused(_) => todo!(),
                WindowEvent::KeyboardInput { event: KeyEvent {
                    logical_key: winit::keyboard::Key::Named(NamedKey::Escape),
                    ..
                }, .. } => {
                    window_target.exit();
                },
                // WindowEvent::ModifiersChanged(_) => todo!(),
                // WindowEvent::Ime(_) => todo!(),
                // WindowEvent::CursorMoved { device_id, position } => todo!(),
                // WindowEvent::CursorEntered { device_id } => todo!(),
                // WindowEvent::CursorLeft { device_id } => todo!(),
                // WindowEvent::MouseWheel { device_id, delta, phase } => todo!(),
                // WindowEvent::MouseInput { device_id, state, button } => todo!(),
                // WindowEvent::TouchpadMagnify { device_id, delta, phase } => todo!(),
                // WindowEvent::SmartMagnify { device_id } => todo!(),
                // WindowEvent::TouchpadRotate { device_id, delta, phase } => todo!(),
                // WindowEvent::TouchpadPressure { device_id, pressure, stage } => todo!(),
                // WindowEvent::AxisMotion { device_id, axis, value } => todo!(),
                // WindowEvent::Touch(_) => todo!(),
                // WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer } => todo!(),
                // WindowEvent::ThemeChanged(_) => todo!(),
                // WindowEvent::Occluded(_) => todo!(),
                WindowEvent::RedrawRequested => {
                    frame_counter.update();
                    // let frame = surface.acquire(&context);
                    // let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
                    //     format: Some(surface.config().view_formats[0]),
                    //     ..wgpu::TextureViewDescriptor::default()
                    // });

                    // frame.present();

                    window_loop.window.request_redraw();
                },
                _ => {}
            },
            // Event::DeviceEvent { device_id, event } => todo!(),
            // Event::UserEvent(_) => todo!(),
            // Event::Suspended => todo!(),
            // Event::Resumed => todo!(),
            // Event::AboutToWait => todo!(),
            // Event::LoopExiting => todo!(),
            // Event::MemoryWarning => todo!(),
            _ => {}
        }
    });
}
