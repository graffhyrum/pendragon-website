use askama::Template;
use axum::response::IntoResponse;

pub async fn shoutouts() -> impl IntoResponse {
    ShoutoutsTemplate {
        sections: vec![
            Section {
                title: "Shoutouts",
                subtitle: "Content creators that I love and admire.  I would not be where I am today without them. Thank you all!",
                content: vec![
                    ContentEntry {
                        title: "Michael B Paulson AKA Primeagen, The",
                        link: vec![Link {
                            title: "Youtube",
                            href: "https://www.youtube.com/@ThePrimeagen",
                        }],
                        description: "My favorite VIM content creator and second favorite coding sh*tposter. BTW he works at Netflix, just so you know... Netflix btw... =D",
                    },
                    ContentEntry {
                        title: "Fireship",
                        link: vec![Link {
                            title: "Main Website",
                            href: "https://fireship.io/",
                        }, Link {
                            title: "Youtube",
                            href: "https://www.youtube.com/c/fireship",
                        }],
                        description: "Succinct and entertaining tech news and code tutorials.",
                    },
                    ContentEntry {
                        title: "Robert Martin AKA Uncle Bob",
                        link: vec![Link {
                            title: "Wiki Page",
                            href: "https://en.wikipedia.org/wiki/Robert_C._Martin",
                        }],
                        description: "A very old programmer, likes to talk about punch cards and Star Trek. He also invented 4/5ths of the SOLID principles or something. On an unrelated note, how do YOU feel about Test Driven Development?",
                    },
                    ContentEntry {
                        title: "Matt Pocock",
                        link: vec![Link {
                            title: "Their Website",
                            href: "https://www.mattpocock.com/",
                        }],
                        description: "My favorite Typescript content creator (sorry Prime). Good youtube content and lots of useful code snippets that I've *definitely not stolen* for my gist page.",
                    },
                    ContentEntry {
                        title: "Triss AKA NoBoilerplate",
                        link: vec![Link {
                            title: "Youtube",
                            href: "https://www.youtube.com/@NoBoilerplate",
                        }],
                        description: "Rust evangelist, ADHD life-hacker, and overall phenomenal content creator. Can't say enough good things. Thanks for telling me about Obsidian!",
                    },
                    ContentEntry {
                        title: "Jeremy Chone",
                        link: vec![Link {
                            title: "His Website",
                            href: "https://jeremychone.com/",
                        }],
                        description: "My second favorite Rust content creator (sorry again, Prime. You got the first entry in the list, don't be mad.). I would have taken WAY longer to make this site without his content.",
                    },
                    ContentEntry {
                        title: "Kai Lentit AKA Programmers are also human",
                        link: vec![Link {
                            title: "Twitter/X",
                            href: "https://twitter.com/kailentit",
                        }, Link {
                            title: "Youtube",
                            href: "https://www.youtube.com/@programmersarealsohuman5909",
                        }],
                        description: "Satirical Developer 'interviews'",
                    },
                    ContentEntry {
                        title: "Gleb Bahmutov",
                        link: vec![Link {
                            title: "His Website",
                            href: "https://glebbahmutov.com/",
                        }],
                        description: "My favorite QA Engineering content creator. He made a ton of content on Cypress that kept me employed through my first QAE role.",
                    },
                ],
            },
        ],
    }
}


//region Structs
#[derive(Template)]
#[template(path = "pages/shoutouts.html")]
struct ShoutoutsTemplate {
    sections: Vec<Section>,
}

struct Section {
    title: &'static str,
    subtitle: &'static str,
    content: Vec<ContentEntry>,
}

struct ContentEntry {
    title: &'static str,
    link: Vec<Link>,
    description: &'static str,
}

struct Link {
    title: &'static str,
    href: &'static str,
}
//endregion