pub struct Material {
    pub albedo: [f32; 4],
    pub ambient: f32,
    pub specular: f32,
    pub reflectiveness: i32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            albedo: [0.5, 0.2, 0.9, 1.],
            ambient: 0.5,
            specular: 0.5,
            reflectiveness: 32,
        }
    }
}
