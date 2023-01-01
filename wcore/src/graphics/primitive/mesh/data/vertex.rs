use cgmath::{Vector3, Vector2, vec3, vec2};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub pos : Vector3<f32>,
    pub uv  : Vector2<f32>,
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub fn describe<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride : mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode    : wgpu::VertexStepMode::Vertex,
            attributes   : &Self::ATTRIBUTES,
        }
    }

    pub fn vertices_rect(min: f32, max: f32) -> Vec<Self> {
        return vec![ 
            Vertex { pos: vec3(min, min, 1.0), uv: vec2(0.0, 0.0) },
            Vertex { pos: vec3(min, max, 1.0), uv: vec2(0.0, 1.0) },
            Vertex { pos: vec3(max, max, 1.0), uv: vec2(1.0, 1.0) },
            Vertex { pos: vec3(max, max, 1.0), uv: vec2(1.0, 1.0) },
            Vertex { pos: vec3(max, min, 1.0), uv: vec2(1.0, 0.0) },
            Vertex { pos: vec3(min, min, 1.0), uv: vec2(0.0, 0.0) },
        ];
    }
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}