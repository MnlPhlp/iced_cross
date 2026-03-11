mod android;

pub fn build() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    if target_os == "android" {
        android::gen_glue_module();
    }
    if target_arch != "wasm32" {
        tauri_build::build();
    }
}
