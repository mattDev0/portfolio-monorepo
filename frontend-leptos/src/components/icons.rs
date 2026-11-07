use leptos::prelude::*;

/// Inner SVG markup (24x24 viewBox, stroked with `currentColor`) for a named icon.
fn markup(name: &str) -> Option<&'static str> {
    Some(match name {
        "globe" => {
            r#"<circle cx="12" cy="12" r="10"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/><path d="M2 12h20"/>"#
        }
        "network" => {
            r#"<rect x="9" y="2" width="6" height="6" rx="1"/><rect x="2" y="16" width="6" height="6" rx="1"/><rect x="16" y="16" width="6" height="6" rx="1"/><path d="M5 16v-3a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1v3M12 12V8"/>"#
        }
        "phone" => r#"<rect x="5" y="2" width="14" height="20" rx="2"/><path d="M12 18h.01"/>"#,
        "monitor" => {
            r#"<rect x="2" y="3" width="20" height="14" rx="2"/><path d="M8 21h8M12 17v4"/>"#
        }
        "helm" => {
            r#"<circle cx="12" cy="12" r="9"/><circle cx="12" cy="12" r="2.5"/><path d="M12 3v6.5M12 14.5V21M3 12h6.5M14.5 12H21M5.6 5.6l4.6 4.6M13.8 13.8l4.6 4.6M18.4 5.6l-4.6 4.6M10.2 13.8l-4.6 4.6"/>"#
        }
        "folder" => {
            r#"<path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/>"#
        }
        "wrench" => {
            r#"<path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94z"/>"#
        }
        "cloud" => r#"<path d="M17.5 19H9a7 7 0 1 1 6.71-9h1.79a4.5 4.5 0 1 1 0 9Z"/>"#,
        "zap" => r#"<path d="M13 2 3 14h9l-1 8 10-12h-9z"/>"#,
        "coffee" => {
            r#"<path d="M10 2v2M14 2v2M6 2v2"/><path d="M16 8a1 1 0 0 1 1 1v8a4 4 0 0 1-4 4H7a4 4 0 0 1-4-4V9a1 1 0 0 1 1-1h14a4 4 0 1 1 0 8h-1"/>"#
        }
        "cpu" => {
            r#"<rect x="4" y="4" width="16" height="16" rx="2"/><rect x="9" y="9" width="6" height="6"/><path d="M15 2v2M15 20v2M2 15h2M2 9h2M20 15h2M20 9h2M9 2v2M9 20v2"/>"#
        }
        "chart" => r#"<path d="M3 3v16a2 2 0 0 0 2 2h16M18 17V9M13 17V5M8 17v-3"/>"#,
        "shield" => r#"<path d="M12 3l8 3v6c0 5-3.5 8-8 9-4.5-1-8-4-8-9V6z"/>"#,
        "music" => {
            r#"<path d="M9 18V5l12-2v13"/><circle cx="6" cy="18" r="3"/><circle cx="18" cy="16" r="3"/>"#
        }
        "github" => {
            r#"<path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.4 5.4 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"/><path d="M9 18c-4.51 2-5-2-7-2"/>"#
        }
        "search" => r#"<circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/>"#,
        "arrow-down" => r#"<path d="M12 5v14M19 12l-7 7-7-7"/>"#,
        "play" => r#"<path d="M6 3l14 9-14 9z"/>"#,
        "lightbulb" => {
            r#"<path d="M15 14c.2-1 .7-1.7 1.5-2.5 1-.9 1.5-2.2 1.5-3.5A6 6 0 0 0 6 8c0 1 .2 2.2 1.5 3.5.7.7 1.3 1.5 1.5 2.5"/><path d="M9 18h6M10 22h4"/>"#
        }
        _ => return None,
    })
}

/// Icon name for an emoji used in the page content, if there is one.
fn name_for_emoji(glyph: &str) -> Option<&'static str> {
    Some(match glyph.trim_end_matches('\u{fe0f}') {
        "🌐" => "globe",
        "⚙" => "network",
        "📱" => "phone",
        "🖥" => "monitor",
        "☸" => "helm",
        "📁" => "folder",
        "🔧" => "wrench",
        "☁" => "cloud",
        "⚡" => "zap",
        "☕" => "coffee",
        "🦀" => "cpu",
        "📊" => "chart",
        "🛡" => "shield",
        "🎵" => "music",
        "🐙" => "github",
        "🔍" => "search",
        "⬇" => "arrow-down",
        "💡" => "lightbulb",
        _ => return None,
    })
}

/// Inline SVG icon that follows the surrounding text colour. Sized 1em by default.
#[component]
pub fn Icon(name: &'static str, #[prop(optional)] class: &'static str) -> impl IntoView {
    let size = if class.is_empty() {
        "w-[1em] h-[1em]"
    } else {
        class
    };
    view! {
        <svg
            class=format!("inline-block shrink-0 align-[-0.125em] {size}")
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="1.75"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
            inner_html=markup(name).unwrap_or_default()
        ></svg>
    }
}

/// Renders the SVG icon matching an emoji from the content, or the emoji itself if none matches.
#[component]
pub fn EmojiIcon(glyph: &'static str, #[prop(optional)] class: &'static str) -> impl IntoView {
    match name_for_emoji(glyph) {
        Some(name) => view! { <Icon name=name class=class /> }.into_any(),
        None => view! { <span>{glyph}</span> }.into_any(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMOJI: [&str; 18] = [
        "🌐", "⚙️", "📱", "🖥️", "☸️", "📁", "🔧", "☁️", "⚡", "☕", "🦀", "📊", "🛡️", "🎵", "🐙",
        "🔍", "⬇️", "💡",
    ];

    #[test]
    fn every_emoji_has_an_icon_with_markup() {
        for glyph in EMOJI {
            let name = name_for_emoji(glyph).unwrap_or_else(|| panic!("no icon for {glyph}"));
            assert!(markup(name).is_some(), "no markup for {name}");
        }
    }

    #[test]
    fn unknown_emoji_has_no_icon() {
        assert!(name_for_emoji("🍕").is_none());
        assert!(markup("nope").is_none());
    }
}
