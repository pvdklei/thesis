use crate::material::Material;
use crate::{cameras::Eye, lights};
use pgl::buffer::{Buffer, BufferType, DrawType};
use pgl::shader::ShaderProgram;
use pgl::window;
use std::cell::RefCell;
use std::rc::Rc;

/// Methods that every shaders in this file should have in common.
pub trait Shader {
    fn bind(&self);
    // Sets the model matrix or transform of the shader.
    fn set_model(&self, model: &glm::Mat4);
    fn set_material(&self, material: &Material);
}

impl<T> Shader for T
where
    T: std::ops::Deref<Target = RefCell<ShaderProgram>>,
{
    fn bind(&self) {
        self.borrow().bind()
    }
    fn set_model(&self, model: &glm::Mat4) {
        let mut shader = self.borrow_mut();
        shader.set_mat4fs("uModel", std::slice::from_ref(model));
    }
    fn set_material(&self, m: &Material) {
        let mut shader = self.borrow_mut();
        shader.set_float("uMaterial.ambient", m.ambient);
        shader.set_float("uMaterial.specular", m.specular);
        shader.set_int("uMaterial.reflectiveness", m.reflectiveness);
        shader.set_vec4fs("uMaterial.albedo", std::slice::from_ref(&m.albedo));
    }
}

macro_rules! impl_deref_shader {
    ($name:ty) => {
        impl std::ops::Deref for $name {
            type Target = RefCell<ShaderProgram>;
            fn deref(&self) -> &Self::Target {
                &self.s
            }
        }
    };
}
impl_deref_shader!(NormalMapping);
impl_deref_shader!(Ui);
impl_deref_shader!(Flat);
impl_deref_shader!(Textured);
impl_deref_shader!(NormalAlbedoMapping);

const MAX_POINT_LIGHTS: usize = 1;

/// The uniform buffer that holds common data used in every shader.
pub struct AppUniforms {
    buffer: Buffer,
    pub data: AppUniformsData,
}

/// The actual data send to the GPU
#[repr(C)]
pub struct AppUniformsData {
    pub view_projection: glm::Mat4,
    pub view: glm::Mat4,
    pub projection: glm::Mat4,
    pub ortho: glm::Mat4,
    pub eye_position: glm::Vec3,
    _padding1: [i8; 4],
    pub point_lights: [lights::PointLight; MAX_POINT_LIGHTS],
    // std140
}

impl AppUniforms {
    pub fn new() -> Self {
        let buffer = Buffer::new(BufferType::Uniform, DrawType::Dynamic);
        buffer.bind();
        buffer.init(std::mem::size_of::<AppUniformsData>());
        buffer.set_binding(0);
        buffer.unbind();
        Self {
            data: AppUniformsData {
                view_projection: glm::Mat4::identity(),
                view: glm::Mat4::identity(),
                projection: glm::Mat4::identity(),
                ortho: glm::Mat4::identity(),
                eye_position: glm::Vec3::zeros(),
                _padding1: [0; 4],
                point_lights: [lights::PointLight::new([0., 0., 0.], [1., 1., 1.]);
                    MAX_POINT_LIGHTS],
            },
            buffer,
        }
    }
    pub fn set_ubo(&self) {
        self.buffer.bind();
        self.buffer.subbuffer(std::slice::from_ref(&self.data), 0);
    }

    pub fn update(
        &mut self,
        eye: &dyn Eye,
        win: &window::GlfwWindow,
        light_pos: [f32; 3],
        light_color: [f32; 3],
    ) {
        let u = &mut self.data;
        u.eye_position = eye.position();
        u.view_projection = eye.view_projection();
        u.projection = eye.projection();
        u.view = eye.view();
        const ORTHO_RAD: f32 = 4.;
        let aspect = win.aspect();
        u.ortho = glm::ortho(
            -ORTHO_RAD,
            ORTHO_RAD,
            -ORTHO_RAD / aspect,
            ORTHO_RAD / aspect,
            -100.,
            100.,
        );
        u.point_lights[0].position = light_pos;
        u.point_lights[0].color = light_color;
    }
}

// ALL SHADERS
//
// These are basically compisition of the ShaderProgram struct,
// each having their own personal functions for setting uniforms.

#[derive(Clone)]
pub struct NormalMapping {
    s: Rc<RefCell<ShaderProgram>>,
}

impl NormalMapping {
    pub fn new(p: impl AsRef<std::path::Path>) -> Self {
        let s = ShaderProgram::from_path(p, Default::default()).unwrap();
        s.bind();
        s.bind_uniform_block("App", 0);
        Self {
            s: Rc::new(RefCell::new(s)),
        }
    }
    pub fn set_uniforms(&mut self, normal_map: i32) {
        let mut s = self.s.borrow_mut();
        s.set_int("uNormalMap", normal_map);
    }
}

#[derive(Clone)]
pub struct NormalAlbedoMapping {
    s: Rc<RefCell<ShaderProgram>>,
}

impl NormalAlbedoMapping {
    pub fn new() -> Self {
        let s = ShaderProgram::from_path("shaders/nm_tex.glsl", Default::default()).unwrap();
        s.bind();
        s.bind_uniform_block("App", 0);
        Self {
            s: Rc::new(RefCell::new(s)),
        }
    }
    pub fn set_uniforms(&mut self, normal_map: i32, albedo_map: i32) {
        let mut s = self.s.borrow_mut();
        s.set_int("uNormalMap", normal_map);
        s.set_int("uAlbedoMap", albedo_map);
    }
}

#[derive(Clone)]
pub struct Textured {
    s: Rc<RefCell<ShaderProgram>>,
}

impl Textured {
    pub fn new() -> Self {
        //let s =
        //ShaderProgram::from_frag_and_vert_path("shaders/tex.frag", "shaders/tex.vert").unwrap();
        let s = ShaderProgram::from_path("shaders/tex.glsl", Default::default()).unwrap();
        s.bind();
        s.bind_uniform_block("App", 0);
        Self {
            s: Rc::new(RefCell::new(s)),
        }
    }
    pub fn set_uniforms(&mut self, albedo_map: i32) {
        let mut s = self.s.borrow_mut();
        s.set_int("uAlbedoMap", albedo_map);
    }
}

#[derive(Clone)]
pub struct Flat {
    s: Rc<RefCell<ShaderProgram>>,
}

impl Flat {
    pub fn new() -> Self {
        let s = ShaderProgram::from_path("shaders/flat.glsl", Default::default()).unwrap();
        s.bind();
        s.bind_uniform_block("App", 0);
        Self {
            s: Rc::new(RefCell::new(s)),
        }
    }

    pub fn set_uniforms(&mut self, albedo: &glm::Vec4) {
        let mut s = self.s.borrow_mut();
        s.bind();
        s.set_vec4fs("uMaterial.albedo", std::slice::from_ref(albedo));
    }
}

#[derive(Clone)]
pub struct Ui {
    s: Rc<RefCell<ShaderProgram>>,
}

impl Ui {
    pub fn new() -> Self {
        let s = ShaderProgram::from_path("shaders/ui.glsl", Default::default()).unwrap();
        Self {
            s: Rc::new(RefCell::new(s)),
        }
    }
    pub fn set_uniforms(&mut self, transform: &glm::Mat4) {
        let mut s = self.s.borrow_mut();
        s.set_mat4fs("uTransform", std::slice::from_ref(transform));
    }
}
