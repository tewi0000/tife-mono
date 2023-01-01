use winit::window::Window;

pub struct GraphicsContext {
    pub device  : wgpu::Device,
    pub surface : wgpu::Surface,
    pub queue   : wgpu::Queue,

    pub surface_configuration : wgpu::SurfaceConfiguration,
    pub scale_factor          : f64,
}

impl GraphicsContext {
    pub async fn new(window: &Window) -> Option<Self> {
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference       : wgpu::PowerPreference::HighPerformance,
                compatible_surface     : Some(&surface),
                force_fallback_adapter : false,
            },
        ).await?;

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label    : None,
                features : wgpu::Features::empty(),
                limits   : {
                    let mut limits = wgpu::Limits::default();
                    limits.max_bind_groups = 8;
                    limits 
                },
            },
            None,
        ).await.ok()?;

        let size = window.inner_size();
        let surface_configuration = wgpu::SurfaceConfiguration {
            usage        : wgpu::TextureUsages::RENDER_ATTACHMENT,
            format       : *surface.get_supported_formats(&adapter).first()?,
            width        : size.width,
            height       : size.height,
            present_mode : wgpu::PresentMode::AutoVsync,
            alpha_mode   : wgpu::CompositeAlphaMode::Auto,
        }; surface.configure(&device, &surface_configuration);

        let scale_factor = window.scale_factor();
        return Some(Self {
            surface,
            device,
            queue,

            surface_configuration,
            scale_factor,
        });
    }
}