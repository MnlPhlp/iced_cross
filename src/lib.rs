use iced::Task;
#[cfg(feature = "tauri-plugins")]
// re-exports for use in macro generated code
#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen;

#[macro_export]
#[cfg(all(not(target_arch = "wasm32"), feature = "tauri-plugins"))]
macro_rules! init_tauri_context {
    () => {
        $crate::set_tauri_context(tauri::generate_context!());
    };
}

#[macro_export]
#[cfg(any(target_arch = "wasm32", not(feature = "tauri-plugins")))]
macro_rules! init_tauri_context {
    () => {};
}

#[macro_export]
macro_rules! main {
    ($app:path) => {
        fn main() {
            iced_cross::init_tauri_context!();
            #[cfg(not(target_os = "android"))]
            {
                iced_cross::run_app::<$app>();
            }
            #[cfg(target_os = "android")]
            {
                panic!("Main should never be called on Android")
            }
        }
    };
}
#[macro_export]
#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
macro_rules! lib {
    ($app:path) => {};
}

#[macro_export]
#[cfg(target_os = "android")]
macro_rules! lib {
    ($app:path) => {
        #[unsafe(no_mangle)]
        fn android_main(app: iced::AndroidApp) {
            $crate::set_tauri_context(tauri::generate_context!());
            $crate::run_app::<$app>(app);
        }
        include!(concat!(env!("OUT_DIR"), "/android_glue.rs"));
    };
}

#[macro_export]
#[cfg(target_arch = "wasm32")]
macro_rules! lib {
    ($app:path) => {
        use $crate::wasm_bindgen;
        #[wasm_bindgen::prelude::wasm_bindgen(start)]
        fn run_web() {
            $crate::run_app::<$app>();
        }
    };
}

#[cfg(all(not(target_arch = "wasm32"), feature = "tauri-plugins"))]
static TAURI_CONTEXT: std::sync::Mutex<Option<tauri::Context>> = std::sync::Mutex::new(None);

#[cfg(all(not(target_arch = "wasm32"), feature = "tauri-plugins"))]
pub fn set_tauri_context(context: tauri::Context) {
    let mut lock = TAURI_CONTEXT.lock().expect("TAURI_CONTEXT Mutex poisoned");
    *lock = Some(context);
}

#[cfg(all(not(target_arch = "wasm32"), feature = "tauri-plugins"))]
fn get_tauri_context() -> tauri::Context {
    TAURI_CONTEXT
        .lock()
        .expect("TAURI_CONTEXT Mutex poisoned")
        .take()
        .expect(
            "Tauri context is not set. make sure to include iced_cross::main! and iced_cross::lib! macro in your main.rs and lib.rs",
        )
}

#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
pub fn init_logging() {
    tracing_subscriber::fmt::init();
}

#[cfg(target_arch = "wasm32")]
fn init_logging() {
    wasm_tracing::set_as_global_default();
}

#[cfg(target_os = "android")]
fn init_logging() {
    use tracing_android::layer;
    use tracing_subscriber::prelude::*;
    tracing_subscriber::registry()
        .with(layer(env!("CARGO_PKG_NAME")).expect("Failed to init android tracing layer"))
        .init();
}

#[allow(unused_mut)]
pub fn run_app<APP: IcedApp>(#[cfg(target_os = "android")] android_app: iced::AndroidApp) {
    init_logging();
    #[cfg(all(not(target_arch = "wasm32"), feature = "tauri-plugins"))]
    {
        let builder = tauri::Builder::default();
        let builder = APP::init_plugins(builder);
        builder
            .build(get_tauri_context())
            .expect("Failed to build Tauri application");
    }
    let mut app = iced::application(APP::new, APP::update, APP::view);
    #[cfg(any(target_os = "android", target_arch = "wasm32"))]
    {
        // on Android and web font detection does not work so we manually set a font for now
        const FIRA_SANS: &[u8] = include_bytes!("../assets/FiraSans-Medium.ttf");
        use iced::font;
        app = app.font(FIRA_SANS).default_font(font::Font {
            family: font::Family::Name("Fira Sans"),
            weight: font::Weight::Medium,
            stretch: font::Stretch::Normal,
            style: font::Style::Normal,
        })
    }
    #[cfg(target_os = "android")]
    app.run(android_app).expect("Failed to run the application");
    #[cfg(not(target_os = "android"))]
    app.run().expect("Failed to run the application");
}

pub trait IcedApp
where
    Self: Sized + 'static,
{
    type Message: std::fmt::Debug + Send;

    #[cfg(all(not(target_arch = "wasm32"), feature = "tauri-plugins"))]
    fn init_plugins(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
        builder
    }

    fn new() -> (Self, Task<Self::Message>);
    fn update(&mut self, message: Self::Message) -> Task<Self::Message>;
    fn view(&self) -> iced::Element<'_, Self::Message, iced::widget::Theme, iced::Renderer>;
}
