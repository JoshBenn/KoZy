use crate::color;

pub struct Mouse {
    pub x_pos: f64,
    pub y_pos: f64,
}

pub struct Application {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub clear_color: wgpu::Color,
    pub window: winit::window::Window,
    pub render_pipeline: wgpu::RenderPipeline,

    pub mouse: Mouse,
    pub cursor_active: bool,

    pub frame_count: u32,
    pub last_frame_time: std::time::Instant,
    pub last_fps_update: std::time::Instant,
}

impl Application {
    pub async fn new(window: winit::window::Window) -> Self {
        let size = window.inner_size();

        // Instance creates Adapters and Surfaces
        // BackendBit::Primary => Vulkan, + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

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
                features: wgpu::Features::empty(),
                // Full list of limits: https://docs.rs/wgpu/latest/wgpu/struct.Limits.html
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None,
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied().find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        // Defines how the surface creates its underlying surface textures
        let config = wgpu::SurfaceConfiguration {
            // Describe how they'll be used --> RENDER_ATTACHMENT is write to the screen
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            // Define how they'll be stored on the GPU
            format: surface_format,
            // Width and Height of the surface texter in pixels (NEVER 0)
            width: size.width,
            height: size.height,
            // Use the PresentMode enum
            // -- Determines how to sync the surface with the display
            // -- Full list: https://docs.rs/wgpu/latest/wgpu/enum.PresentMode.html
            present_mode: surface_caps.present_modes[0],
            // Transparent windows
            alpha_mode: surface_caps.alpha_modes[0],
            // List of TextureFormats
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        // Link to the shader module
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // Create the render_pipeline
        let render_pipeline_laout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_laout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            // Describes how to interpret the vertices
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                // Direction the surface is facing
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                // How many samples the pipeline will use
                count: 1,
                // Which samples are active (All)
                mask: !0,
                // Anti-aliasing
                alpha_to_coverage_enabled: false,
            },
            // How many array layers the render attachments have
            multiview: None,
        });

        Self {
            instance,
            adapter,
            surface,
            device,
            queue,
            config,
            clear_color: color::Color::Grey.rgb(1.0),
            size,
            window,
            render_pipeline,

            mouse: Mouse { x_pos: 0.0, y_pos: 0.0 },
            cursor_active: false,

            frame_count: 0,
            last_frame_time: std::time::Instant::now(),
            last_fps_update: std::time::Instant::now(),
        }
    }

    pub fn window(&self) -> &winit::window::Window {
        return &self.window;
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &winit::event::WindowEvent) -> bool {
        #[allow(unused_variables, deprecated)]
        match event {
            // winit::event::WindowEvent::Resized(_) => todo!(),
            // winit::event::WindowEvent::Moved(_) => todo!(),
            // winit::event::WindowEvent::CloseRequested => todo!(),
            // winit::event::WindowEvent::Destroyed => todo!(),
            // winit::event::WindowEvent::DroppedFile(_) => todo!(),
            // winit::event::WindowEvent::HoveredFile(_) => todo!(),
            // winit::event::WindowEvent::HoveredFileCancelled => todo!(),
            // winit::event::WindowEvent::ReceivedCharacter(_) => todo!(),
            winit::event::WindowEvent::Focused(_) => return true,
            // winit::event::WindowEvent::KeyboardInput { device_id, input, is_synthetic } => todo!(),
            // winit::event::WindowEvent::ModifiersChanged(_) => todo!(),
            // winit::event::WindowEvent::Ime(_) => todo!(),
            winit::event::WindowEvent::CursorMoved { position, ..} => {
                self.mouse.x_pos = position.x;
                self.mouse.y_pos = position.y;
                // self.clear_color = wgpu::Color {
                //     r: position.x as f64 / self.size.width as f64,
                //     g: position.y as f64 / self.size.height as f64,
                //     b: 1.0,
                //     a: 1.0,
                // };
                return true;
            },
            winit::event::WindowEvent::CursorEntered { .. } => {
                self.cursor_active = true;
                return true;
            },
            winit::event::WindowEvent::CursorLeft { device_id } => {
                self.cursor_active = false;
                return false;
            },
            // winit::event::WindowEvent::MouseWheel { device_id, delta, phase, modifiers } => todo!(),
            // winit::event::WindowEvent::MouseInput { device_id, state, button, modifiers } => todo!(),
            // winit::event::WindowEvent::TouchpadMagnify { device_id, delta, phase } => todo!(),
            // winit::event::WindowEvent::SmartMagnify { device_id } => todo!(),
            // winit::event::WindowEvent::TouchpadRotate { device_id, delta, phase } => todo!(),
            // winit::event::WindowEvent::TouchpadPressure { device_id, pressure, stage } => todo!(),
            // winit::event::WindowEvent::AxisMotion { device_id, axis, value } => todo!(),
            // winit::event::WindowEvent::Touch(_) => todo!(),
            // winit::event::WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size } => todo!(),
            // winit::event::WindowEvent::ThemeChanged(_) => todo!(),
            // winit::event::WindowEvent::Occluded(_) => todo!(),
            _ => {
                self.clear_color = color::Color::Grey.rgb(1.0);
                return false;
            },
        }
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // Get a frame to render to
        let output = self.surface.get_current_texture()?;

        // Create TextureView to control how the render code interacts with the texture
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Create CommandEncoder for the commands to send to the GPU
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // FPS counter
        self.frame_count += 1;
        if self.last_fps_update.elapsed() >= std::time::Duration::from_secs(1) {
            let fps = self.frame_count;
            let title = format!("KoZy - FPS: {}", fps);
            self.window.set_title(&title);

            self.frame_count = 0;
            self.last_fps_update = std::time::Instant::now();
        }

        { // Nested block to drop the &mut encoder -- allows encoder.finish()
            // Create a RenderPass for drawing to the screen -- clear the screen
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                // Where the color is drawn to
                color_attachments: &[
                    // @location(0) targets this
                    Some(wgpu::RenderPassColorAttachment {
                        // Texture to render to 
                        view: &view,
                        // Texture that receives the resolved output
                        // -- Same as `view` unless multisampling is enabled
                        resolve_target: None,
                        // Tells wgpu what to do with the colors on the screen
                        ops: wgpu::Operations {
                            // Tells wgpu how to handle the colors stored from the previous frame
                            load: wgpu::LoadOp::Clear(self.clear_color),
                            // Tells wgpu whether to store the rendered results to the texture
                            store: wgpu::StoreOp::Store,
                        },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }
        
        // Finish the command buffer - submit to GPU render queue
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        return Ok(());
    }
}
