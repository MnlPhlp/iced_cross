pub fn gen_glue_module() {
    let identifier = "com.example.counter"; // TODO: get this from the tauri.conf.json
    let method_pkg = identifier.replace('-', "_1").replace('.', "_");
    let code = format!(
        "
mod android_glue {{
    use tauri::wry::prelude::*;

    // this function is a glue between PluginManager.kt > handlePluginResponse and Rust
    #[allow(non_snake_case)]
    #[unsafe(no_mangle)]
    pub fn Java_app_tauri_plugin_PluginManager_handlePluginResponse(
        mut env: JNIEnv,
        _: JClass,
        id: i32,
        success: JString,
        error: JString,
    ) {{
        ::tauri::handle_android_plugin_response(&mut env, id, success, error);
    }}

    // this function is a glue between PluginManager.kt > sendChannelData and Rust
    #[allow(non_snake_case)]
    #[unsafe(no_mangle)]
    pub fn Java_app_tauri_plugin_PluginManager_sendChannelData(
        mut env: JNIEnv,
        _: JClass,
        id: i64,
        data: JString,
    ) {{
        ::tauri::send_channel_data(&mut env, id, data);
    }}
    #[unsafe(no_mangle)]
    #[allow(non_snake_case)]
    fn Java_{method_pkg}_WryActivity_create() {{}}

    #[unsafe(no_mangle)]
    #[allow(non_snake_case)]
    fn Java_{method_pkg}_WryActivity_start() {{}}
}}
"
    );
    let path = format!(
        "{}/android_glue.rs",
        std::env::var("OUT_DIR").expect("missing OUT_DIR env Variable")
    );
    std::fs::write(path, code).expect("Failed to write android glue module");
}
