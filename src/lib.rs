use std::marker::PhantomData;

use winit::{dpi::PhysicalSize, window::Window};

pub mod core;
pub mod line;

pub struct Context {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,

    surface_config: wgpu::SurfaceConfiguration,
    surface_format: wgpu::TextureFormat,
    surface_caps: wgpu::SurfaceCapabilities,
}

impl Context {
    pub fn blocked_new(window: &Window) -> Context {
        pollster::block_on(Context::async_new(window))
    }
    pub async fn async_new(window: &Window) -> Context {
        let size = window.inner_size();
        
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(&window).unwrap() };

        let adapter =
        instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface)
        }).await.unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            limits: wgpu::Limits::default(),
            features: wgpu::Features::empty(),
        }, None).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &surface_config);
        
        Context {
            surface,
            device,
            queue,

            surface_config,
            surface_format,
            surface_caps
        }
    }
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 1 && new_size.height > 1 {
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config)
        }
    }
    pub fn wgpu_surface(&self) -> &wgpu::Surface {
        &self.surface
    }
    pub fn wgpu_device(&self) -> &wgpu::Device {
        &self.device
    }
    pub fn wgpu_queue(&self) -> &wgpu::Queue {
        &self.queue
    }
    pub fn wgpu_surface_format(&self) -> &wgpu::TextureFormat {
        &self.surface_format
    }

}