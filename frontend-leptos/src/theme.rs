use leptos::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::closure::Closure;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    System,
    Light,
    Dark,
}

pub fn cycle_theme(current: Theme) -> Theme {
    match current {
        Theme::System => Theme::Light,
        Theme::Light => Theme::Dark,
        Theme::Dark => Theme::System,
    }
}

pub fn parse_theme(s: &str) -> Theme {
    match s.trim().to_lowercase().as_str() {
        "light" => Theme::Light,
        "dark" => Theme::Dark,
        _ => Theme::System,
    }
}

pub fn theme_as_str(theme: Theme) -> &'static str {
    match theme {
        Theme::System => "system",
        Theme::Light => "light",
        Theme::Dark => "dark",
    }
}

pub fn theme_aria_label(theme: Theme) -> String {
    let next = match theme {
        Theme::System => "light",
        Theme::Light => "dark",
        Theme::Dark => "system",
    };
    format!(
        "Theme: {}. Click to switch to {} mode.",
        theme_as_str(theme),
        next
    )
}

pub fn theme_title(theme: Theme, resolved_theme: &str) -> String {
    format!(
        "Active Theme: {} (Resolved: {})",
        theme_as_str(theme).to_uppercase(),
        resolved_theme.to_uppercase()
    )
}

#[derive(Copy, Clone, Debug)]
pub struct ThemeContext {
    pub theme: RwSignal<Theme>,
    pub resolved_theme: RwSignal<&'static str>,
}

impl ThemeContext {
    pub fn cycle(&self) {
        let next = cycle_theme(self.theme.get_untracked());
        self.set_theme(next);
    }

    pub fn set_theme(&self, next: Theme) {
        self.theme.set(next);
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item("theme", theme_as_str(next)).map_err(|e| {
                        web_sys::console::warn_2(
                            &"Failed to write theme to localStorage:".into(),
                            &e,
                        );
                    });
                }
            }
        }
    }
}

pub fn provide_theme_context() -> ThemeContext {
    let initial_theme = {
        #[cfg(target_arch = "wasm32")]
        {
            web_sys::window()
                .and_then(|w| w.local_storage().ok().flatten())
                .and_then(|s| s.get_item("theme").ok().flatten())
                .map(|val| parse_theme(&val))
                .unwrap_or(Theme::System)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Theme::System
        }
    };

    let theme = RwSignal::new(initial_theme);
    let resolved_theme = RwSignal::new("dark");
    let ctx = ThemeContext {
        theme,
        resolved_theme,
    };
    provide_context(ctx);

    #[cfg(target_arch = "wasm32")]
    {
        // Reactive effect to update DOM class and resolved_theme
        Effect::new(move |_| {
            let current = theme.get();
            if let Some(window) = web_sys::window() {
                let prefers_dark = window
                    .match_media("(prefers-color-scheme: dark)")
                    .ok()
                    .flatten()
                    .map(|m| m.matches())
                    .unwrap_or(false);

                let is_dark = match current {
                    Theme::Dark => true,
                    Theme::System => prefers_dark,
                    Theme::Light => false,
                };

                if let Some(document) = window.document() {
                    if let Some(root) = document.document_element() {
                        let class_list = root.class_list();
                        if is_dark {
                            let _ = class_list.add_1("dark");
                            resolved_theme.set("dark");
                        } else {
                            let _ = class_list.remove_1("dark");
                            resolved_theme.set("light");
                        }
                    }
                }
            }
        });

        // Listen for system changes when theme == System
        if let Some(window) = web_sys::window() {
            if let Ok(Some(media_query)) = window.match_media("(prefers-color-scheme: dark)") {
                let listener = Closure::<dyn FnMut(web_sys::MediaQueryListEvent)>::new(
                    move |_e: web_sys::MediaQueryListEvent| {
                        if theme.get_untracked() == Theme::System {
                            if let Some(window) = web_sys::window() {
                                let prefers_dark = window
                                    .match_media("(prefers-color-scheme: dark)")
                                    .ok()
                                    .flatten()
                                    .map(|m| m.matches())
                                    .unwrap_or(false);

                                if let Some(document) = window.document() {
                                    if let Some(root) = document.document_element() {
                                        let class_list = root.class_list();
                                        if prefers_dark {
                                            let _ = class_list.add_1("dark");
                                            resolved_theme.set("dark");
                                        } else {
                                            let _ = class_list.remove_1("dark");
                                            resolved_theme.set("light");
                                        }
                                    }
                                }
                            }
                        }
                    },
                );

                let _ = media_query
                    .add_event_listener_with_callback("change", listener.as_ref().unchecked_ref());
                listener.forget();
            }
        }
    }

    ctx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_cycle_order() {
        assert_eq!(cycle_theme(Theme::System), Theme::Light);
        assert_eq!(cycle_theme(Theme::Light), Theme::Dark);
        assert_eq!(cycle_theme(Theme::Dark), Theme::System);
    }

    #[test]
    fn test_theme_parse() {
        assert_eq!(parse_theme("light"), Theme::Light);
        assert_eq!(parse_theme("LIGHT"), Theme::Light);
        assert_eq!(parse_theme("dark"), Theme::Dark);
        assert_eq!(parse_theme("DARK"), Theme::Dark);
        assert_eq!(parse_theme("system"), Theme::System);
        // Unknown stored value falls back to system
        assert_eq!(parse_theme("unknown"), Theme::System);
        assert_eq!(parse_theme(""), Theme::System);
        assert_eq!(parse_theme("auto"), Theme::System);
    }

    #[test]
    fn test_theme_labels() {
        assert_eq!(
            theme_aria_label(Theme::System),
            "Theme: system. Click to switch to light mode."
        );
        assert_eq!(
            theme_aria_label(Theme::Light),
            "Theme: light. Click to switch to dark mode."
        );
        assert_eq!(
            theme_aria_label(Theme::Dark),
            "Theme: dark. Click to switch to system mode."
        );
        assert_eq!(
            theme_title(Theme::System, "dark"),
            "Active Theme: SYSTEM (Resolved: DARK)"
        );
    }
}
