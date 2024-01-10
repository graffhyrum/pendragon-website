use axum::response::IntoResponse;
use askama::Template;

pub async fn index() -> impl IntoResponse {
    IndexTemplate {
        skills: vec![
            Skill {
                img_src:"ts_logo.png",
                img_alt: "Typescript Logo",
                title: "Typescript",
                skill_doc_link: Some("https://www.typescriptlang.org/docs/"),
            },
            Skill {
                img_src: "playwright-logo.png",
                img_alt: "Playwright Logo",
                title: "Playwright",
                skill_doc_link: Some("https://playwright.dev/"),
            },
            Skill {
                img_src:"automation.png",
                img_alt: "Test Automation Icon",
                title: "Test Automation",
                skill_doc_link: None,
            },
            Skill {
                img_src:"documentation.png",
                img_alt: "Documentation Icon",
                title: "Documentation",
                skill_doc_link: None,
            },
            Skill {
                img_src:"process_improvement.png",
                img_alt: "Process Improvement Icon",
                title: "Process Improvement",
                skill_doc_link: None,
            },
            Skill {
                img_src:"planning.png",
               img_alt: "Strategic Planning Icon",
                title: "Strategic Planning",
                skill_doc_link: None,
            },
            Skill {
                img_src:"leadership.png",
                img_alt: "Leadership Icon",
                title: "Team Leadership",
                skill_doc_link: None,
            },
            Skill {
                img_src:"project_management.png",
                img_alt: "JIRA Logo",
                title: "Project Management",
                skill_doc_link: None,
            },
            Skill {
                img_src:"public_speaking.png",
                img_alt: "Public Speaking Icon",
                title: "Public Speaking",
                skill_doc_link: None,
            },
            Skill {
                img_src:"accessibility.png",
                img_alt: "Accessibility Icon",
                title: "Accessibility",
                skill_doc_link: None,
            },
        ]
    }
}

#[derive(Template)]
#[template(path = "pages/index.html")]
struct IndexTemplate {
    skills: Vec<Skill>,
}

struct Skill {
    img_src: &'static str,
    img_alt: &'static str,
    title: &'static str,
    skill_doc_link: Option<&'static str>
}
