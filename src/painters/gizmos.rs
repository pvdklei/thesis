use crate::vertices::PosNorm;
use pgl::vao::VertexArray;

pub struct Gizmos {
    vao: VertexArray,
    shader: pgl::shader::ShaderProgram,
    n_faces: usize,
}

impl Gizmos {
    pub fn new() -> Self {
        let mut vao = VertexArray::new_static();
        let (v, i) = pgeom::arrow(0.4, 4.);
        let v = v
            .iter()
            .map(|v| {
                let mut pos = v.position;
                pos[1] += 1.;
                PosNorm {
                    position: pos,
                    normal: v.normal,
                }
            })
            .collect::<Vec<_>>();
        vao.bind();
        vao.buffer_indices(&i);
        vao.new_vertex_buffer_filled("all", &v);
        VertexArray::unbind();
        Self {
            vao,
            shader: pgl::shader::ShaderProgram::from_path(
                "shaders/gizmos.glsl",
                Default::default(),
            )
            .unwrap(),
            n_faces: i.len(),
        }
    }

    pub fn draw(&mut self) {
        let (x, y, w, h) = pgl::utils::gl::viewport_info();
        const W_FRAC: f32 = 0.23;
        let new_width = (W_FRAC * w as f32) as usize;
        let new_height = (W_FRAC * h as f32) as usize;
        let (xa, ya, wa, ha) = (w - new_width, h - new_height, new_width, new_height);

        pgl::utils::gl::viewport(xa, ya, wa, ha);

        self.vao.bind();
        self.shader.bind();

        const SCALE: f32 = 0.9;

        let mut model = glm::scaling::<f32>(&glm::Vec3::new(SCALE, SCALE, SCALE));
        self.shader
            .set_vec4fs("uMaterial.albedo", &[[0.0f32, 1., 0., 1.]]);
        self.shader.set_mat4fs("uModel", &[model]);
        pgl::utils::gl::draw(self.n_faces * 3);

        model.swap_columns(0, 1);
        self.shader
            .set_vec4fs("uMaterial.albedo", &[[1.0f32, 0., 0., 1.]]);
        self.shader.set_mat4fs("uModel", &[model]);
        pgl::utils::gl::draw(self.n_faces * 3);

        model.swap_columns(0, 1);
        model.swap_columns(1, 2);
        self.shader
            .set_vec4fs("uMaterial.albedo", &[[0.0f32, 0., 1., 1.]]);
        self.shader.set_mat4fs("uModel", &[model]);
        pgl::utils::gl::draw(self.n_faces * 3);

        pgl::utils::gl::viewport(x, y, w, h);
    }
}
