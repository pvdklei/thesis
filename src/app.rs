use crate::cameras::Eye;
use crate::shaders::Shader;
use crate::{cameras, imgui_widgets, lights, material, painters, shaders, time, vertices};

/// Simple inspector GUI application for viewing hard-coded graphics.
pub struct App {
    window: pgl::window::GlfwWindow,
    uniforms: shaders::AppUniforms, // Uniform buffer accessed by all shaders.
    imgui: imgui::Context,
    time: time::Time,

    state: State,
    cameras: Cameras,
    renderers: Renderers,
    shaders: Shaders,
    _textures: Textures, // Never used, but saved so that drop function is not called.
    scene: Scene,
}

impl App {
    pub fn new() -> Self {
        let model_reciever = Self::make_geometry();

        let window = pgl::window::GlfwWindow::new(1400, 800, "PGA FOR THE WIN");
        pgl::utils::gl::set_default_options();

        let mut imgui = imgui::Context::create();
        imgui_glfw::imgui::impl_glfw::init(&mut imgui, window.window);

        let uniforms = shaders::AppUniforms::new();

        let state = State {
            flying_cam: false,
            bgcolor: [0., 0., 0.],
            n_draws: 1,
            texture: 1,
            normal_map: 1,
            shader: 4,
            wireframe: false,
            model_rotation_x: 0.,
        };

        let cameras = Cameras {
            inspector: cameras::Inspector::default(),
            fly: cameras::Flying::new(),
        };

        let _textures = Textures {
            world: pgl::texture::Texture::from_path("imgs/world.jpeg", Default::default()),
            brick_normals: pgl::texture::Texture::from_path(
                "imgs/brick_normals.png",
                Default::default(),
            ),
            wall_normals: pgl::texture::Texture::from_path(
                "imgs/wall_normals.jpeg",
                Default::default(),
            ),
            wall_albedo: pgl::texture::Texture::from_path(
                "imgs/wall_albedo.jpeg",
                Default::default(),
            ),
        };
        // Bind textures to opengl texture slots.
        _textures.brick_normals.bind_to(1).unwrap();
        _textures.world.bind_to(2).unwrap();
        _textures.wall_albedo.bind_to(3).unwrap();
        _textures.wall_normals.bind_to(4).unwrap();

        let shaders = Shaders {
            bitang_nm: shaders::NormalMapping::new("shaders/nm_bitang.glsl"),
            matrix_nm: shaders::NormalMapping::new("shaders/nm_matrix.glsl"),
            rotor_nm: shaders::NormalMapping::new("shaders/nm_rotor.glsl"),
            motor_nm: shaders::NormalMapping::new("shaders/nm_motor.glsl"),
            outer_log_motor_nm: shaders::NormalMapping::new("shaders/nm_outer_log_motor.glsl"),
            outer_log_rotor_nm: shaders::NormalMapping::new("shaders/nm_outer_log_rotor.glsl"),
            cayley_log_motor_nm: shaders::NormalMapping::new("shaders/nm_cayley_motor.glsl"),
            cayley_log_rotor_nm: shaders::NormalMapping::new("shaders/nm_cayley_rotor.glsl"),
            qtang_nm: shaders::NormalMapping::new("shaders/nm_qtang.glsl"),
            textured: shaders::Textured::new(),
            flat: shaders::Flat::new(),
            nm_tex: shaders::NormalAlbedoMapping::new(),
        };

        let renderers = Renderers {
            imgui: painters::imgui::ImguiRenderer::new(&mut imgui, shaders::Ui::new()),
            gizmos: painters::gizmos::Gizmos::new(),
        };

        let time = time::Time::new(pgl::window::GlfwWindow::time());
        let scene = Scene {
            transform: glm::Mat4::identity(),
            models: Vec::new(),
            model_reciever,
            material: material::Material::default(),
            light: lights::CameraFollowingLight::new(),
        };

        Self {
            window,
            imgui,
            time,
            uniforms,
            cameras,
            state,
            _textures,
            renderers,
            shaders,
            scene,
        }
    }
    pub fn update(&mut self) {
        // Checking whether a mesh creation thread has finished a mesh.
        while let Ok(mesh) = self.scene.model_reciever.try_recv() {
            self.scene.models.push(Self::make_model(&mesh));
        }

        imgui_glfw::imgui::impl_glfw::new_frame(&mut self.imgui);

        self.time.update(pgl::window::GlfwWindow::time());

        let main_eye = unsafe { self.main_camera().as_mut().unwrap() };
        main_eye.update(&self.window, self.time.dt);

        self.uniforms.update(
            main_eye,
            &self.window,
            self.scene.light.position(&self.window, main_eye).into(),
            self.scene.light.color,
        );
        self.uniforms.set_ubo();

        self.window.poll_events();
    }

    pub fn draw(&mut self) {
        self.time.gpu_timer.begin();
        self.draw_scene();
        self.time.gpu_timer.end();

        // 3D COORDINATE GIZMO
        self.renderers.gizmos.draw();

        // UI
        let mut ui = self.imgui.frame();
        imgui_widgets::material_editor(&mut ui, &mut self.scene.material);
        imgui_widgets::main_options(&mut ui, &mut self.state, &mut self.scene);
        imgui_widgets::shading(&mut ui, &mut self.state);
        imgui_widgets::performance(&mut ui, &self.time, &self.scene, &self.window, &self.state);
        //let main_camera = unsafe { self.main_camera().as_ref().unwrap() };
        //self.scene.transform =
        //imgui_widgets::imguizmos(&mut ui, self.scene.transform, main_camera, &self.window);
        //ui.show_demo_window(mut true);
        self.renderers.imgui.draw(ui);

        // FINALIZE
        self.window.swap_buffers();
        pgl::utils::gl::clear_color(
            self.state.bgcolor[0],
            self.state.bgcolor[1],
            self.state.bgcolor[2],
        );
        pgl::utils::gl::clear();
    }

    /// Draws all model in the scene.
    fn draw_scene(&mut self) {
        use pgl::settings;
        if self.state.wireframe {
            settings::enable(&[settings::Option::Wireframe]);
        } else {
            settings::disable(&[settings::Option::Wireframe]);
        }

        let shader = unsafe { self.set_shading().as_mut().unwrap() };
        shader.set_material(&self.scene.material);
        for i in 0..self.state.n_draws {
            let translation = glm::translation::<f32>(&(i as f32 * 3.0 * -glm::Vec3::z()));
            shader.set_model(&(translation * self.scene.transform));
            for model in self.scene.models.iter() {
                if !model.active {
                    continue;
                }
                model.vao.bind();
                pgl::utils::gl::draw(model.n_indices);
            }
        }
        settings::disable(&[settings::Option::Wireframe]);
    }

    pub fn should_stop(&self) -> bool {
        self.window.should_close() || self.window.is_key_pressed(pgl::window::Key::Escape)
    }

    /// Returns the camera that is active.
    fn main_camera(&mut self) -> *mut dyn Eye {
        if self.state.flying_cam {
            &mut self.cameras.fly
        } else {
            &mut self.cameras.inspector
        }
    }

    /// Sets the shading settings based on the state (changable in GUI)
    /// and returns the active shader.
    fn set_shading(&mut self) -> *mut dyn Shader {
        let texture_slot = match self.state.texture {
            0 => 2,
            1 => 3,
            _ => unreachable!(),
        };
        let normal_map_slot = match self.state.normal_map {
            0 => 1,
            1 => 4,
            _ => unreachable!(),
        };

        match self.state.shader {
            0 => {
                self.shaders.flat.bind();
                self.shaders
                    .flat
                    .set_uniforms(&self.scene.material.albedo.into());
                &mut self.shaders.flat
            }
            1 => {
                self.shaders.textured.bind();
                self.shaders.textured.set_uniforms(texture_slot);
                &mut self.shaders.textured
            }
            2 => {
                self.shaders.matrix_nm.bind();
                self.shaders.matrix_nm.set_uniforms(normal_map_slot);
                &mut self.shaders.matrix_nm
            }
            3 => {
                self.shaders.nm_tex.bind();
                self.shaders
                    .nm_tex
                    .set_uniforms(normal_map_slot, texture_slot);
                &mut self.shaders.nm_tex
            }
            4 => {
                self.shaders.rotor_nm.bind();
                self.shaders.rotor_nm.set_uniforms(normal_map_slot);
                &mut self.shaders.rotor_nm
            }
            5 => {
                self.shaders.motor_nm.bind();
                self.shaders.motor_nm.set_uniforms(normal_map_slot);
                &mut self.shaders.motor_nm
            }
            // 6 => {
            //     self.shaders.log_motor_nm.bind();
            //     self.shaders.log_motor_nm.set_uniforms(normal_map_slot);
            //     &mut self.shaders.log_motor_nm
            // }
            6 => {
                self.shaders.outer_log_motor_nm.bind();
                self.shaders
                    .outer_log_motor_nm
                    .set_uniforms(normal_map_slot);
                &mut self.shaders.outer_log_motor_nm
            }
            7 => {
                self.shaders.outer_log_rotor_nm.bind();
                self.shaders
                    .outer_log_rotor_nm
                    .set_uniforms(normal_map_slot);
                &mut self.shaders.outer_log_rotor_nm
            }
            8 => {
                self.shaders.qtang_nm.bind();
                self.shaders.qtang_nm.set_uniforms(normal_map_slot);
                &mut self.shaders.qtang_nm
            }
            9 => {
                self.shaders.bitang_nm.bind();
                self.shaders.bitang_nm.set_uniforms(normal_map_slot);
                &mut self.shaders.bitang_nm
            }
            10 => {
                self.shaders.cayley_log_motor_nm.bind();
                self.shaders
                    .cayley_log_motor_nm
                    .set_uniforms(normal_map_slot);
                &mut self.shaders.cayley_log_motor_nm
            }
            11 => {
                self.shaders.cayley_log_rotor_nm.bind();
                self.shaders
                    .cayley_log_rotor_nm
                    .set_uniforms(normal_map_slot);
                &mut self.shaders.cayley_log_rotor_nm
            }
            _ => unreachable!(),
        }
    }

    /// Creates a model out of a mesh
    fn make_model(mesh: &Mesh) -> Model {
        let mut vao = pgl::vao::VertexArray::new_static();
        vao.bind();
        vao.buffer_indices(&mesh.faces);
        vao.new_vertex_buffer_filled("all", &mesh.vertices);

        let model = Model {
            vao,
            n_indices: mesh.faces.len() * 3,
            n_vertices: mesh.vertices.len(),
            name: mesh.name.clone(),
            group: mesh.group.clone(),
            active: false,
        };
        model
    }

    // Spawns threads that load and creates a mesh, returning the channel
    // reciever that the finshed meshes are send to.
    fn make_geometry() -> std::sync::mpsc::Receiver<Mesh> {
        let (sender, receiver) = std::sync::mpsc::channel();
        let sender_gun = sender.clone();
        let sender_geom = sender.clone();
        let sender_car = sender.clone();

        // CAR
        std::thread::spawn(move || {
            if let Ok(gun) = pgeom::obj::load("models/bugatti/bugatti.obj") {
                for mesh in gun.iter() {
                    let (vertices, faces) = mesh.render_data(|v| {
                        vertices::All::from(vertices::PosUVNormTang {
                            position: v.position,
                            uv: v.uv.unwrap_or_default(),
                            normal: v.normal.unwrap_or_default(),
                            tangent: [0., 0., 0.],
                        })
                    });
                    let mesh = Mesh {
                        vertices,
                        faces,
                        name: mesh.name.clone().unwrap_or("Dont know but from car".into()),
                        group: "Bugatti".into(),
                    };
                    sender_car.send(mesh).unwrap();
                }
            };
        });

        // SHAPES
        std::thread::spawn(move || {
            [
                (pgeom::sphere(200, 200), "Sphere"),
                (pgeom::cylinder(10, 1), "Cylinder"),
                (pgeom::rect(), "Rect"),
                (pgeom::grid(100, 200), "Grid"),
                (pgeom::monkey_saddle(200, 200), "Monkey Saddle"),
            ]
            .iter()
            .for_each(|((vertices, faces), name)| {
                let vertices = vertices
                    .iter()
                    .map(|v| vertices::PosUVNormTang {
                        position: v.position,
                        normal: v.normal,
                        uv: v.uv,
                        tangent: v.tangent,
                    })
                    .map(|v| vertices::All::from(v))
                    .collect::<Vec<_>>();
                let m = Mesh {
                    faces: faces.to_vec(),
                    vertices,
                    name: (*name).into(),
                    group: (*name).into(),
                };
                sender_geom.send(m).unwrap();
            });
        });

        // GUN
        std::thread::spawn(move || {
            if let Ok(gun) = pgeom::obj::load("models/gun/Handgun_obj.obj") {
                for mesh in gun.iter() {
                    let (vertices, faces) = mesh.render_data(|v| {
                        vertices::All::from(vertices::PosUVNormTang {
                            position: v.position,
                            uv: v.uv.unwrap_or_default(),
                            normal: v.normal.unwrap(),
                            tangent: [0., 0., 0.],
                        })
                    });
                    let m = Mesh {
                        vertices,
                        faces,
                        name: mesh.name.clone().unwrap_or("Unknown Gun Part".into()),
                        group: "Gun".into(),
                    };
                    sender_gun.send(m).unwrap();
                }
            }
        });
        receiver
    }
}

/// Contains the loose settings of the app, mostly changable
/// in the GUI.
pub struct State {
    pub flying_cam: bool,
    pub bgcolor: [f32; 3],
    pub n_draws: u32,
    pub texture: usize,
    pub normal_map: usize,
    pub shader: usize,
    pub wireframe: bool,
    pub model_rotation_x: f32,
}

struct Cameras {
    pub fly: cameras::Flying,
    pub inspector: cameras::Inspector,
}

struct Renderers {
    pub imgui: painters::imgui::ImguiRenderer,
    pub gizmos: painters::gizmos::Gizmos,
}

struct Shaders {
    pub flat: shaders::Flat,
    pub textured: shaders::Textured,
    pub matrix_nm: shaders::NormalMapping,
    pub nm_tex: shaders::NormalAlbedoMapping,
    pub rotor_nm: shaders::NormalMapping,
    pub motor_nm: shaders::NormalMapping,
    pub outer_log_motor_nm: shaders::NormalMapping,
    pub outer_log_rotor_nm: shaders::NormalMapping,
    pub cayley_log_motor_nm: shaders::NormalMapping,
    pub cayley_log_rotor_nm: shaders::NormalMapping,
    pub qtang_nm: shaders::NormalMapping,
    pub bitang_nm: shaders::NormalMapping,
}

struct Textures {
    pub world: pgl::texture::Texture,
    pub brick_normals: pgl::texture::Texture,
    pub wall_normals: pgl::texture::Texture,
    pub wall_albedo: pgl::texture::Texture,
}

pub struct Scene {
    pub model_reciever: std::sync::mpsc::Receiver<Mesh>,
    pub models: Vec<Model>,
    pub material: material::Material,
    pub light: lights::CameraFollowingLight,
    pub transform: glm::Mat4,
}

/// All info that is needed to draw a model based on state
pub struct Model {
    pub vao: pgl::vao::VertexArray,
    pub n_indices: usize,
    pub n_vertices: usize,
    pub name: String,
    pub group: String,
    pub active: bool,
}

/// The actual data desribing a model, can be discarded when
/// a vertex array has been created (data is shipped to GPU).
pub struct Mesh {
    pub faces: Vec<[u32; 3]>,
    pub vertices: Vec<vertices::All>,
    pub name: String,
    pub group: String,
}
