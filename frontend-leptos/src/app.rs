use leptos::prelude::*;
use std::sync::LazyLock;

use crate::config::{PortfolioConfig, get_config, get_unique_tech_tags};
use crate::sections::{
    About, CaseStudyModal, Experience, Footer, Header, Infrastructure, Navigation, Projects, Skills,
};
use crate::spotify::use_spotify;
use crate::telemetry::use_telemetry;
use crate::theme::provide_theme_context;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::closure::Closure;

pub static CONFIG: LazyLock<PortfolioConfig> = LazyLock::new(get_config);

// The signal setters are only used in the browser (wasm32) code below.
#[cfg_attr(not(target_arch = "wasm32"), allow(unused_variables))]
#[component]
pub fn App() -> impl IntoView {
    provide_theme_context();

    let (selected_tech, set_selected_tech) = signal(None::<String>);
    let (show_devops_case_study, set_show_devops_case_study) = signal(false);
    let (is_visible, set_is_visible) = signal(true);
    let (active_section, set_active_section) = signal("about");
    let (is_scrolled, set_is_scrolled) = signal(false);

    let all_tech_tags = get_unique_tech_tags(&CONFIG.projects);

    #[cfg(target_arch = "wasm32")]
    {
        // 1. Visibility change listener
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                let vis_closure = Closure::<dyn FnMut()>::new(move || {
                    if let Some(w) = web_sys::window() {
                        if let Some(doc) = w.document() {
                            let visible =
                                doc.visibility_state() == web_sys::VisibilityState::Visible;
                            set_is_visible.set(visible);
                        }
                    }
                });
                let _ = document.add_event_listener_with_callback(
                    "visibilitychange",
                    vis_closure.as_ref().unchecked_ref(),
                );
                vis_closure.forget();
            }
        }

        // 2. Scroll listener
        if let Some(window) = web_sys::window() {
            let scroll_closure = Closure::<dyn FnMut()>::new(move || {
                if let Some(w) = web_sys::window() {
                    let scroll_y = w.scroll_y().unwrap_or(0.0);
                    set_is_scrolled.set(scroll_y > 50.0);

                    let sections = [
                        "about",
                        "skills",
                        "projects",
                        "experience",
                        "infrastructure",
                    ];
                    let scroll_pos = scroll_y + 200.0;

                    if let Some(doc) = w.document() {
                        for sec in sections {
                            if let Some(el) = doc.get_element_by_id(sec) {
                                if let Ok(html_el) = el.dyn_into::<web_sys::HtmlElement>() {
                                    let top = html_el.offset_top() as f64;
                                    let height = html_el.offset_height() as f64;
                                    if scroll_pos >= top && scroll_pos < top + height {
                                        set_active_section.set(sec);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            });

            let _ = window.add_event_listener_with_callback(
                "scroll",
                scroll_closure.as_ref().unchecked_ref(),
            );
            scroll_closure.forget();
        }

        // 3. IntersectionObserver for scroll-reveal animations
        Effect::new(move |_| {
            if let Some(window) = web_sys::window() {
                if let Some(doc) = window.document() {
                    let observer_closure =
                        Closure::<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>::new(
                            |entries: js_sys::Array, _obs: web_sys::IntersectionObserver| {
                                for i in 0..entries.length() {
                                    if let Ok(entry) = entries
                                        .get(i)
                                        .dyn_into::<web_sys::IntersectionObserverEntry>(
                                    ) {
                                        if entry.is_intersecting() {
                                            let target = entry.target();
                                            let _ = target.class_list().add_1("active");
                                        }
                                    }
                                }
                            },
                        );

                    let init = web_sys::IntersectionObserverInit::new();
                    init.set_threshold(&wasm_bindgen::JsValue::from_f64(0.05));
                    init.set_root_margin("0px 0px -50px 0px");

                    if let Ok(observer) = web_sys::IntersectionObserver::new_with_options(
                        observer_closure.as_ref().unchecked_ref(),
                        &init,
                    ) {
                        if let Ok(nodes) = doc.query_selector_all(".reveal-in") {
                            for i in 0..nodes.length() {
                                if let Some(node) = nodes.item(i) {
                                    if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                                        observer.observe(&el);
                                    }
                                }
                            }
                        }
                    }
                    observer_closure.forget();
                }
            }
        });
    }

    let telemetry = use_telemetry(is_visible.into());
    let spotify = use_spotify(is_visible.into());

    view! {
        <div
            class="min-h-screen bg-[var(--color-bg-primary)] text-[var(--color-text-primary)] flex flex-col items-center pt-12 pb-8 px-4 sm:px-6 lg:px-8"
            style="background-image: var(--gradient-hero)"
        >
            <Navigation
                is_scrolled=is_scrolled.into()
                active_section=active_section.into()
                config=&CONFIG
            />

            <Header config=&CONFIG />

            <main class="w-full max-w-5xl space-y-16 md:space-y-20">
                <About config=&CONFIG />
                <Skills config=&CONFIG />
                <Projects
                    config=&CONFIG
                    selected_tech=selected_tech.into()
                    set_selected_tech=set_selected_tech
                    set_show_case_study=set_show_devops_case_study
                    all_tech_tags=all_tech_tags
                />
                <Experience config=&CONFIG />
                <Infrastructure telemetry=telemetry spotify=spotify />
            </main>

            <Footer config=&CONFIG />

            <CaseStudyModal
                show=show_devops_case_study.into()
                set_show=set_show_devops_case_study
                config=&CONFIG
            />
        </div>
    }
}
