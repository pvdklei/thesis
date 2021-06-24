use imgui::*;

mod support;

// rect is [x, y, w, h]
fn draw_text_centered(
    ui: &Ui,
    draw_list: &DrawListMut,
    rect: [f32; 4],
    text: &ImStr,
    color: [f32; 3],
) {
    let text_size = ui.calc_text_size(text);
    let cx = (rect[2] - text_size[0]) / 2.0;
    let cy = (rect[3] - text_size[1]) / 2.0;
    draw_list.add_text([rect[0] + cx, rect[1] + cy], color, text);
}

fn main() {
    let system = support::init(file!());
    system.main_loop(move |_, ui| {
        {
            let bg_draw_list = ui.get_background_draw_list();
            bg_draw_list
                .add_circle([150.0, 150.0], 150.0, [1.0, 0.0, 0.0])
                .thickness(4.0)
                .build();
            draw_text_centered(
                ui,
                &bg_draw_list,
                [0.0, 0.0, 300.0, 300.0],
                im_str!("background draw list"),
                [0.0, 0.0, 0.0],
            );
        }

        {
            let [w, h] = ui.io().display_size;
            let fg_draw_list = ui.get_foreground_draw_list();
            fg_draw_list
                .add_circle([w - 150.0, h - 150.0], 150.0, [1.0, 0.0, 0.0])
                .thickness(4.0)
                .build();
            draw_text_centered(
                ui,
                &fg_draw_list,
                [w - 300.0, h - 300.0, 300.0, 300.0],
                im_str!("foreground draw list"),
                [1.0, 0.0, 0.0],
            );
        }

        Window::new(im_str!("Draw list"))
            .size([300.0, 110.0], Condition::FirstUseEver)
            .scroll_bar(false)
            .build(ui, || {
                ui.button(im_str!("random button"));
                let draw_list = ui.get_window_draw_list();
                let o = ui.cursor_screen_pos();
                let ws = ui.content_region_avail();
                draw_list
                    .add_circle([o[0] + 10.0, o[1] + 10.0], 5.0, [1.0, 0.0, 0.0])
                    .thickness(4.0)
                    .build();
                draw_list
                    .add_circle([o[0] + ws[0] - 10.0, o[1] + 10.0], 5.0, [0.0, 1.0, 0.0])
                    .thickness(4.0)
                    .build();
                draw_list
                    .add_circle(
                        [o[0] + ws[0] - 10.0, o[1] + ws[1] - 10.0],
                        5.0,
                        [0.0, 0.0, 1.0],
                    )
                    .thickness(4.0)
                    .build();
                draw_list
                    .add_circle([o[0] + 10.0, o[1] + ws[1] - 10.0], 5.0, [1.0, 1.0, 0.0])
                    .thickness(4.0)
                    .build();
                draw_text_centered(
                    ui,
                    &draw_list,
                    [o[0], o[1], ws[0], ws[1]],
                    im_str!("window draw list"),
                    [1.0, 1.0, 1.0],
                );
            });
    });
}
