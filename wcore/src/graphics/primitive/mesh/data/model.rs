use bytemuck::{Zeroable, Pod};

use crate::graphics::primitive::mesh::instanced::Instance;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ModelRaw {
    model: [[f32; 4]; 4],
    color: [f32; 4]
}

impl ModelRaw {
    pub fn describe<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

unsafe impl Zeroable for ModelRaw { }
unsafe impl Pod for ModelRaw { }

pub struct Model {
    pub position : cgmath::Vector3<f32>,
    pub rotation : cgmath::Quaternion<f32>,
    pub scale    : cgmath::Vector3<f32>,
    pub color    : cgmath::Vector4<f32>,
}

impl Instance<ModelRaw> for Model {
    fn to_raw(&self) -> ModelRaw {
        return ModelRaw {
            color: self.color.into(),
            model: (cgmath::Matrix4::from_translation(self.position)
                  * cgmath::Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
                  * cgmath::Matrix4::from(self.rotation)).into(),
        };
    }
}