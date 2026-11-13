use crate::components::{EmojiIcon, SectionHeading};
use crate::config::{PortfolioConfig, ProjectItem};
use leptos::prelude::*;

#[component]
pub fn Projects(
    config: &'static PortfolioConfig,
    selected_tech: Signal<Option<String>>,
    set_selected_tech: WriteSignal<Option<String>>,
    set_show_case_study: WriteSignal<bool>,
    all_tech_tags: Vec<String>,
) -> impl IntoView {
    let devops_project = config
        .projects
        .iter()
        .find(|p| p.title == "DevOps Control Center")
        .expect("DevOps Control Center project not found in config");

    let toggle_tech = move |tech: &str| {
        if selected_tech.get().as_deref() == Some(tech) {
            set_selected_tech.set(None);
        } else {
            set_selected_tech.set(Some(tech.to_string()));
        }
    };

    view! {
        <section id="projects" class="reveal-in space-y-6">
            <SectionHeading eyebrow="Projects" title="Featured Projects" />

            // Project Filter Controls
            <div class="flex gap-2 overflow-x-auto sm:overflow-visible sm:flex-wrap pb-2 sm:pb-0 custom-scrollbar">
                <button
                    on:click=move |_| set_selected_tech.set(None)
                    class=move || {
                        if selected_tech.get().is_none() {
                            "shrink-0 whitespace-nowrap px-3 py-1 rounded-lg text-xs font-bold tracking-wide border transition-all duration-300 cursor-pointer bg-[var(--color-accent-solid)] border-[var(--color-accent-solid)] text-white shadow-[0_0_8px_var(--color-accent-border)]"
                        } else {
                            "shrink-0 whitespace-nowrap px-3 py-1 rounded-lg text-xs font-bold tracking-wide border transition-all duration-300 cursor-pointer bg-[var(--color-bg-tag)] border-[var(--color-border-primary)] text-[var(--color-text-tertiary)] hover:text-[var(--color-hover-text)]"
                        }
                    }
                >
                    "All"
                </button>
                {all_tech_tags.into_iter().map(|tag| {
                    let tag_str = tag.clone();
                    let tag_for_click = tag.clone();
                    let tag_for_active = tag.clone();
                    let is_active = move || selected_tech.get().as_ref() == Some(&tag_for_active);
                    let btn_class = move || {
                        if is_active() {
                            "shrink-0 whitespace-nowrap px-3 py-1 rounded-lg text-xs font-bold tracking-wide border transition-all duration-300 cursor-pointer bg-[var(--color-accent-solid)] border-[var(--color-accent-solid)] text-white shadow-[0_0_8px_var(--color-accent-border)]"
                        } else {
                            "shrink-0 whitespace-nowrap px-3 py-1 rounded-lg text-xs font-bold tracking-wide border transition-all duration-300 cursor-pointer bg-[var(--color-bg-tag)] border-[var(--color-border-primary)] text-[var(--color-text-tertiary)] hover:text-[var(--color-hover-text)] hover:border-[var(--color-border-secondary)]"
                        }
                    };
                    view! {
                        <button
                            on:click=move |_| toggle_tech(&tag_for_click)
                            class=btn_class
                        >
                            {tag_str}
                        </button>
                    }
                }).collect_view()}
            </div>

            // Featured Card or Grid Layout
            <div class="space-y-6">
                // Featured Card: DevOps Control Center
                {move || {
                    let current_filter = selected_tech.get();
                    let matches_devops = match &current_filter {
                        None => true,
                        Some(f) => devops_project.tech.contains(f),
                    };

                    if matches_devops {
                        Some(view! {
                            <div
                                class="hover-lift backdrop-blur-md p-6 md:p-8 rounded-2xl border-t-2 border-t-[var(--color-accent-solid)] border-x border-b border-[var(--color-border-primary)] relative overflow-hidden transition-all duration-300 group hover:border-[var(--color-accent-border)]"
                                style="background: linear-gradient(to bottom right, var(--color-accent-bg), var(--color-bg-surface), var(--color-bg-primary)); box-shadow: var(--shadow-card)"
                            >
                                <div class="absolute top-0 right-0 w-32 h-32 bg-[var(--color-accent-bg)] rounded-full blur-2xl -z-10"></div>
                                <div class="flex flex-col md:flex-row md:items-center justify-between gap-6">
                                    <div class="flex-1 space-y-4">
                                        <div class="flex items-center space-x-2">
                                            <span class="bg-[var(--color-accent-bg)] text-[var(--color-accent)] border border-[var(--color-accent-border)] px-2.5 py-0.5 rounded-full text-[10px] font-bold tracking-wider uppercase">
                                                "Featured Project"
                                            </span>
                                            <span class="text-xl"><EmojiIcon glyph="⚙️" /></span>
                                        </div>
                                        <h4 class="text-xl md:text-2xl font-bold text-[var(--color-text-primary)] group-hover:text-[var(--color-accent)] transition-colors tracking-wide">
                                            {devops_project.title.as_str()}
                                        </h4>
                                        <p class="text-[var(--color-text-secondary)] text-sm md:text-base leading-relaxed">
                                            {devops_project.description.as_str()}
                                        </p>

                                        <div class="flex flex-wrap gap-1.5 pt-2">
                                            {devops_project.tech.iter().map(|tech| {
                                                let tech_str = tech.as_str();
                                                let is_tag_active = move || selected_tech.get().as_deref() == Some(tech_str);
                                                let tag_class = move || {
                                                    if is_tag_active() {
                                                        "border text-[10px] px-2.5 py-0.5 rounded-md font-semibold tracking-wide cursor-pointer transition-all duration-200 bg-[var(--color-bg-tag-active)] border-[var(--color-accent-solid)] text-[var(--color-accent)]"
                                                    } else {
                                                        "border text-[10px] px-2.5 py-0.5 rounded-md font-semibold tracking-wide cursor-pointer transition-all duration-200 bg-[var(--color-bg-tag)] border-[var(--color-border-primary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] hover:border-[var(--color-border-secondary)]"
                                                    }
                                                };
                                                view! {
                                                    <button
                                                        type="button"
                                                        aria-pressed=move || is_tag_active().to_string()
                                                        on:click=move |_| toggle_tech(tech_str)
                                                        class=tag_class
                                                    >
                                                        {tech_str}
                                                    </button>
                                                }
                                            }).collect_view()}
                                        </div>
                                    </div>

                                    <div class="flex flex-col justify-center space-y-3 w-full md:w-64 flex-shrink-0">
                                        <button
                                            on:click=move |_| set_show_case_study.set(true)
                                            class="px-5 py-3 bg-[var(--color-accent-solid)] hover:bg-[var(--color-accent-solid-hover)] text-white rounded-xl text-xs font-bold transition-all duration-300 w-full cursor-pointer flex items-center justify-center space-x-1.5"
                                            style="box-shadow: var(--shadow-button)"
                                        >
                                            <EmojiIcon glyph="⚙️" />
                                            <span>"System Design & Demo"</span>
                                        </button>

                                        <a
                                            href=devops_project.link.as_str()
                                            target="_blank"
                                            rel="noopener noreferrer"
                                            class="px-5 py-3 bg-[var(--color-hover-bg)] border border-[var(--color-border-primary)] hover:bg-[var(--color-hover-bg)] hover:border-[var(--color-border-secondary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] rounded-xl text-xs font-bold transition-all duration-300 w-full text-center flex items-center justify-center space-x-1.5"
                                        >
                                            <span>"View Repository"</span>
                                            <span>"→"</span>
                                        </a>

                                        {devops_project.incidents_link.as_deref().map(|incidents| {
                                            view! {
                                                <a
                                                    href=incidents
                                                    target="_blank"
                                                    rel="noopener noreferrer"
                                                    class="px-5 py-3 bg-[var(--color-hover-bg)] border border-[var(--color-border-primary)] hover:bg-[var(--color-hover-bg)] hover:border-[var(--color-border-secondary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] rounded-xl text-xs font-bold transition-all duration-300 w-full text-center flex items-center justify-center space-x-1.5"
                                                >
                                                    <span>"Incident Write-ups"</span>
                                                    <span>"→"</span>
                                                </a>
                                            }
                                        })}
                                    </div>
                                </div>
                            </div>
                        })
                    } else {
                        None
                    }
                }}

                // Other Projects Grid
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6 md:items-start">
                    {move || {
                        let current_filter = selected_tech.get();
                        config.projects
                            .iter()
                            .filter(|p| p.title != "DevOps Control Center")
                            .filter(move |p| {
                                match &current_filter {
                                    None => true,
                                    Some(f) => p.tech.contains(f),
                                }
                            })
                            .map(|project| {
                                render_project_card(project, selected_tech, toggle_tech)
                            })
                            .collect_view()
                    }}
                </div>
            </div>
        </section>
    }
}

fn render_project_card<F>(
    project: &'static ProjectItem,
    selected_tech: Signal<Option<String>>,
    toggle_tech: F,
) -> impl IntoView
where
    F: Fn(&str) + Copy + 'static,
{
    let (accent_class, emoji) = if project.title.contains("Flutter") {
        ("border-t-[var(--color-accent-orange)]", "📱")
    } else if project.title.contains("Portfolio") {
        ("border-t-[var(--color-accent-solid)]", "🌐")
    } else if project.title.contains("WSL") {
        ("border-t-[var(--color-accent-gold)]", "🖥️")
    } else if project.title.contains("GitOps") {
        ("border-t-[var(--color-accent-orange)]", "☸️")
    } else {
        ("border-t-[var(--color-accent-gold)]", "📁")
    };

    let title = &project.title;
    let description = &project.description;
    let link = &project.link;

    let is_card_matched = move || {
        selected_tech
            .get()
            .as_ref()
            .map(|f| project.tech.contains(f))
            .unwrap_or(false)
    };

    let card_class = move || {
        let is_matched = is_card_matched();
        format!(
            "hover-lift bg-[var(--color-bg-surface)] backdrop-blur-md p-6 rounded-2xl border-t-2 {} border-x border-b transition-all duration-500 flex flex-col justify-between group md:[&:last-child:nth-child(odd)]:col-span-2 {}",
            accent_class,
            if is_matched {
                "border-[var(--color-accent-border)]"
            } else {
                "border-[var(--color-border-primary)] hover:border-[var(--color-accent-border)]"
            }
        )
    };

    let card_style = move || {
        if is_card_matched() {
            "box-shadow: var(--shadow-card-hover)"
        } else {
            "box-shadow: var(--shadow-card)"
        }
    };

    view! {
        <div class=card_class style=card_style>
            <div>
                <div class="flex items-center space-x-2 mb-2">
                    <span class="text-lg"><EmojiIcon glyph=emoji /></span>
                    <h4 class="text-lg font-bold text-[var(--color-text-primary)] group-hover:text-[var(--color-accent)] transition-colors tracking-wide">
                        {title.as_str()}
                    </h4>
                </div>
                <p class="text-[var(--color-text-tertiary)] text-sm leading-relaxed mb-6">
                    {description.as_str()}
                </p>
            </div>
            <div>
                <div class="flex flex-wrap gap-1.5 mb-5">
                    {project.tech.iter().map(|tech| {
                        let tech_str = tech.as_str();
                        let is_tag_active = move || selected_tech.get().as_deref() == Some(tech_str);
                        let tag_class = move || {
                            if is_tag_active() {
                                "border text-[10px] px-2.5 py-0.5 rounded-md font-semibold tracking-wide cursor-pointer transition-all duration-200 bg-[var(--color-bg-tag-active)] border-[var(--color-accent-solid)] text-[var(--color-accent)]"
                            } else {
                                "border text-[10px] px-2.5 py-0.5 rounded-md font-semibold tracking-wide cursor-pointer transition-all duration-200 bg-[var(--color-bg-tag)] border-[var(--color-border-primary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] hover:border-[var(--color-border-secondary)]"
                            }
                        };
                        view! {
                            <button
                                type="button"
                                aria-pressed=move || is_tag_active().to_string()
                                on:click=move |_| toggle_tech(tech_str)
                                class=tag_class
                            >
                                {tech_str}
                            </button>
                        }
                    }).collect_view()}
                </div>

                <a
                    href=link.as_str()
                    target="_blank"
                    rel="noopener noreferrer"
                    class="px-4 py-2 border border-[var(--color-accent-border)] rounded-lg text-xs font-semibold text-[var(--color-accent)] hover:text-[var(--color-accent-solid-hover)] hover:border-[var(--color-accent)] inline-flex items-center transition-all duration-300 w-full justify-center"
                >
                    "View Repository" <span class="ml-1">"→"</span>
                </a>

                {project.incidents_link.as_deref().map(|incidents| {
                    view! {
                        <a
                            href=incidents
                            target="_blank"
                            rel="noopener noreferrer"
                            class="px-4 py-2 border border-[var(--color-accent-border)] rounded-lg text-xs font-semibold text-[var(--color-accent)] hover:text-[var(--color-accent-solid-hover)] hover:border-[var(--color-accent)] inline-flex items-center transition-all duration-300 w-full justify-center mt-2"
                        >
                            "Incident Write-up" <span class="ml-1">"→"</span>
                        </a>
                    }
                })}
            </div>
        </div>
    }
}
