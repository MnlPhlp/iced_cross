// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(not(target_os = "android"))]
    {
        iced_cross::run_app::<counter_lib::Counter>();
    }
    #[cfg(target_os = "android")]
    {
        panic!("Main should never be called on Android")
    }
}
