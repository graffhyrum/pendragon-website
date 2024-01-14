use askama::{Template};
use axum::response::IntoResponse;

pub async fn my_work() -> impl IntoResponse {
    MyWorkTemplate {
        sections: vec![
            Section {
                title: "Projects",
                subtitle: "This is a collection of projects that I have worked on.",
                content: vec![
                    ContentEntry {
                        title: "This Website",
                        href: "https://github.com/graffhyrum/pendragon-website",
                        content: "This website is built using Rust, Axum, and Askama.",
                    },
                    ContentEntry {
                        title: "Playwright JSON Summary Reporter",
                        href: "https://github.com/graffhyrum/playwright-json-summary-reporter",
                        content: "A custom reporter for Playwright that outputs a JSON summary of test results.
        I contributed to this project by adding a new feature that allows the user to specify a custom output file
        path.",
                    },
                    ContentEntry {
                        title: "Playwright Project Builder",
                        href: "https://github.com/graffhyrum/playwright-project-builder",
                        content: "A Playwright test project config factory. It is designed to
be a starting point for Playwright projects. By setting environment variables for
each environment you would want to test, you can execute a setup project,
your test suite against all enabled environments, and a teardown project. ",
                    },
                    ContentEntry {
                        title: "OvationCXM",
                        href: "https://www.ovationcxm.com/",
                        content: "A customer experience management platform that helps businesses collect and analyze customer feedback.
        I contributed to this project by triaging (and sometimes fixing) bugs, and was the primary developer for the automated testing framework.",
                    },
                ],
            },
            Section {
                title: "Gists",
                subtitle: "This is a collection of gists that I have created.",
                content: vec![
                    ContentEntry {
                        title: "Typescript - Recursive Partial Type",
                        href: "https://gist.github.com/graffhyrum/7f267cea2021ad4246be23ec5f6d4a4e",
                        content: r#"<script src="https://gist.github.com/graffhyrum/7f267cea2021ad4246be23ec5f6d4a4e.js"></script>"#,
                    },
                    ContentEntry {
                        title: "Typescript - 'One of' Type",
                        href: "https://gist.github.com/graffhyrum/d705dc05cf3890303dd9aa1c9598b08d",
                        content: r#"<script src="https://gist.github.com/graffhyrum/d705dc05cf3890303dd9aa1c9598b08d.js"></script>"#,
                    },
                    ContentEntry {
                        title: "Typescript - Type-safe Entries",
                        href: "https://gist.github.com/graffhyrum/1253b24fbe80d5f508544736d83d9532",
                        content: r#"<script src="https://gist.github.com/graffhyrum/1253b24fbe80d5f508544736d83d9532.js"></script>"#,
                    },
                    ContentEntry {
                        title: "Typescript - Branded Types",
                        href: "https://gist.github.com/graffhyrum/bdf39a9e7fe18876fcc1dabf11c92457",
                        content: r#"<script src="https://gist.github.com/graffhyrum/bdf39a9e7fe18876fcc1dabf11c92457.js"></script>"#,
                    },
                ],
            },
        ],
    }
}

#[derive(Template)]
#[template(path = "pages/my_work.html")]
struct MyWorkTemplate {
    sections: Vec<Section>,
}

struct Section {
    title: &'static str,
    subtitle: &'static str,
    content: Vec<ContentEntry>,
}

struct ContentEntry {
    title: &'static str,
    href: &'static str,
    // this is marked as 'safe' in the template. be sure to use a raw string literal (r#""#)
    // if you want to include HTML tags in the content.
    content: &'static str,
}

