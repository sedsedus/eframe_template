#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(eframe_template::TemplateApp::new(cc))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        fn get_loading_text() -> Option<web_sys::Element> {
            web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id("loading_text"))
        }

        let previous_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            // Show in the HTML that start has failed
            get_loading_text().map(|e| {
                e.set_inner_html(
                    "<p> The app has crashed. See the developer console for details. </p>",
                )
            });
            // Propagate panic info to the previously registered panic hook
            previous_hook(panic_info);
        }));

        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(eframe_template::TemplateApp::new(cc))),
            )
            .await
            .expect("should start eframe");

        // loaded successfully, remove the loading indicator
        get_loading_text().map(|e| e.remove());
    });
}
