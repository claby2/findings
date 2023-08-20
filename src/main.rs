#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use serde::{Deserialize, Serialize};

fn main() {
    #[cfg(feature = "ssr")]
    {
        LaunchBuilder::new(App)
            .addr(std::net::SocketAddr::from(([0, 0, 0, 0], 8080)))
            .launch();
    }
    #[cfg(not(feature = "ssr"))]
    {
        LaunchBuilder::new(App).launch();
    }
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Finding {
    title: String,
    link: String,
}

fn Findings(cx: Scope) -> Element {
    let findings = use_future(cx, (), |_| get_findings());
    return cx.render(match findings.value() {
        Some(Ok(findings)) => rsx! {
            ul { class: "list-disc list-inside",
                for finding in findings {
                    li { class: "mb-1 text-gray-400",
                        a {
                            class: "text-blue-400 hover:underline",
                            href: "{finding.link}",
                            "{finding.title}"
                        }
                    }
                }
            }
        },
        Some(Err(err)) => rsx! { p { class: "text-red-400", "Error: {err}" } },
        None => rsx! { p { "Loading..." } },
    });
}

#[server]
async fn get_findings() -> Result<Vec<Finding>, ServerFnError> {
    use std::fs;
    use std::path::Path;

    let path = Path::new("dist/data/findings.txt");
    let text = fs::read_to_string(path)?;

    let mut findings = text
        .lines()
        .filter_map(|line| {
            if let Some((link, title)) = line.split_once(' ') {
                Some(Finding {
                    title: title.to_string(),
                    link: link.to_string(),
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    findings.sort_by(|a, b| a.title.cmp(&b.title));
    Ok(findings)
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        main { class: "bg-gradient-to-b from-gray-800 to-gray-900 min-h-screen dark",
            div { class: "mx-auto max-w-screen-sm p-8 text-gray-200",
                h1 { class: "font-bold text-3xl mb-4", "Interesting Findings" }
                p { class: "mb-6 text-gray-400",
                    "A collection of interesting links I've accrued over the years."
                }
                Findings {}
            }
        }
    })
}
