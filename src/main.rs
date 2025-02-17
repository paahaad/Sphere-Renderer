use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};
use glam::Vec3;
use std::sync::Arc;

mod renderer;
use renderer::{Camera, SphereRenderer, Sphere};

fn create_test_spheres() -> Vec<Sphere> {
    let mut spheres = Vec::new();
    let mut rng = rand::thread_rng();

    for i in 0..1_000_000 {
        let sphere = Sphere {
            position: [
                rand::random::<f32>() * 200.0 - 100.0,
                rand::random::<f32>() * 200.0 - 100.0,
                rand::random::<f32>() * 200.0 - 100.0,
            ],
            radius: 0.5,
            material_index: (i % 100) as u32,
            _padding: [0; 3],
        };
        spheres.push(sphere);
    }
    spheres
}

async fn run(event_loop: EventLoop<()>, window: Window) {
    let size = window.inner_size();

    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        flags: wgpu::InstanceFlags::default(),
        backend_options: wgpu::BackendOptions::default(),
    });

    let surface = unsafe { instance.create_surface(&window) }.unwrap();
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }).await.unwrap();

    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: Default::default(),
        },
        None,
    ).await.unwrap();

    let device = Arc::new(device);
    let queue = Arc::new(queue);

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps.formats.iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_caps.formats[0]);

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &config);

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, -50.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let mut renderer = SphereRenderer::new(
        device.clone(),
        queue.clone(),
        &config,
    );

    let spheres = create_test_spheres();
    renderer.update_sphere_data(&spheres);

    event_loop.run(|event, target| {
        match event {
            Event::WindowEvent { window_id, event } => {
                if window_id == window.id() {
                    match event {
                        WindowEvent::CloseRequested => target.exit(),
                        WindowEvent::KeyboardInput { 
                            event: KeyEvent { 
                                physical_key: PhysicalKey::Code(keycode),
                                state: ElementState::Pressed,
                                ..
                            },
                            ..
                        } => {
                            match keycode {
                                KeyCode::KeyW => camera.move_forward(1.0),
                                KeyCode::KeyS => camera.move_forward(-1.0),
                                KeyCode::KeyA => camera.move_right(-1.0),
                                KeyCode::KeyD => camera.move_right(1.0),
                                _ => (),
                            }
                        }
                        WindowEvent::Resized(new_size) => {
                            if new_size.width > 0 && new_size.height > 0 {
                                config.width = new_size.width;
                                config.height = new_size.height;
                                surface.configure(&device, &config);
                                camera.update_aspect(new_size.width as f32 / new_size.height as f32);
                            }
                        }
                        _ => (),
                    }
                }
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            Event::WindowEvent { 
                window_id,
                event: WindowEvent::RedrawRequested,
            } if window_id == window.id() => {
                match surface.get_current_texture() {
                    Ok(frame) => {
                        renderer.render(
                            camera.view_matrix(),
                            camera.projection_matrix(),
                            camera.position(),
                        );
                        frame.present();
                    }
                    Err(wgpu::SurfaceError::Lost) => {
                        surface.configure(&device, &config);
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => target.exit(),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            _ => (),
        }
    }).unwrap();
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("PBR Spheres")
        .with_inner_size(winit::dpi::PhysicalSize::new(1280, 720))
        .build(&event_loop)
        .unwrap();
    
    pollster::block_on(run(event_loop, window));
}