use pthesis::*;

fn main() {
    let mut app = app::App::new();
    while !app.should_stop() {
        pgl::utils::gl::check_error();
        pgl::utils::gl::flush_error();
        app.update();
        app.draw();
    }
}
