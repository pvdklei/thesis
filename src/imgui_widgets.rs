use crate::material::Material;
use crate::{app, cameras, time};
use pgl::window::Key;

/// Lets you change the material params on every object in the scene
pub fn material_editor(ui: &mut imgui::Ui, material: &mut Material) {
    imgui::Window::new(imgui::im_str!("Material")).build(ui, || {
        imgui::Slider::new(imgui::im_str!("Reflectiveness"))
            .range(1..=100)
            .build(ui, &mut material.reflectiveness);
        imgui::Slider::new(imgui::im_str!("Ambient"))
            .range(0.0..=1.0)
            .build(ui, &mut material.ambient);
        imgui::Slider::new(imgui::im_str!("Specular"))
            .range(0.0..=1.0)
            .build(ui, &mut material.specular);
        imgui::ColorEdit::new(imgui::im_str!("Albedo"), &mut material.albedo).build(&ui);
    });
}

/// Shows some basic performance performance statistics
pub fn performance(
    ui: &mut imgui::Ui,
    time: &time::Time,
    scene: &app::Scene,
    window: &pgl::window::GlfwWindow,
    state: &app::State,
) {
    let mut n_indices = 0;
    let mut n_vertices = 0;
    for model in scene.models.iter() {
        if !model.active {
            continue;
        }
        n_indices += model.n_indices;
        n_vertices += model.n_vertices;
    }
    let (w, h) = window.window_size();
    let n_fragments = w * h;
    imgui::Window::new(imgui::im_str!("Performance")).build(ui, || {
        ui.text(format!("Delta Time: {:.2} ms", time.dt * 1000.));
        ui.text(format!("FPS: {:.2}", 1. / time.dt));
        ui.text(format!(
            "Draw Scene GPU Time: {:.3} ms",
            time.gpu_timer.result() as f64 / 1000000.
        ));
        ui.text(format!(
            "Number Of Indices: {}",
            n_indices * state.n_draws as usize
        ));
        ui.text(format!(
            "Number Of Vertices: {}",
            n_vertices * state.n_draws as usize
        ));
        ui.text(format!("Number Of Fragments: {}", n_fragments));
        ui.text(format!("Number Of Draw Calls: {}", state.n_draws));
    });
}

/// The imgui window with all main settings: models, camera state, light pos, etc
pub fn main_options(ui: &mut imgui::Ui, state: &mut app::State, scene: &mut app::Scene) {
    imgui::Window::new(imgui::im_str!("Main Settings")).build(ui, || {
        ui.checkbox(
            imgui::im_str!("Light Follows Mouse"),
            &mut scene.light.with_mouse,
        );
        ui.checkbox(imgui::im_str!("Move Free"), &mut state.flying_cam);
        ui.checkbox(imgui::im_str!("Wireframe"), &mut state.wireframe);
        imgui::ColorEdit::new(imgui::im_str!("Background Color"), &mut state.bgcolor).build(&ui);
        imgui::ColorEdit::new(imgui::im_str!("Light Color"), &mut scene.light.color).build(&ui);
        imgui::Slider::new(imgui::im_str!("Rotation Of Model"))
            .range(0.0..=5.0)
            .build(&ui, &mut state.model_rotation_x);
        imgui::Slider::new(imgui::im_str!("Number Of Geometries"))
            .range(1..=40)
            .build(ui, &mut state.n_draws);

        // finds all model groups there are and gives an active flag if one
        // is active and deactive flag if one is not active.
        // Group := (name, active, not active)
        let mut groups: Vec<ModelGroup> = Vec::new();
        for m in scene.models.iter() {
            if !groups.iter().any(|g| g.name == m.group) {
                groups.push(ModelGroup::new(m.group.clone()))
            }
        }
        for group in groups.iter_mut() {
            if scene
                .models
                .iter()
                .any(|m| m.group == group.name && m.active)
            {
                group.any_active = true;
            } else {
                group.any_active = false;
            }
            if scene
                .models
                .iter()
                .filter(|m| m.group == group.name)
                .all(|m| m.active)
            {
                group.any_nonactive = false;
            } else {
                group.any_nonactive = true;
            }
        }

        imgui::TreeNode::new(imgui::im_str!("Activate Model Groups"))
            .opened(true, imgui::Condition::FirstUseEver)
            .build(ui, || {
                let id = ui.push_id(1);
                groups.iter().for_each(|g| {
                    let clicked = imgui::Selectable::new(&imgui::ImString::from(g.name.clone()))
                        .selected(g.any_nonactive)
                        .build(ui);
                    if clicked {
                        for m in scene.models.iter_mut() {
                            if m.group == g.name {
                                m.active = true;
                            }
                        }
                    }
                });
                id.pop();
            });
        imgui::TreeNode::new(imgui::im_str!("Deactivate Model Groups"))
            .opened(true, imgui::Condition::FirstUseEver)
            .build(ui, || {
                let id = ui.push_id(1);
                groups.iter().for_each(|g| {
                    let clicked = imgui::Selectable::new(&imgui::ImString::from(g.name.clone()))
                        .selected(g.any_active)
                        .build(ui);
                    if clicked {
                        for m in scene.models.iter_mut() {
                            if m.group == g.name {
                                m.active = false;
                            }
                        }
                    }
                });
                id.pop();
            });
        imgui::TreeNode::new(imgui::im_str!("Models")).build(ui, || {
            scene.models.iter_mut().for_each(|m| {
                let clicked = imgui::Selectable::new(&imgui::ImString::from(m.name.clone()))
                    .selected(m.active)
                    .build(ui);
                if clicked {
                    m.active = !m.active;
                }
            });
        });
    });
}

/// Lets you change textures and shaders used on the models in the scene
pub fn shading(ui: &mut imgui::Ui, state: &mut app::State) {
    imgui::Window::new(imgui::im_str!("Shading")).build(ui, || {
        imgui::ListBox::new(imgui::im_str!("Shader")).build_simple(
            ui,
            &mut state.shader,
            &[
                "Flat Phong Shading",
                "Albedo Mapping",
                "Normal Mapping With Matrix",
                "Normal And Albedo Mapping",
                "Normal Mapping With Rotor",
                "Normal Mapping With Motor",
                "Normal Mapping With Outer Log Motor",
                "Normal Mapping With Outer Log Rotor",
                "Normal Mapping With QTangent",
                "Normal Mapping With Tang and BiTang",
                "With Cayley Motor",
                "With Cayley Rotor",
            ],
            &get_name,
        );
        imgui::ListBox::new(imgui::im_str!("Texture")).build_simple(
            ui,
            &mut state.texture,
            &["World", "Wall"],
            &get_name,
        );
        imgui::ListBox::new(imgui::im_str!("Normal Map")).build_simple(
            ui,
            &mut state.normal_map,
            &["Bricks", "Wall"],
            &get_name,
        );
    });
}

/// Used by imgui to get labels from the items passed in a list. In
/// this case, the items and labels are the same.
fn get_name<'a>(a: &'a &str) -> std::borrow::Cow<'a, imgui::ImStr> {
    imgui::ImString::new(*a).into()
}

/// Adds the options to change the model transform with transformation gizmos
/// and adds a grid to the scene.  
//pub fn imguizmos(
    //ui: &mut imgui::Ui,
    //model_matrix: glm::Mat4,
    //cam: &dyn cameras::Eye,
    //win: &pgl::window::GlfwWindow,
//) -> glm::Mat4 {
    //let gizmo = imguizmo::Gizmo::begin_frame(ui);
    //let rect = imguizmo::Rect::from_display(ui);
    //gizmo.set_rect(rect.x, rect.y, rect.width, rect.height);
    //gizmo.set_orthographic(false);
    //let view = cam.view();
    //let projection = cam.projection();
    //let mode = imguizmo::Mode::World;
    //let op = if win.is_key_pressed(Key::N) {
        //imguizmo::Operation::Scale
    //} else if win.is_key_pressed(Key::M) {
        //imguizmo::Operation::Rotate
    //} else {
        //imguizmo::Operation::Translate
    //};
    //let mut model_matrix: [[f32; 4]; 4] = model_matrix.into();
    ////println!("{:#?}", "Bef");
    //gizmo.manipulate(
        //&view.into(),
        //&projection.into(),
        //op,
        //mode,
        //&mut model_matrix,
        //None,
        //None,
        //None,
        //None,
    //);
    ////let mut view = view.clone();
    ////let (w, h) = win.window_size();
    ////gizmo.view_manipulate(&mut view.into(), 8.0, [0.0, 0.0], [w as f32, h as f32], 0);

    //gizmo.draw_grid(
        //&view.into(),
        //&projection.into(),
        //&glm::Mat4::identity().into(),
        //100.0,
    //);
    ////println!("{:#?}", "Af");
    //model_matrix.into()
//}

/// Some models belong together, but must be drawn seperately.
/// For instance different parts of a car. All these models have
/// the same group.
///
/// Used in main settings for changing the active models.
struct ModelGroup {
    name: String,
    any_active: bool,
    any_nonactive: bool,
}

impl ModelGroup {
    pub fn new(name: String) -> Self {
        Self {
            name,
            any_active: false,
            any_nonactive: true,
        }
    }
}
