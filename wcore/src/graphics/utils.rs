use wgpu::{Device, TextureView};

pub fn render_pass<'a>(encoder: &'a mut wgpu::CommandEncoder, view: &'a wgpu::TextureView, depth_buffer: Option<&'a TextureView>) -> wgpu::RenderPass<'a> {
    return encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[
            // [[location(0)]] in the fragment shader
            Some(wgpu::RenderPassColorAttachment {
                resolve_target: None,
                view: view,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(
                        wgpu::Color {
                            r: 0.01,
                            g: 0.01,
                            b: 0.01,
                            a: 1.0,
                        }
                    ),
                    store: true,
                }
            })
        ],
        depth_stencil_attachment: if depth_buffer.is_some() { Some(wgpu::RenderPassDepthStencilAttachment {
            view: depth_buffer.unwrap(),
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: true,
            }),
            stencil_ops: None,
        }) } else { None },
    });
}

pub fn render<'a: 'r, 'r>(encoder      : &'a mut wgpu::CommandEncoder,
                          view         : &'a wgpu::TextureView,
                          depth_buffer : Option<&'a TextureView>,
                          lambda       : impl FnOnce(wgpu::RenderPass<'r>) -> ()) {
    #[allow(unused_mut)]
    let mut pass = render_pass(encoder, view, depth_buffer);
    lambda(pass);
}

pub fn submit(queue: &wgpu::Queue, device: &wgpu::Device, lambda: impl FnOnce(&mut wgpu::CommandEncoder)) {
    let descriptor = wgpu::CommandEncoderDescriptor { label: Some("Render Encoder") };
    let mut encoder = device.create_command_encoder(&descriptor);
    lambda(&mut encoder);

    queue.submit(std::iter::once(encoder.finish()));
}

pub fn pipeline(device  : &Device,
                shader  : &wgpu::ShaderModule,
                config  : &wgpu::SurfaceConfiguration,
                groups  : &[&wgpu::BindGroupLayout],
                buffers : &[wgpu::VertexBufferLayout],
                depth   : bool) -> wgpu::RenderPipeline {
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label                : Some("Render Pipeline Layout"),
        push_constant_ranges : &[],
        bind_group_layouts   : groups,
    });

    return device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label  : Some("Render Pipeline"),
        layout : Some(&pipeline_layout),

        // Shaders
        vertex: wgpu::VertexState {
            module      : &shader,
            entry_point : "vertex_main",
            buffers     : buffers,
        },

        fragment: Some(wgpu::FragmentState {
            module      : &shader,
            entry_point : "fragment_main",
            targets     : &[Some(wgpu::ColorTargetState {
                format     : config.format,
                blend      : Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask : wgpu::ColorWrites::ALL,
            })],
        }),

        // Other
        primitive: wgpu::PrimitiveState {
            topology           : wgpu::PrimitiveTopology::TriangleList,
            strip_index_format : None,
            front_face         : wgpu::FrontFace::Ccw,
            cull_mode          : Some(wgpu::Face::Back),
            polygon_mode       : wgpu::PolygonMode::Fill, // Other modes require certain features
            unclipped_depth    : false,                   // Requires Features::DEPTH_CLIP_CONTROL
            conservative       : false,                   // Requires Features::CONSERVATIVE_RASTERIZATION
        },

        depth_stencil: if depth { Some(wgpu::DepthStencilState {
            format              : wgpu::TextureFormat::Depth32Float,
            depth_write_enabled : true,
            depth_compare       : wgpu::CompareFunction::Less,
            stencil             : wgpu::StencilState::default(),
            bias                : wgpu::DepthBiasState::default(),
        }) } else { None },
        
        multiview   : None,
        multisample : wgpu::MultisampleState {
            count                     : 1,
            mask                      : !0,
            alpha_to_coverage_enabled : false,
        },
    });
}

pub fn bind<'a, 'b: 'a>(bind_group: &'b wgpu::BindGroup, render_pass: &mut wgpu::RenderPass<'a>, index: u32) {
    render_pass.set_bind_group(index, bind_group, &[]);
}