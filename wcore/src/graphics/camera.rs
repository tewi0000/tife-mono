
use cgmath::{Matrix4, Vector3, Rad, InnerSpace};

pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub trait Transformation {
    fn apply(&self) -> Matrix4<f32>;
}

/* Camera 2D */
pub struct Camera2D {
    pub position : Vector3<f32>,
}

impl Camera2D {
    pub fn new<V: Into<Vector3<f32>>>(position: V) -> Self {
        return Self {
            position: position.into(),
        };
    }
}

impl Transformation for Camera2D {
    fn apply(&self) -> Matrix4<f32> {
        return Matrix4::from_translation(self.position);
    }
}

/* Camera 3D */
pub struct Camera3D {
    pub position : Vector3<f32>,
    pub yaw      : Rad<f32>,
    pub pitch    : Rad<f32>,
}

impl Camera3D {
    pub fn new<V: Into<Vector3<f32>>, Y: Into<Rad<f32>>, P: Into<Rad<f32>>>(position: V, yaw: Y, pitch: P) -> Self {
        return Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        };
    }
}

impl Transformation for Camera3D {
    fn apply(&self) -> Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        return Matrix4::look_to_rh( // This is sad.
            (self.position.x, self.position.y, self.position.z).into(),
            Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            Vector3::unit_y(),
        );
    }
}

/* Generic porojection matrix */
pub trait Projection {
    fn resize(&mut self, width: u32, height: u32);
}

/* Perspective projection matrix */
pub struct ProjectionPerspective {
    aspect: f32,
    fovy: Rad<f32>,
    znear: f32,
    zfar: f32,
}

impl ProjectionPerspective {
    pub fn new<F: Into<Rad<f32>>>(width: u32, height: u32, fovy: F, znear: f32, zfar: f32) -> Self {
        return Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear,
            zfar,
        };
    }
}

impl Projection for ProjectionPerspective {
    fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }
}

impl Transformation for ProjectionPerspective {
    fn apply(&self) -> Matrix4<f32> {
        return cgmath::perspective(self.fovy, self.aspect, self.znear, self.zfar);
    }
}

/* Orthographic projection matrix */
pub struct ProjectionOrthographic {
    width: f32,
    height: f32,
    znear: f32,
    zfar: f32,
}

impl ProjectionOrthographic {
    pub fn new(width: u32, height: u32, znear: f32, zfar: f32) -> Self {
        return Self {
            width: width as f32,
            height: height as f32,
            znear,
            zfar,
        };
    }
}

impl Projection for ProjectionOrthographic {
    fn resize(&mut self, width: u32, height: u32) {
        self.width = width as f32;
        self.height = height as f32;
    }
}

impl Transformation for ProjectionOrthographic {
    fn apply(&self) -> Matrix4<f32> {
        return cgmath::ortho(0.0, self.width, self.height, 0.0, self.znear, self.zfar);
    }
}