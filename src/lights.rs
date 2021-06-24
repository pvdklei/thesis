use crate::cameras::Eye;

use pgl::window;

/// Light that floats at the position of the camera, and 
/// is offsetable with the mouse position. So if 
/// the mouse is at the top of the screen, the lights floats 
/// a above the camera.
pub struct CameraFollowingLight {
    pub with_mouse: bool,
    pub color: [f32; 3],
}

impl CameraFollowingLight {
    pub fn new() -> Self {
        Self {
            with_mouse: true,
            color: [0.8, 0.8, 0.8],
        }
    }
    pub fn position(&self, window: &window::GlfwWindow, eye: &dyn Eye) -> glm::Vec3 {
        if !self.with_mouse {
            return eye.position();
        }

        let (cur_x, cur_y) = window.cursor_pos();
        let (win_w, win_h) = window.window_size();
        let offset_hor = cur_x / win_w as f32 - 0.5;
        let offset_vert = cur_y / win_h as f32 - 0.5;

        let eye_pos = eye.position();
        let eye_dir = -eye_pos;
        let offset_hor_dir = glm::cross(&eye_dir, &glm::Vec3::new(0., 1., 0.)).normalize();
        let offset_vert_dir = glm::cross(&eye_dir, &offset_hor_dir).normalize();

        const FACTOR: f32 = 4.;
        eye_pos + (offset_hor_dir * offset_hor + offset_vert_dir * offset_vert) * FACTOR
    }
}


/// Light data that is transferred to shaders.
#[repr(C)]
pub struct PointLight {
    pub position: [f32; 3],
    _padding0: [i8; 4],
    pub color: [f32; 3],
    _padding1: [i8; 4],
    // std140
}

impl PointLight {
    pub fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        Self {
            position,
            _padding0: [0; 4],
            color,
            _padding1: [0; 4],
        }
    }
}

#[repr(C)]
pub struct DirLight {
    pub direction: [f32; 3],
    _padding0: [i8; 4],
    pub color: [f32; 3],
    _padding1: [i8; 4],
    // std140
}

impl DirLight {
    pub fn new(direction: [f32; 3], color: [f32; 3]) -> Self {
        Self {
            direction,
            _padding0: [0; 4],
            color,
            _padding1: [0; 4],
        }
    }
}
