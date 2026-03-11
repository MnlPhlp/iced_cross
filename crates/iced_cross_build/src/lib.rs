mod android;

pub fn build() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "android" {
        android::gen_glue_module();
    }
}
