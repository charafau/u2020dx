use dioxus::prelude::*;

use crate::models::fetch_repositories;

mod models;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div { class: "min-h-screen bg-gray-50 dark:bg-gray-900",
            RepoListView {}
        }
    }
}

#[component]
pub fn RepoListView() -> Element {
    let mut repos = use_resource(move || async move { fetch_repositories().await });

    rsx! {
        div { class: "max-w-4xl mx-auto  pb-20",
            div {
                class: "flex items-center justify-between px-6 mb-4",
                background_color: "#0f1116",
                h2 { class: "text-xl  font-bold mb-8 text-white-800 dark:text-gray-100 flex items-center gap-2",
                    "Top Repositories"
                }
            }
            match &*repos.read_unchecked() {
                Some(Ok(items)) => rsx! {
                    div { class: "grid grid-cols-1 sm:grid-cols-2 gap-6 px-6 py-2",
                        for repo in items {
                            RepoCard { repo: repo.clone() }
                        }
                    }
                },
                Some(Err(err)) => rsx! {
                    div { class: "bg-red-50 dark:bg-red-900/20 p-6 rounded-2xl text-red-600 dark:text-red-400 border border-red-100 dark:border-red-900/30",
                        h3 { class: "font-bold mb-2", "Error Loading Repositories" }
                        p { "{err}" }
                        button {
                            class: "mt-4 px-4 py-2 bg-red-600 text-white rounded-lg text-sm font-medium hover:bg-red-700 transition-colors",
                            onclick: move |_| repos.restart(),
                            "Try Again"
                        }
                    }
                },
                None => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-4",
                        div { class: "animate-spin rounded-full h-10 w-10 border-4 border-blue-500 border-t-transparent" }
                        p { class: "text-gray-500 animate-pulse", "Fetching repositories..." }
                    }
                }
            }
        }
    }
}

#[component]
fn RepoCard(repo: models::Repo) -> Element {
    rsx! {
        a {
            href: "{repo.html_url}",
            target: "_blank",
            class: "group flex flex-col p-6 bg-white dark:bg-gray-800 rounded-2xl shadow-sm hover:shadow-xl transition-all duration-300 border border-gray-200 dark:border-gray-700 hover:-translate-y-1 active:scale-[0.98]",
            div { class: "flex items-center gap-3 mb-4",
                img {
                    src: "{repo.owner.avatar_url}",
                    class: "w-10 h-10 rounded-full ring-2 ring-blue-50 dark:ring-blue-900/30",
                }
                div { class: "flex-1 min-w-0",
                    h3 { class: "text-base font-bold text-gray-900 dark:text-white truncate group-hover:text-blue-600 transition-colors",
                        "{repo.name}"
                    }
                    p { class: "text-xs text-gray-500 dark:text-gray-400",
                        "by {repo.owner.login}"
                    }
                }
            }
            div { class: "mt-auto flex items-center gap-4 text-xs font-semibold text-gray-600 dark:text-gray-300",
                if let Some(lang) = &repo.language {
                    span { class: "flex items-center gap-1.5",
                        span { class: "w-2.5 h-2.5 rounded-full bg-blue-500" }
                        "{lang}"
                    }
                }
                span { class: "flex items-center gap-1",
                    "⭐ {repo.stargazers_count}"
                }
                span { class: "flex items-center gap-1",
                    "🍴 {repo.forks}"
                }
            }
        }
    }
}
