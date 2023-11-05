//Import crates
use std::ptr;
use vulkano::{
    instance::{Instance, InstanceCreateInfo, InstanceExtensions},
    swapchain::Surface,
    Version, VulkanLibrary,
};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

fn main() {
    //Build the event loop
    //let event_loop = EventLoop::new();
    let _instance = {
        //Create library
        let vulk_library = VulkanLibrary::new()
            .unwrap_or_else(|err| panic!("Couldn't load Vulkan Library: {:?}", err));
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
            vulk_library,
            InstanceCreateInfo {
                enabled_extensions: vulk_extensions,
                ..Default::default()
                },
        ).unwrap_or_else(|err| panic! ("Could not create instance: {:?}", err));
    };

    let window = build_window();
    
    

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
    });*/
    println!("made it bois");
}
