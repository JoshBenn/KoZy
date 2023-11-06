//Import crates
use std::{collections::HashMap, error::Error, sync::Arc};
use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, 
        AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo,
    },
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, 
        DeviceExtensions, QueueCreateInfo, Queue
    },
    image::{view::ImageView, Image, ImageUsage},
    instance::{Instance, InstanceCreateInfo, InstanceExtensions},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    pipeline::{
        graphics::{
            color_blend::{ColorBlendAttachmentState, ColorBlendState},
            input_assembly::InputAssemblyState,
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition},
            viewport::{Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        DynamicState, GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    sync::{self, GpuFuture},
    swapchain::{
        acquire_next_image, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo
    },
    Validated, Version, VulkanError, VulkanLibrary,
};

use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

struct WindowSurface {
    window: Arc<Window>,
    Swapchain: Arc<Swapchain>,
    framebuffers: Vec<Arc<Framebuffer>>,
    recreate_swapchain: bool,
    previous_frame_end: Option<Box<dyn GpuFuture>>,
}

fn main() -> Result<(), impl Error> {
/*    //Create the instance
    let event_loop = EventLoop::new().unwrap();
    
    //Create a new instance
    let instance = {
        //Create library
        let vulk_library = VulkanLibrary::new().unwrap();

        //let _vulk_extensions = Surface::required_extensions(&_event_loop).unwrap();
        //Create extensions
        let vulk_extensions = InstanceExtensions{
            khr_surface: true,            //Required default for drawing
            khr_xlib_surface: true,       //For x11 Library
            khr_wayland_surface: true,    //For wayland library
            //khr_android_surface: true,   //For android
            //khr_win32_surface: true,     //For win32
            .. InstanceExtensions::empty()
        };
        
        Instance::new(
            _vulk_library,
            InstanceCreateInfo {
                enabled_extensions: vulk_extensions,
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY, //For macOS
                max_api_version: Some(Version::V1_1),
                ..Default::default()
                },
        ).unwrap_or_else(|err| panic! ("Could not create instance: {:?}", err));
    };

    //Create the event loop
    let window = Arc::new(WindowBuilder::new().build(&event_loop).unwrap());
    let surface = Surface::from_window(instance.clone(), window.clone()).unwrap();

    //ControlFlow changes window behavior:
    // -- Wait = Wait for events, Poll = continuous
    event_loop.set_control_flow(ControlFlow::Wait);

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Close requested, stopping program!");
                elwt.exit();
            },
            Event::AboutToWait => {
                //Redraw request event
                //--Call when redraw or if continuously redraw
                window.request_redraw();
            },
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                //Redraw application
                //--Call when OS request made
            },
            _ => ()
        }
    });
    
    //Create the window
    //let window = build_window();
    //create the surface
    /*let _surface = unsafe {
        Surface::from_xlib(
            instance.clone(),
            window.hwnd(),
            Some(window),
        ).unwrap()
    };*/

    
    //Create swapchain
    /*let ext = DeviceExtensions { //Enable swapchain on device (not instance)
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };*/

    //Get information from the device
    /*let surface_capabilities = device
        .physical_device()
        .surface_capabilities(&_surface, Default::default())?;

    //double buffer
    let min_image_count = match surface_capabilities.max_image_count {
        None => max(2, surface-capabilities.min_image_count),
        Some(limit) => min(max(2, surface_capabilities.min_image_count), limit)
    };*/

    //preserve current surface transform
    /*let pre_transform = surface_capabilities.current_transform;

    //Use first available format
    let (image_format, color_space) = device
        .physical_device()
        .surface_formats(&_suface, Default::default())?[0];

    //New swapchain
    let (swapchain, images) = Swapchain::new(
        device,
        surface,
        SwapchainCreateInfo {
            min_image_count,
            image_format,
            image_extent,
            image_usage: ImageUsage::COLOR_ATTACHMENT,
            pre_transform,
            composite_alpha,
            present_mode,
            full_screen_exclusive,
            ..Default::default()
        }
    )?;*/

    
    

    //Define vulkan
    /*let app_info = ApplicationInfo {
        application_name: Some("KoZy GUI".into()),
        application_version: Some(Version {major: 0, minor: 1, patch: 0}),
        engine_name: Some("No Engine".into()),
        engine_version: Some(Version {major: 0, minor: 1, patch: 0}),
    };*/

    //Extensions definition
    //let required_extensions = InstanceExtensions::none();
    
    //Create Vulkan Instance
    /*let instance = match Instance::new(Some(&app_info), &required_extensions, None) {
        Ok(i) => i,
        Err(err) => panic!("ERROR: Failed to create Vulkan Instance: {}", err),
    };*/

    //Choose GPU
    /*let gpu = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("No device available");
    println!("Using: {} (type: {:?})", gpu.name(), gpu.ty());*/
    
    //Build the window
    /*let window = WindowBuilder::new()
        .with_title("KoZy GUI")
        .build_vk_surface(&event_loop, instance.clone())
        .expect("ERROR: Failed to create Vulkan window");*/

    //Run event loop
    /*event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait; //ControlFlow::Poll
    
        match event {
            //Close window when close event
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            //Any other event
            _=>(),
        }
 */   });*/
    println!("made it bois");
}
