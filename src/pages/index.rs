use askama::Template;

use crate::assets;
use crate::content::{self, Link, Project, Role, Site, SkillGroup};

#[derive(Template)]
#[template(path = "pages/index.html")]
pub struct IndexTemplate {
    pub site: &'static Site,
    pub tenure: &'static str,
    pub experience: &'static [Role],
    pub projects: &'static [Project],
    pub skills: &'static [SkillGroup],
    pub links: &'static [Link],
    /// Content-hash cache-busting tokens, so versioned asset URLs can be immutable.
    pub css_v: String,
    pub js_v: String,
}

impl IndexTemplate {
    pub fn new() -> Self {
        Self {
            site: &content::SITE,
            tenure: content::tenure(),
            experience: content::EXPERIENCE,
            projects: content::PROJECTS,
            skills: content::SKILLS,
            links: content::LINKS,
            css_v: assets::fingerprint("app.generated.css"),
            js_v: assets::fingerprint("js/reveal.js"),
        }
    }
}

impl Default for IndexTemplate {
    fn default() -> Self {
        Self::new()
    }
}
