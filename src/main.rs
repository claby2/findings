#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(App);
}

#[derive(PartialEq)]
struct Finding {
    title: String,
    link: String,
}

#[inline_props]
fn Findings<'a>(cx: Scope, findings: &'a [Finding]) -> Element {
    cx.render(rsx! {
        h1 {
            class: "font-bold text-3xl mb-4",
            "Interesting Findings"
        }
        p {
            class: "mb-6 text-gray-400",
            "A collection of interesting links I've accrued over the years."
        }
        ul {
            class: "list-disc list-inside",
            for finding in findings {
                li {
                    class: "mb-1 text-gray-400",
                    a {
                        class: "text-blue-400 hover:underline",
                        href: "{finding.link}",
                        "{finding.title}"
                    }
                }
            }
        }
    })
}

async fn get_findings() -> Result<Vec<Finding>, gloo_net::Error> {
    let resp = gloo_net::http::Request::get("/findings.txt").send().await?;

    let mut findings = vec![];

    let text = resp.text().await?;
    for line in text.lines() {
        if let Some((link, title)) = line.split_once(' ') {
            findings.push(Finding {
                title: title.to_string(),
                link: link.to_string(),
            });
        }
    }

    findings.sort_by(|a, b| a.title.cmp(&b.title));
    Ok(findings)
}

fn App(cx: Scope) -> Element {
    let findings = use_future(cx, (), |_| get_findings());
    let element = if let Some(findings) = findings.value() {
        match findings {
            Ok(findings) => cx.render(rsx! {
                Findings { findings: findings }
            }),
            Err(err) => cx.render(rsx! {
                p { "Error: {err}" }
            }),
        }
    } else {
        cx.render(rsx! {
            p { "Loading..." }
        })
    };
    cx.render(rsx! {
        main {
            class: "bg-gradient-to-b from-gray-800 to-gray-900 min-h-screen dark",
            div {
                class: "mx-auto max-w-screen-sm p-8 text-gray-200",
                element
            }
        }
    })
}
