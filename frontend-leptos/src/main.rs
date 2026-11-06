#![allow(clippy::collapsible_if)]

fn main() {
    console_error_panic_hook::set_once();

    #[cfg(target_arch = "wasm32")]
    {
        use frontend_leptos::app::App;
        use wasm_bindgen::JsCast;

        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(root) = document
                    .get_element_by_id("root")
                    .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
                {
                    leptos::mount::mount_to(root, App).forget();
                }
                if let Some(loading) = document.get_element_by_id("app-loading") {
                    loading.remove();
                }
            }
        }
    }
}
