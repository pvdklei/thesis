use crate::shaders;
use crate::vertices;
use shaders::Shader;

pub struct ImguiRenderer {
    vao: pgl::vao::VertexArray,
    shader: shaders::Ui,
    font_atlas: pgl::texture::Texture,
}

impl ImguiRenderer {
    const MAX_UI_VERTICES: usize = 20000;
    const MAX_UI_INDICES: usize = 20000;

    pub fn new(ctx: &mut imgui::Context, shader: shaders::Ui) -> Self {
        let mut vao = pgl::vao::VertexArray::new_dynamic();
        vao.bind();
        vao.new_vertex_buffer_empty::<vertices::PosUVCol>("all", Self::MAX_UI_VERTICES);
        vao.ibo.init(Self::MAX_UI_INDICES * 4);
        Self {
            shader,
            vao,
            font_atlas: Self::get_font_atlas(ctx),
        }
    }

    fn get_font_atlas(ctx: &mut imgui::Context) -> pgl::texture::Texture {
        let mut fonts = ctx.fonts();
        let atlas = fonts.build_rgba32_texture();
        let atlas = pgl::texture::Texture::from_data(
            atlas.data,
            Default::default(),
            (atlas.width, atlas.height),
        );
        fonts.tex_id = (atlas.id as usize).into();
        atlas
    }

    pub fn draw(&mut self, ui: imgui::Ui) {
        let drawdata = ui.render();
        let t = glm::ortho(
            drawdata.display_pos[0],
            drawdata.display_pos[0] + drawdata.display_size[0],
            drawdata.display_pos[1] + drawdata.display_size[1],
            drawdata.display_pos[1],
            1.,
            -1.,
        );
        self.shader.bind();
        self.shader.set_uniforms(&t);
        self.font_atlas.bind_to(0).unwrap();

        //let clip_off = drawdata.display_pos;
        //let clip_scale = drawdata.framebuffer_scale;
        //let fb_height = drawdata.display_size[1] * drawdata.framebuffer_scale[1];
        //let fb_width = drawdata.display_size[0] * drawdata.framebuffer_scale[0];

        for dl in drawdata.draw_lists() {
            let i = dl
                .idx_buffer()
                .iter()
                .map(|i| *i as u32)
                .collect::<Vec<_>>();
            let v = dl
                .vtx_buffer()
                .iter()
                .map(|v| vertices::PosUVCol {
                    position: v.pos,
                    color: [
                        v.col[0] as f32 / 255.,
                        v.col[1] as f32 / 255.,
                        v.col[2] as f32 / 255.,
                        v.col[3] as f32 / 255.,
                    ],
                    uv: v.uv,
                })
                .collect::<Vec<_>>();

            assert!(v.len() < Self::MAX_UI_VERTICES);
            assert!(i.len() < Self::MAX_UI_INDICES);

            self.vao.bind();
            self.vao.subbuffer("all", &v, &i, 0);

            use pgl::settings::Option;
            pgl::settings::disable(&[Option::Depth]);
            dl.commands().for_each(|cmd| match cmd {
                imgui::DrawCmd::Elements {
                    count,
                    cmd_params:
                        imgui::DrawCmdParams {
                            idx_offset,
                            /*clip_rect: [cx, cy, cz, cw],*/
                            ..
                        },
                } => {
                    // TODO: HANDLE CLIPPING
                    //let (cx, cy, cz, cw) = (
                    //(cx - clip_off[0]) * clip_scale[0],
                    //(cy - clip_off[1]) * clip_scale[1],
                    //(cz - clip_off[0]) * clip_scale[0],
                    //(cw - clip_off[1]) * clip_scale[1],
                    //);
                    //pgl::utils::gl::scissor(
                    //cx as usize,
                    //(fb_height - cy) as usize,
                    //(cz - cx) as usize,
                    //(cw - cy) as usize,
                    //);
                    pgl::utils::gl::draw_offset(count, idx_offset * std::mem::size_of::<u32>());
                }
                _ => {
                    unimplemented!()
                }
            });
            pgl::settings::enable(&[Option::Depth]);
        }
    }
}
