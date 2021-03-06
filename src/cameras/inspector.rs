use pgl::window::Key;

/// Camera that can rotate around the origin, and always looks
/// at the origin. Y-axis is up, always. Longitude and latitude
/// can be altered using the AWSD keys, and distance using the
/// up and down keys
pub struct Inspector {
    projection: glm::Mat4,
    view: glm::Mat4,
    view_projection: glm::Mat4,
    distance_from_origin: f32,
    longitude: f32,
    latitude: f32,
}

impl Default for Inspector {
    fn default() -> Self {
        Self::new(4., 0., 0.)
    }
}

impl Inspector {
    const X_SENSITIVITY: f32 = 3.0;
    const Y_SENSITIVITY: f32 = 3.0;
    const DISTANCE_SENSITIVITY: f32 = 6.0;

    pub fn new(distance_from_origin: f32, longitude: f32, latitude: f32) -> Self {
        Self {
            projection: glm::Mat4::identity(),
            view: glm::Mat4::identity(),
            view_projection: glm::Mat4::identity(),
            distance_from_origin,
            longitude,
            latitude,
        }
    }
}

impl super::Eye for Inspector {
    fn projection(&self) -> glm::Mat4 {
        self.projection
    }
    fn view(&self) -> glm::Mat4 {
        self.view
    }
    fn view_projection(&self) -> glm::Mat4 {
        self.view_projection
    }

    fn position(&self) -> glm::Vec3 {
        let x = self.distance_from_origin * self.latitude.cos() * self.longitude.sin();
        let y = self.distance_from_origin * self.latitude.sin();
        let z = self.distance_from_origin * self.latitude.cos() * self.longitude.cos();
        glm::Vec3::new(x, y, z)
    }

    fn update(&mut self, input: &pgl::window::GlfwWindow, dt: f64) {
        let w = input.is_key_pressed(Key::W);
        let s = input.is_key_pressed(Key::S);
        let a = input.is_key_pressed(Key::A);
        let d = input.is_key_pressed(Key::D);
        let up = input.is_key_pressed(Key::Up);
        let down = input.is_key_pressed(Key::Down);

        use crate::utils::speed;
        let dt = dt as f32;
        let (dlong, dlat, ddis) = (
            speed(d, a, Self::X_SENSITIVITY) * dt,
            speed(w, s, Self::Y_SENSITIVITY) * dt,
            speed(up, down, Self::DISTANCE_SENSITIVITY) * dt,
        );
        self.longitude += dlong;
        self.latitude += dlat;
        self.distance_from_origin -= ddis;

        self.projection = glm::perspective(input.aspect(), glm::pi::<f32>() * 0.25, 0.01, 200.);
        self.view = glm::look_at(
            &self.position(),
            &glm::Vec3::new(0., 0., 0.),
            &glm::Vec3::new(0., 1., 0.),
        );
        self.view_projection = self.projection * self.view;
    }
}
