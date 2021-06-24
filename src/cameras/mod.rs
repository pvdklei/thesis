pub mod inspector;
pub use inspector::Inspector;
pub mod fly;
pub use fly::Flying;

/// Generic camera/eye, so that updates can be done using trait object
/// instead of conditional checking which camera is in use.
pub trait Eye {
    fn position(&self) -> glm::Vec3;
    fn view_projection(&self) -> glm::Mat4 {
        self.projection() * self.view()
    }
    fn view(&self) -> glm::Mat4;
    fn projection(&self) -> glm::Mat4;
    fn update(&mut self, _input: &pgl::window::GlfwWindow, _dt: f64) {}
}
