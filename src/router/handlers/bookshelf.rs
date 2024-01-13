use askama::Template;
use axum::response::IntoResponse;

pub async fn bookshelf() -> impl IntoResponse {
    BookshelfTemplate {
        sections: vec![
            Section {
                title: "Disciplines",
                subtitle: "Disciplines that I'm interested in and want to learn more about.",
                content: vec![
                    ContentEntry {
                        title: "The Twelve Factor App",
                        href: "https://12factor.net/",
                        description: "A methodology for building modern, scalable, maintainable software-as-a-service apps.",
                    },
                ],
            },
            Section {
                title: "Articles",
                subtitle: "Articles that I've read and found interesting.",
                content: vec![
                    ContentEntry {
                        title: "The Grug Brained Developer",
                        href: "https://grugbrain.dev/",
                        description: "A layman's guide to thinking like the self-aware smol brained.",
                    },
                    ContentEntry {
                        title: "HTMX on Locality of Behavior",
                        href: "https://htmx.org/essays/locality-of-behaviour/",
                        description: "An article on the tradeoffs of Separation of Concerns and Locality of Behavior.",
                    },
                    ContentEntry {
                        title: "The Website vs. Web App Dichotomy Doesn't Exist",
                        href: "https://jakelazaroff.com/words/the-website-vs-web-app-dichotomy-doesnt-exist",
                        description: "An analysis of websites across two axes: Static vs Dynamic & Online vs Offline."
                    }
                ],
            },
            Section {
                title: "Cool Tools",
                subtitle: "Tools I like to use or think are interesting",
                content: vec![
                    ContentEntry {
                        title: "Photopea",
                        href: "https://www.photopea.com/",
                        description: "Free photo editing in the browser."
                    },
                    ContentEntry {
                        title: "Obsidian",
                        href: "https://obsidian.md/",
                        description: "Obsidian is the private and flexible writing app that adapts to the way you think."
                    }
                ]
            }
        ],
    }
}


//region Structs
#[derive(Template)]
#[template(path = "pages/bookshelf.html")]
struct BookshelfTemplate {
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
    description: &'static str,
}
//endregion