use crate::components::{EmojiIcon, SectionHeading};
use crate::config::{PortfolioConfig, ProjectItem, project_link_label};
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
        <section id="projects" class="reveal-in space-y-5">
            <SectionHeading eyebrow="Selected Work" title="Projects" />

            // Project Filter Controls
            <div class="flex gap-2 overflow-x-auto sm:overflow-visible sm:flex-wrap pb-2 sm:pb-0 custom-scrollbar">
                <button
                    type="button"
                    on:click=move |_| set_selected_tech.set(None)
                    class=move || {
                        if selected_tech.get().is_none() {
                            "shrink-0 whitespace-nowrap px-3 py-1 rounded-md font-mono text-xs font-medium border transition-colors duration-200 cursor-pointer bg-[var(--color-accent-solid)] border-[var(--color-accent-solid)] text-white"
                        } else {
                            "shrink-0 whitespace-nowrap px-3 py-1 rounded-md font-mono text-xs font-medium border transition-colors duration-200 cursor-pointer bg-[var(--color-bg-surface)] border-[var(--color-border-primary)] text-[var(--color-text-tertiary)] hover:text-[var(--color-hover-text)] hover:border-[var(--color-border-secondary)]"
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
                            "shrink-0 whitespace-nowrap px-3 py-1 rounded-md font-mono text-xs font-medium border transition-colors duration-200 cursor-pointer bg-[var(--color-accent-solid)] border-[var(--color-accent-solid)] text-white"
                        } else {
                            "shrink-0 whitespace-nowrap px-3 py-1 rounded-md font-mono text-xs font-medium border transition-colors duration-200 cursor-pointer bg-[var(--color-bg-surface)] border-[var(--color-border-primary)] text-[var(--color-text-tertiary)] hover:text-[var(--color-hover-text)] hover:border-[var(--color-border-secondary)]"
                        }
                    };
                    view! {
                        <button
                            type="button"
                            on:click=move |_| toggle_tech(&tag_for_click)
                            class=btn_class
                        >
                            {tag_str}
                        </button>
                    }
                }).collect_view()}
            </div>

            // Featured Card + Grid Layout
            <div class="space-y-5">
                // Featured Card: DevOps Control Center
                {move || {
                    let current_filter = selected_tech.get();
                    let matches_devops = match &current_filter {
                        None => true,
                        Some(f) => devops_project.tech.contains(f),
                    };

                    if matches_devops {
                        Some(view! {
                            <div class="card-surface p-6 md:p-7 rounded-xl border-[var(--color-border-secondary)]">
                                <div class="flex flex-col md:flex-row md:items-center justify-between gap-6">
                                    <div class="flex-1 space-y-3.5">
                                        <div class="flex items-center space-x-2">
                                            <span class="font-mono text-[11px] font-medium text-[var(--color-accent)] uppercase tracking-[0.12em]">
                                                "Featured"
                                            </span>
                                            <span class="text-[var(--color-text-faint)]">"·"</span>
                                            <span class="text-sm text-[var(--color-text-muted)]"><EmojiIcon glyph="⚙️" /></span>
                                        </div>
                                        <h4 class="text-xl md:text-2xl font-bold text-[var(--color-text-primary)] tracking-tight">
                                            {devops_project.title.as_str()}
                                        </h4>
                                        <p class="text-[var(--color-text-secondary)] text-sm sm:text-[15px] leading-relaxed">
                                            {devops_project.description.as_str()}
                                        </p>

                                        <div class="flex flex-wrap gap-1.5 pt-1">
                                            {devops_project.tech.iter().map(|tech| {
                                                let tech_str = tech.as_str();
                                                let is_tag_active = move || selected_tech.get().as_deref() == Some(tech_str);
                                                let tag_class = move || {
                                                    if is_tag_active() {
                                                        "border font-mono text-[11px] px-2 py-0.5 rounded-md cursor-pointer transition-colors duration-150 bg-[var(--color-bg-tag-active)] border-[var(--color-accent-border)] text-[var(--color-accent)]"
                                                    } else {
                                                        "border font-mono text-[11px] px-2 py-0.5 rounded-md cursor-pointer transition-colors duration-150 bg-[var(--color-bg-tag)] border-[var(--color-border-primary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] hover:border-[var(--color-border-secondary)]"
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

                                    <div class="flex flex-col justify-center space-y-2.5 w-full md:w-60 flex-shrink-0">
                                        <button
                                            type="button"
                                            on:click=move |_| set_show_case_study.set(true)
                                            class="px-4 py-2.5 bg-[var(--color-accent-solid)] hover:bg-[var(--color-accent-solid-hover)] text-white rounded-lg text-xs font-semibold transition-colors duration-200 w-full cursor-pointer flex items-center justify-center space-x-1.5"
                                        >
                                            <EmojiIcon glyph="⚙️" />
                                            <span>"System Design & Demo"</span>
                                        </button>

                                        <a
                                            href=devops_project.link.as_str()
                                            target="_blank"
                                            rel="noopener noreferrer"
                                            class="px-4 py-2.5 bg-[var(--color-bg-surface-elevated)] border border-[var(--color-border-primary)] hover:bg-[var(--color-hover-bg-strong)] hover:border-[var(--color-border-secondary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] rounded-lg text-xs font-medium transition-colors duration-200 w-full text-center flex items-center justify-center space-x-1.5"
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
                                                    class="px-4 py-2.5 bg-[var(--color-bg-surface-elevated)] border border-[var(--color-border-primary)] hover:bg-[var(--color-hover-bg-strong)] hover:border-[var(--color-border-secondary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] rounded-lg text-xs font-medium transition-colors duration-200 w-full text-center flex items-center justify-center space-x-1.5"
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
                <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
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
    let emoji = if project.title.contains("Flutter") {
        "📱"
    } else if project.title.contains("Portfolio") {
        "🌐"
    } else if project.title.contains("WSL") {
        "🖥️"
    } else if project.title.contains("GitOps") {
        "☸️"
    } else {
        "📁"
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
        if is_card_matched() {
            "card-surface p-6 rounded-xl border-[var(--color-accent-border)] transition-colors duration-200 flex flex-col justify-between md:[&:last-child:nth-child(odd)]:col-span-2"
        } else {
            "card-surface p-6 rounded-xl hover:border-[var(--color-border-secondary)] transition-colors duration-200 flex flex-col justify-between md:[&:last-child:nth-child(odd)]:col-span-2"
        }
    };

    view! {
        <div class=card_class>
            <div>
                <div class="flex items-center space-x-2 mb-2">
                    <span class="text-base text-[var(--color-accent)]"><EmojiIcon glyph=emoji /></span>
                    <h4 class="text-base sm:text-lg font-bold text-[var(--color-text-primary)] tracking-tight">
                        {title.as_str()}
                    </h4>
                </div>
                <p class="text-[var(--color-text-tertiary)] text-sm leading-relaxed mb-5">
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
                                "border font-mono text-[11px] px-2 py-0.5 rounded-md cursor-pointer transition-colors duration-150 bg-[var(--color-bg-tag-active)] border-[var(--color-accent-border)] text-[var(--color-accent)]"
                            } else {
                                "border font-mono text-[11px] px-2 py-0.5 rounded-md cursor-pointer transition-colors duration-150 bg-[var(--color-bg-tag)] border-[var(--color-border-primary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] hover:border-[var(--color-border-secondary)]"
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
                    class="px-4 py-2 border border-[var(--color-border-secondary)] rounded-lg text-xs font-medium text-[var(--color-text-primary)] hover:text-[var(--color-accent)] hover:border-[var(--color-accent-border)] hover:bg-[var(--color-accent-bg)] inline-flex items-center transition-colors duration-200 w-full justify-center"
                >
                    {project_link_label(project)} <span class="ml-1.5">"→"</span>
                </a>

                {project.incidents_link.as_deref().map(|incidents| {
                    view! {
                        <a
                            href=incidents
                            target="_blank"
                            rel="noopener noreferrer"
                            class="px-4 py-2 border border-[var(--color-border-primary)] rounded-lg text-xs font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] hover:bg-[var(--color-hover-bg-strong)] hover:border-[var(--color-border-secondary)] inline-flex items-center transition-colors duration-200 w-full justify-center mt-2"
                        >
                            "Incident Write-up" <span class="ml-1.5">"→"</span>
                        </a>
                    }
                })}
            </div>
        </div>
    }
}
