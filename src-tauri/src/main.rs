// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(target_os = "linux")]
    {
        // prevent "Error 71 dispatching to Wayland display." error
        std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
        // prevent "Could not create GBM EGL display: EGL_SUCCESS."
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }

    kimaier_lib::run().expect("Run converter")
}
