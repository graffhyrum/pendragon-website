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
                        description: "An analysis of websites across two axes: Static vs Dynamic & Online vs Offline.",
                    },
                ],
            },
            Section {
                title: "Cool Tools",
                subtitle: "Tools I like to use or think are interesting",
                content: vec![
                    ContentEntry {
                        title: "Test Automation University",
                        href: "https://testautomationu.applitools.com/",
                        description: "An excellent, free, training resource for learning QA automation.",
                    },
                    ContentEntry {
                        title: "Photopea",
                        href: "https://www.photopea.com/",
                        description: "Free photo editing in the browser.",
                    },
                    ContentEntry {
                        title: "Obsidian",
                        href: "https://obsidian.md/",
                        description: "Obsidian is the private and flexible writing app that adapts to the way you think.",
                    },
                ],
            },
            Section {
                title: "Shoutouts",
                subtitle: "Content creators that I love and admire.  I would not be where I am today without them. Thank you all!",
                content: vec![
                    ContentEntry {
                        title: "Michael B Paulson AKA Primeagen, The",
                        href: "https://www.youtube.com/@ThePrimeagen",
                        description: "My favorite VIM content creator and second favorite coding sh*tposter. BTW he works at Netflix, just so you know... Netflix btw... =D",
                    },
                    ContentEntry {
                        title: "Robert Martin AKA Uncle Bob",
                        href: "https://en.wikipedia.org/wiki/Robert_C._Martin",
                        description: "A very old programmer, likes to talk about punch cards and Star Trek. He also invented 4/5ths of the SOLID principles or something. On an unrelated note, how do YOU feel about Test Driven Development?",
                    },
                    ContentEntry {
                        title: "Matt Pocock",
                        href: "https://www.mattpocock.com/",
                        description: "My favorite Typescript content creator (sorry Prime). Good youtube content and lots of useful code snippets that I've *definitely not stolen* for my gist page.",
                    },
                    ContentEntry {
                        title: "Triss AKA NoBoilerplate",
                        href: "https://www.youtube.com/@NoBoilerplate",
                        description: "Rust evangelist, ADHD life-hacker, and overall phenomenal content creator. Can't say enough good things. Thanks for telling me about Obsidian!"
                    },
                    ContentEntry {
                        title: "Jeremy Chone",
                        href: "https://jeremychone.com/",
                        description: "My second favorite Rust content creator (sorry again, Prime. You got the first entry in the list, don't be mad.). I would have taken WAY longer to make this site without his content.",
                    },
                    ContentEntry {
                        title: "Gleb Bahmutov",
                        href: "https://glebbahmutov.com/",
                        description: "My favorite QA Engineering content creator. He made a ton of content on Cypress that kept me employed through my first QAE role.",
                    },
                ],
            },
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