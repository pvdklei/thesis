// We use `chlorine` over (the more well known) `cty` right now since `cty`
// doesn't fully match std::os::raw (leading to issues like
// https://github.com/japaric/cty/issues/18). Chlorine *does* match std::os::raw
// (and libc), but has a longer and more confusing name, so we just alias it.
// Also, this makes it easier to switch to something else/back easier, if we
// decide to.
//
// Note that with the exception of bugs like the above, which crate we use (cty,
// chlorine, libc, std::os::raw, ...) shouldn't matter to end user code¹, since
// these are type aliases that should all be equivalent. This means that we're
// free to switch back iff the bug is fixed, and users are free to use whichever
// they prefer regardless of what we chose.
//
// (TODO: using extern crate for this is a hack, we probably should replace this
// with `use chlorine as cty` in the binding files eventually, but lets punt on
// it for a bit)
//
// ¹ The exception to this is that `std::os::raw` isn't there for `no_std`, and
// `libc` has potentially undesirable linking impacts on windows.
extern crate chlorine as cty;
pub use crate::ImDrawData;

extern "C" {
    pub fn ImGui_ImplGlfw_InitForOpenGL(
        window: *mut glfw::ffi::GLFWwindow,
        install_callbacks: bool,
    );
    pub fn ImGui_ImplGlfw_Shutdown();
    pub fn ImGui_ImplGlfw_NewFrame();

    pub fn ImGui_ImplOpenGL3_Init(glsl_version: *const std::os::raw::c_char);
    pub fn ImGui_ImplOpenGL3_Shutdown();
    pub fn ImGui_ImplOpenGL3_NewFrame();
    pub fn ImGui_ImplOpenGL3_RenderDrawData(draw_data: *mut ImDrawData);

}

pub mod backend {
    pub fn glfw_init_for_opengl(window: &mut glfw::ffi::GLFWwindow, install_callbacks: bool) {
        unsafe {
            super::ImGui_ImplGlfw_InitForOpenGL(window as *mut _, install_callbacks);
        }
    }
    pub fn glfw_shutdown() {
        unsafe {
            super::ImGui_ImplGlfw_Shutdown();
        }
    }
    pub fn glfw_new_frame() {
        unsafe {
            super::ImGui_ImplGlfw_NewFrame();
        }
    }
    pub fn opengl_init(glslv: &str) {
        unsafe {
            let v = std::ffi::CString::new(glslv).unwrap();
            super::ImGui_ImplOpenGL3_Init(v.as_ptr());
        }
    }
    pub fn opengl_new_frame() {
        unsafe {
            super::ImGui_ImplOpenGL3_NewFrame();
        }
    }
    pub fn opengl_shutdown() {
        unsafe {
            super::ImGui_ImplOpenGL3_Shutdown();
        }
    }
    pub fn opengl_render_drawdata(data: *mut super::ImDrawData) {
        unsafe {
            super::ImGui_ImplOpenGL3_RenderDrawData(data);
        }
    }
}

#[cfg(feature = "wasm")]
mod wasm_bindings;

#[cfg(feature = "wasm")]
pub use crate::wasm_bindings::*;

#[cfg(not(feature = "wasm"))]
mod bindings;

#[cfg(not(feature = "wasm"))]
pub use crate::bindings::*;

impl ImVec2 {
    #[inline]
    pub const fn new(x: f32, y: f32) -> ImVec2 {
        ImVec2 { x, y }
    }
    #[inline]
    pub const fn zero() -> ImVec2 {
        ImVec2 { x: 0.0, y: 0.0 }
    }
}

impl From<[f32; 2]> for ImVec2 {
    #[inline]
    fn from(array: [f32; 2]) -> ImVec2 {
        ImVec2::new(array[0], array[1])
    }
}

impl From<(f32, f32)> for ImVec2 {
    #[inline]
    fn from((x, y): (f32, f32)) -> ImVec2 {
        ImVec2::new(x, y)
    }
}

impl From<ImVec2> for [f32; 2] {
    #[inline]
    fn from(v: ImVec2) -> [f32; 2] {
        [v.x, v.y]
    }
}

impl From<ImVec2> for (f32, f32) {
    #[inline]
    fn from(v: ImVec2) -> (f32, f32) {
        (v.x, v.y)
    }
}

impl ImVec4 {
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> ImVec4 {
        ImVec4 { x, y, z, w }
    }
    #[inline]
    pub const fn zero() -> ImVec4 {
        ImVec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
}

impl From<[f32; 4]> for ImVec4 {
    #[inline]
    fn from(array: [f32; 4]) -> ImVec4 {
        ImVec4::new(array[0], array[1], array[2], array[3])
    }
}

impl From<(f32, f32, f32, f32)> for ImVec4 {
    #[inline]
    fn from((x, y, z, w): (f32, f32, f32, f32)) -> ImVec4 {
        ImVec4::new(x, y, z, w)
    }
}

impl From<ImVec4> for [f32; 4] {
    #[inline]
    fn from(v: ImVec4) -> [f32; 4] {
        [v.x, v.y, v.z, v.w]
    }
}

impl From<ImVec4> for (f32, f32, f32, f32) {
    #[inline]
    fn from(v: ImVec4) -> (f32, f32, f32, f32) {
        (v.x, v.y, v.z, v.w)
    }
}

#[test]
fn test_imvec2_memory_layout() {
    use core::mem;
    assert_eq!(mem::size_of::<ImVec2>(), mem::size_of::<[f32; 2]>());
    assert_eq!(mem::align_of::<ImVec2>(), mem::align_of::<[f32; 2]>());
    let test = ImVec2::new(1.0, 2.0);
    let ref_a: &ImVec2 = &test;
    let ref_b: &[f32; 2] = unsafe { &*(&test as *const _ as *const [f32; 2]) };
    assert_eq!(&ref_a.x as *const _, &ref_b[0] as *const _);
    assert_eq!(&ref_a.y as *const _, &ref_b[1] as *const _);
}

#[test]
fn test_imvec4_memory_layout() {
    use core::mem;
    assert_eq!(mem::size_of::<ImVec4>(), mem::size_of::<[f32; 4]>());
    assert_eq!(mem::align_of::<ImVec4>(), mem::align_of::<[f32; 4]>());
    let test = ImVec4::new(1.0, 2.0, 3.0, 4.0);
    let ref_a: &ImVec4 = &test;
    let ref_b: &[f32; 4] = unsafe { &*(&test as *const _ as *const [f32; 4]) };
    assert_eq!(&ref_a.x as *const _, &ref_b[0] as *const _);
    assert_eq!(&ref_a.y as *const _, &ref_b[1] as *const _);
    assert_eq!(&ref_a.z as *const _, &ref_b[2] as *const _);
    assert_eq!(&ref_a.w as *const _, &ref_b[3] as *const _);
}
