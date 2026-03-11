use std::env;

pub fn gen_glue_module() {
    let pkg_name = env!("CARGO_PKG_NAME");
    let code = format!(
        r#"
mod android_glue {{
    fn test(){{
        println!("Hello from the generated glue module of {pkg_name}!");
    }}
}}
"#
    );
    let path = format!(
        "{}/android_glue.rs",
        std::env::var("OUT_DIR").expect("missing OUT_DIR env Variable")
    );
    std::fs::write(path, code).expect("Failed to write android glue module");
}
// mod android_glue {
//     use jni::JNIEnv;
//     use jni::objects::{JClass, JString};
//
//     #[unsafe(no_mangle)]
//     #[allow(non_snake_case)]
//     fn Java_de_philipp_1manuel_slint_1android_1test_WryActivity_create() {}
//
//     #[unsafe(no_mangle)]
//     #[allow(non_snake_case)]
//     fn Java_de_philipp_1manuel_slint_1android_1test_WryActivity_start() {}
//
//     // this function is a glue between PluginManager.kt > handlePluginResponse and Rust
//     #[allow(non_snake_case)]
//     #[unsafe(no_mangle)]
//     pub fn Java_app_tauri_plugin_PluginManager_handlePluginResponse(
//         mut env: JNIEnv,
//         _: JClass,
//         id: i32,
//         success: JString,
//         error: JString,
//     ) {
//         ::tauri::handle_android_plugin_response(&mut env, id, success, error);
//     }
//
//     // this function is a glue between PluginManager.kt > sendChannelData and Rust
//     #[allow(non_snake_case)]
//     #[unsafe(no_mangle)]
//     pub fn Java_app_tauri_plugin_PluginManager_sendChannelData(
//         mut env: JNIEnv,
//         _: JClass,
//         id: i64,
//         data: JString,
//     ) {
//         ::tauri::send_channel_data(&mut env, id, data);
//     }
// }
