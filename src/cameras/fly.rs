
/// Free moving/flying camera, but still has some issues. 
/// After flying and turning a bit the view gets tilted.
pub struct Flying {
    projection: glm::Mat4,
    view: glm::Mat4,
    view_projection: glm::Mat4,
    pos: glm::Vec3,
    horizontal_angle: f32,
    vertical_angle: f32,
    view_dir: glm::Vec3,
    up_dir: glm::Vec3,
}

impl Flying {
    const Y_SENSITIVITY: f32 = 2.4;
    const X_SENSITIVITY: f32 = 2.4;
    const STRAFE_SENSITIVITY: f32 = 4.0;
    const FORWARD_SENSITIVITY: f32 = 6.0;

    pub fn new() -> Self {
        Self {
            projection: glm::Mat4::identity(),
            view: glm::Mat4::identity(),
            view_projection: glm::Mat4::identity(),
            pos: [0., 0., 4.].into(),
            vertical_angle: 0.0,
            horizontal_angle: 0.0,
            view_dir: -glm::Vec3::z(),
            up_dir: glm::Vec3::y(),
        }
    }

    fn update_directions(&mut self) {
        let rot_vertical = glm::rotation(self.vertical_angle, &glm::Vec3::x());
        let rotation = glm::rotate(&rot_vertical, self.horizontal_angle, &glm::Vec3::y());
        self.up_dir = (rotation * glm::Vec4::y()).xyz().normalize();
        self.view_dir = (rotation * -glm::Vec4::z()).xyz().normalize();
    }
}

impl super::Eye for Flying {
    fn position(&self) -> glm::Vec3 {
        self.pos
    }
    fn projection(&self) -> glm::Mat4 {
        self.projection
    }
    fn view_projection(&self) -> glm::Mat4 {
        self.view_projection
    }
    fn view(&self) -> glm::Mat4 {
        self.view
    }
    fn update(&mut self, input: &pgl::window::GlfwWindow, dt: f64) {
        use pgl::window::Key;
        let w = input.is_key_pressed(Key::W);
        let s = input.is_key_pressed(Key::S);
        let a = input.is_key_pressed(Key::A);
        let d = input.is_key_pressed(Key::D);
        let up = input.is_key_pressed(Key::Up);
        let down = input.is_key_pressed(Key::Down);
        let left = input.is_key_pressed(Key::Left);
        let right = input.is_key_pressed(Key::Right);

        self.vertical_angle = (self.vertical_angle
            + speed(up, down, Self::Y_SENSITIVITY) * dt as f32)
            .clamp(glm::pi::<f32>() * -0.45, glm::pi::<f32>() * 0.45);
        self.horizontal_angle += speed(left, right, Self::X_SENSITIVITY) * dt as f32;

        self.update_directions();

        use crate::utils::speed;
        let strafe_speed = speed(d, a, Self::STRAFE_SENSITIVITY);
        let forward_speed = speed(w, s, Self::FORWARD_SENSITIVITY);

        let strafe_dir = glm::cross(&self.view_dir, &self.up_dir);
        self.pos +=
            strafe_speed * strafe_dir * dt as f32 + forward_speed * self.view_dir * dt as f32;

        self.projection = glm::perspective(input.aspect(), glm::pi::<f32>() * 0.25, 0.01, 200.);
        self.view = glm::look_at(&self.pos, &(self.pos + self.view_dir), &self.up_dir);
        self.view_projection = self.projection * self.view;
    }
}
