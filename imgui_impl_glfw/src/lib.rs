pub mod imgui {
    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub mod impl_glfw {
        use glfw::ffi::*;

        // GLOBALS

        static mut GDATA: GData = GData {
            imgui_io: std::ptr::null_mut(),
            window: std::ptr::null_mut(),
            time: 0.,
        };
        struct GData {
            pub imgui_io: *mut imgui::Io,
            pub window: *mut GLFWwindow,
            pub time: f32,
        }
        unsafe impl Sync for GData {}
        fn gdata() -> &'static mut GData {
            unsafe { &mut GDATA }
        }

        // PUBLIC API

        pub fn init(imgui: &mut imgui::Context, window: *mut GLFWwindow) {
            imgui.set_platform_name(Some(imgui::ImString::new("pimgui_impl_glfw")));
            let io = imgui.io_mut();

            let mut gdata = gdata();
            gdata.imgui_io = io as _;
            gdata.window = window;

            io.backend_flags
                .insert(imgui::BackendFlags::HAS_MOUSE_CURSORS);
            io.backend_flags
                .insert(imgui::BackendFlags::HAS_SET_MOUSE_POS);

            io[imgui::Key::Tab] = KEY_TAB as _;
            io[imgui::Key::LeftArrow] = KEY_LEFT as _;
            io[imgui::Key::RightArrow] = KEY_RIGHT as _;
            io[imgui::Key::UpArrow] = KEY_UP as _;
            io[imgui::Key::DownArrow] = KEY_DOWN as _;
            io[imgui::Key::PageUp] = KEY_PAGE_UP as _;
            io[imgui::Key::PageDown] = KEY_PAGE_DOWN as _;
            io[imgui::Key::Home] = KEY_HOME as _;
            io[imgui::Key::End] = KEY_END as _;
            io[imgui::Key::Insert] = KEY_INSERT as _;
            io[imgui::Key::Delete] = KEY_DELETE as _;
            io[imgui::Key::Backspace] = KEY_BACKSPACE as _;
            io[imgui::Key::Space] = KEY_SPACE as _;
            io[imgui::Key::Enter] = KEY_ENTER as _;
            io[imgui::Key::Escape] = KEY_ESCAPE as _;
            //io[imgui::Key::KeyPadEnter] = VirtualKeyCode::NumpadEnter as _;
            io[imgui::Key::A] = KEY_A as _;
            io[imgui::Key::C] = KEY_C as _;
            io[imgui::Key::V] = KEY_V as _;
            io[imgui::Key::X] = KEY_X as _;
            io[imgui::Key::Y] = KEY_Y as _;
            io[imgui::Key::Z] = KEY_Z as _;

            unsafe {
                glfwSetCursorPosCallback(window, Some(cursor_pos_callback));
                glfwSetKeyCallback(window, Some(key_callback));
                glfwSetCharCallback(window, Some(char_callback));
                glfwSetMouseButtonCallback(window, Some(mouse_button_callback));
                glfwSetScrollCallback(window, Some(scroll_callback));
            }
        }
        pub fn new_frame(imgui: &mut imgui::Context) {
            let GData {
                imgui_io: io,
                window,
                time,
            } = gdata();

            assert!(imgui.fonts().is_built());

            let mut w = 0;
            let mut h = 0;
            let mut fbw = 0;
            let mut fbh = 0;
            unsafe {
                glfwGetWindowSize(*window, &mut w, &mut h);
                glfwGetFramebufferSize(*window, &mut fbw, &mut fbh);
            }
            let mut io = imgui.io_mut();
            io.display_size = [w as _, h as _];
            io.display_framebuffer_scale = [fbw as f32 / w as f32, fbh as f32 / h as f32];
            let new_time = unsafe { glfwGetTime() as f32 };
            io.delta_time = if *time > 0. {
                new_time - *time
            } else {
                1. / 60.
            };
            *time = new_time;
        }

        // ALL GLFW EVENT CALLBACKS

        extern "C" fn mouse_button_callback(
            window: *mut GLFWwindow,
            button: i32,
            action: i32,
            mods: i32,
        ) {
            let GData { imgui_io, .. } = gdata();
            let io = unsafe { imgui_io.as_mut().unwrap() };
            match action {
                PRESS => {
                    io.mouse_down[button as usize] = true;
                }
                RELEASE => {
                    io.mouse_down[button as usize] = false;
                }
                _ => println!("{:#?}", "[IMGUI GLFW WARNING] Unknown mouse button action"),
            }
        }
        extern "C" fn scroll_callback(window: *mut GLFWwindow, xoffset: f64, yoffset: f64) {
            let GData { imgui_io, .. } = gdata();
            let io = unsafe { imgui_io.as_mut().unwrap() };
            io.mouse_wheel_h += xoffset as f32;
            io.mouse_wheel += yoffset as f32;
        }
        extern "C" fn key_callback(
            window: *mut GLFWwindow,
            key: i32,
            scancode: i32,
            action: i32,
            mods: i32,
        ) {
            let GData { imgui_io, .. } = gdata();
            let io = unsafe { imgui_io.as_mut().unwrap() };
            if key as usize >= io.keys_down.len() {
                println!("{:#?}", "[IMGUI GLFW WARNING] Unknown key");
                return;
            }
            match action {
                PRESS => io.keys_down[key as usize] = true,
                RELEASE => io.keys_down[key as usize] = false,
                REPEAT => {}
                _ => println!("{:#?}", "[IMGUI GLFW WARNING] Unknown key action"),
            }
            io.key_ctrl =
                io.keys_down[KEY_LEFT_CONTROL as usize] || io.keys_down[KEY_RIGHT_CONTROL as usize];
            io.key_shift =
                io.keys_down[KEY_LEFT_SHIFT as usize] || io.keys_down[KEY_RIGHT_SHIFT as usize];
            io.key_alt =
                io.keys_down[KEY_LEFT_ALT as usize] || io.keys_down[KEY_RIGHT_ALT as usize];
        }
        extern "C" fn char_callback(window: *mut GLFWwindow, c: u32) {
            let GData { imgui_io, .. } = gdata();
            let io = unsafe { imgui_io.as_mut().unwrap() };
            let c = std::char::from_u32(c).unwrap();
            if c != '\u{7f}' {
                io.add_input_character(c)
            }
        }
        extern "C" fn cursor_pos_callback(window: *mut GLFWwindow, xpos: f64, ypos: f64) {
            let GData { imgui_io, .. } = gdata();
            let io = unsafe { imgui_io.as_mut().unwrap() };
            io.mouse_pos = [xpos as f32, ypos as f32];
        }
    }
}
