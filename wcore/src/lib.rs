pub mod graphics;

use std::time::Instant;

use graphics::{context::GraphicsContext, screen::Screen};
use winit::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, dpi::{LogicalSize, PhysicalSize}, event::{Event, WindowEvent}};

pub struct AppConfig {
    pub width  : u32,
    pub height : u32,
    pub title  : String,
}

impl Default for AppConfig {
    fn default() -> Self {
        return Self {
            width  : 1200,
            height : 800,
            title  : String::from("App"),
        };
    }
}

pub struct App<State> {
    pub screens: Vec<Box<dyn Screen<State>>>
}

impl<State> Default for App<State> {
    fn default() -> Self {
        return Self {
            screens: vec![],
        };
    }
}

impl<State: 'static> App<State> {
    pub fn run(mut self, config: AppConfig, state_lambda: impl FnOnce(&mut GraphicsContext) -> State, screens_lambda: impl FnOnce(&mut GraphicsContext, &mut Vec<Box<dyn Screen<State>>>)) {
        pollster::block_on(async {
            let event_loop = EventLoop::new();
            let window = WindowBuilder::new()
                .with_title(config.title)
                .with_inner_size(LogicalSize::new(config.width, config.height))
                .build(&event_loop)
                .unwrap();

            let mut graphics = GraphicsContext::new(&window).await.unwrap();
            let mut state = state_lambda(&mut graphics);
            screens_lambda(&mut graphics, &mut self.screens);

            event_loop.run(move |event, _, control_flow| {
                // control_flow.set_poll();

                match event {
                    Event::MainEventsCleared => {
                        window.request_redraw();
                    }

                    Event::RedrawRequested(window_id) if window_id == window.id() => {
                        // let now = Instant::now();
                        if let Ok(surface_texture) = graphics.surface.get_current_texture() {
                            let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
                
                            for screen in &mut self.screens {
                                screen.render(&mut state, &view, &mut graphics);
                            }
                
                            surface_texture.present();
                        }
                    }

                    Event::WindowEvent { event, window_id } if window_id == window.id() => {
                        match event {
                            WindowEvent::CursorMoved { device_id, position, .. } => {
                            }

                            WindowEvent::MouseInput { device_id, state: button_state, button, .. } => {
                            }

                            WindowEvent::KeyboardInput { device_id, input, is_synthetic } => {
                            }

                            WindowEvent::ModifiersChanged(modifiers) => {
                            }

                            WindowEvent::Resized(PhysicalSize::<u32> { width, height }) => {
                                if width > 0 && height > 0 {
                                    graphics.surface_configuration.width = width as u32;
                                    graphics.surface_configuration.height = height as u32;
                                    graphics.surface.configure(&graphics.device, &graphics.surface_configuration);
                        
                                    for screen in &mut self.screens {
                                        screen.resize(&mut state, width, height, &mut graphics);
                                    }
                                }
                                
                            }

                            WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size  } => {
                            }

                            WindowEvent::CloseRequested => {
                                *control_flow = ControlFlow::Exit;
                            }

                            _ => {}
                        }
                    }

                    _ => {}
                }
            });
        });
    }
}