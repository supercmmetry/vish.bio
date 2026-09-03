//! Site content as `&'static` data.
//!
//! Repeated collections (roles, projects, skills, links) live here so the templates can
//! loop over them; one-off prose stays as literal HTML in `templates/sections/`.

use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------------------

pub struct Site {
    pub name: &'static str,
    pub role: &'static str,
    pub employer: &'static str,
    pub location: &'static str,
    pub email: &'static str,
    pub description: &'static str,
    pub canonical: &'static str,
    pub theme_color: &'static str,
}

pub struct Highlight {
    pub headline: &'static str,
    pub body: &'static str,
}

pub struct Role {
    pub company: &'static str,
    pub title: &'static str,
    /// Machine-readable start, for `<time datetime>`.
    pub start_iso: &'static str,
    /// Human-readable span, e.g. "Sept 2024 — Present".
    pub period: &'static str,
    /// Team or product line, when it adds something.
    pub context: Option<&'static str>,
    pub highlights: &'static [Highlight],
    pub stack: &'static [&'static str],
    /// Renders in the smaller, condensed treatment.
    pub minor: bool,
}

pub struct Project {
    pub name: &'static str,
    pub url: &'static str,
    pub blurb: &'static str,
    pub tags: &'static [&'static str],
}

pub struct SkillGroup {
    pub label: &'static str,
    pub items: &'static [&'static str],
}

pub struct Link {
    pub label: &'static str,
    pub href: &'static str,
    pub handle: &'static str,
}

/// One state of the coder illustration in the sticky companion column.
///
/// Lives here rather than in the markup for the same reason every other repeated
/// collection does: it is content. The template renders each scene into a `<template>`
/// element and `assets/js/coder.js` swaps between them as sections scroll past.
pub struct Scene {
    /// Matches the `data-scene` on the section that activates it; `"intro"` is the
    /// default baked into the page for the no-JS and above-the-fold case.
    pub id: &'static str,
    /// Drives the figure's posture — see the `.coder--*` rules in styles/app.css.
    pub pose: &'static str,
    /// Window chrome caption, in the manner of a title bar.
    pub caption: &'static str,
    /// Typed out one character at a time. Kept short: the panel is ~22 columns wide at
    /// its narrowest, and anything longer wraps into a mess.
    pub lines: &'static [&'static str],
}

// ---------------------------------------------------------------------------------------
// Tenure — derived, never hardcoded
// ---------------------------------------------------------------------------------------

/// 2022-07-01T00:00:00Z — the start of full-time work at Microsoft.
const CAREER_START_UNIX: u64 = 1_656_633_600;
/// Mean Gregorian year (365.2425 days).
const SECONDS_PER_YEAR: u64 = 31_556_952;

fn years_since_joining() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|now| now.as_secs().saturating_sub(CAREER_START_UNIX) / SECONDS_PER_YEAR)
        .unwrap_or(4)
}

/// Prose-ready tenure, so the hero copy can never go stale the way the PDF did.
pub fn tenure() -> &'static str {
    match years_since_joining() {
        0 | 1 => "the past year",
        2 => "the past two years",
        3 => "the past three years",
        4 => "the past four years",
        5 => "the past five years",
        6 => "the past six years",
        7 => "the past seven years",
        8 => "the past eight years",
        9 => "the past nine years",
        _ => "the past decade",
    }
}

// ---------------------------------------------------------------------------------------
// Content
// ---------------------------------------------------------------------------------------

pub static SITE: Site = Site {
    name: "Vishaal Selvaraj",
    role: "Senior Software Engineer",
    employer: "Microsoft",
    location: "Bangalore, India",
    email: "vishaals2000@gmail.com",
    description: "Vishaal Selvaraj — senior software engineer at Microsoft working on the \
                  Spark query engine behind Microsoft Fabric. Query engines, cluster \
                  schedulers and compression formats.",
    canonical: "https://vish.bio/",
    theme_color: "#faf8f4",
};

pub static EXPERIENCE: &[Role] = &[
    Role {
        company: "Microsoft",
        title: "Senior Software Engineer",
        start_iso: "2026-09",
        period: "Sept 2026 — Present",
        context: Some("Microsoft Fabric · Spark query engine"),
        minor: false,
        highlights: &[Highlight {
            headline: "Making Spark stop recomputing what it already knows.",
            body: "I led the research, design and development of materialized views and \
                   incremental view maintenance inside the Spark query engine that powers \
                   Microsoft Fabric. Medallion-architecture pipelines re-derive the same \
                   tables endlessly; incremental view maintenance updates only what actually \
                   changed. It is the kind of capability that decides whether a data platform \
                   is competitive at the top of the market.",
        }],
        stack: &["Spark", "Scala", "Query optimization", "Distributed systems"],
    },
    Role {
        company: "Microsoft",
        title: "Software Engineer II",
        start_iso: "2024-09",
        period: "Sept 2024 — Sept 2026",
        context: Some("Azure HDInsight on AKS"),
        minor: false,
        highlights: &[Highlight {
            headline: "In-place upgrades for a big data PaaS.",
            body: "Big data clusters are notoriously hard to upgrade without downtime, and most \
                   managed platforms simply don't try. I engineered in-place upgrades for \
                   HDInsight on AKS so customers could absorb the latest open-source features \
                   and fixes with minimal disruption to running workloads — a rare capability \
                   in the market, and one that unblocked enterprise onboarding that had stalled \
                   on exactly this requirement.",
        }],
        stack: &["Kubernetes", "Azure", "Go"],
    },
    Role {
        company: "Microsoft",
        title: "Software Engineer I",
        start_iso: "2022-07",
        period: "July 2022 — Sept 2024",
        context: Some("Azure HDInsight on AKS"),
        minor: false,
        highlights: &[
            Highlight {
                headline: "A custom Kubernetes scheduler for zone-resilient Kafka.",
                body: "Kafka's durability guarantees mean very little if every broker lands in \
                       the same availability zone. I designed and implemented zone-aware broker \
                       placement by writing a custom Kubernetes scheduler, which let key \
                       enterprise customers run zone-resilient Kafka on the platform for the \
                       first time.",
            },
            Highlight {
                headline: "Core platform, held to 99.9%.",
                body: "I wrote several of the platform's foundational components — cluster \
                       provisioning and the gateway layer — and held them to a 99.9% \
                       reliability bar. Every cluster on the platform passes through this path.",
            },
            Highlight {
                headline: "Governance, end to end.",
                body: "I built end-to-end provisioning for Apache Ranger clusters, the component \
                       that governs everything else: row-level policies, topic restrictions, and \
                       access control across Trino and Kafka. Governance is rarely the exciting \
                       part, and it is routinely the thing an enterprise deal hinges on.",
            },
        ],
        stack: &["Kubernetes", "Kafka", "Trino", "Ranger", "Go", "Azure"],
    },
    Role {
        company: "Microsoft",
        title: "Software Engineer Intern",
        start_iso: "2022-01",
        period: "Jan 2022 — July 2022",
        context: None,
        minor: false,
        highlights: &[Highlight {
            headline: "Shipping the replacement while the original was on fire.",
            body: "I built a low-cost alternative to the platform's serverless scaling \
                   infrastructure as an intern project. When the legacy scaling system failed in \
                   production, I took data from test regions to leadership and argued for \
                   deploying mine instead. It shipped, ended the outage's impact on customers, \
                   and permanently replaced the old system — cutting a substantial recurring \
                   maintenance cost in every region.",
        }],
        stack: &["Serverless", "Autoscaling", "Azure"],
    },
    Role {
        company: "SmokeTrees",
        title: "Full-stack intern",
        start_iso: "2020-05",
        period: "May 2020 — July 2020",
        context: None,
        minor: true,
        highlights: &[Highlight {
            headline: "",
            body: "Two products in Flutter and FastAPI: a digital marketplace for BearLyfe, a \
                   Hong Kong wellbeing company, and a home salon-booking app with payments for \
                   fabsalons in India.",
        }],
        stack: &["Flutter", "FastAPI", "PostgreSQL"],
    },
];

pub static PROJECTS: &[Project] = &[
    Project {
        name: "hybridzip",
        url: "https://github.com/hybridzip/hybridzip",
        blurb: "A model-aware data compression platform in C++. Compression ratio is bounded by \
                how well you can predict the next symbol, so hybridzip makes the model a \
                first-class, swappable part of the format instead of something welded into the \
                codec.",
        tags: &["C++", "Compression"],
    },
    Project {
        name: "mitsuha",
        url: "https://github.com/supercmmetry/mitsuha",
        blurb: "A job execution engine built on WASI — sandboxed, portable units of compute \
                without dragging an entire container runtime along.",
        tags: &["Rust", "WASI"],
    },
    Project {
        name: "rainman",
        url: "https://github.com/supercmmetry/rainman",
        blurb: "A lightweight hierarchical memory manager for C++. Ownership arranged as a tree, \
                so releasing a parent releases everything beneath it.",
        tags: &["C++", "Memory"],
    },
    Project {
        name: "bitio",
        url: "https://github.com/supercmmetry/bitio",
        blurb: "A small, fast bit-level I/O library for C++ — the primitive you end up rewriting \
                badly every time you build a codec.",
        tags: &["C++", "I/O"],
    },
    Project {
        name: "lucy",
        url: "https://github.com/GDGVIT/Neo4j-OGM",
        blurb: "An OGM for Neo4j that makes graph models behave like ordinary objects.",
        tags: &["Java", "Neo4j"],
    },
    Project {
        name: "vish.bio",
        url: "https://github.com/supercmmetry/vish.bio",
        blurb: "This site. Rust, Axum and Askama compiled to a single static binary that serves \
                its own assets.",
        tags: &["Rust", "Axum"],
    },
];

pub static SKILLS: &[SkillGroup] = &[
    SkillGroup {
        label: "Systems & languages",
        items: &[
            "Rust", "C++", "Scala", "Go", "Java", "Python", "C#", "TypeScript", "Dart",
        ],
    },
    SkillGroup {
        label: "Distributed & data",
        items: &[
            "Spark",
            "Kafka",
            "Trino",
            "Ranger",
            "Query engines",
            "System design",
            "Big data platforms",
        ],
    },
    SkillGroup {
        label: "Infrastructure",
        items: &["Kubernetes", "Docker", "Azure", "Linux", "Networking"],
    },
    SkillGroup {
        label: "Interfaces",
        items: &["React", "Next", "Nuxt", "Flutter", "Electron"],
    },
    SkillGroup {
        label: "Also",
        items: &["OpenCL / GPGPU", "TensorFlow"],
    },
];

pub static LINKS: &[Link] = &[
    Link {
        label: "Email",
        href: "mailto:vishaals2000@gmail.com",
        handle: "vishaals2000@gmail.com",
    },
    Link {
        label: "GitHub",
        href: "https://github.com/supercmmetry",
        handle: "supercmmetry",
    },
    Link {
        label: "LinkedIn",
        href: "https://linkedin.com/in/vishaal-selvaraj",
        handle: "vishaal-selvaraj",
    },
    Link {
        label: "Telegram",
        href: "https://t.me/supercmmetry",
        handle: "supercmmetry",
    },
    Link {
        label: "Curriculum vitae",
        href: "/assets/resume.pdf",
        handle: "PDF",
    },
];

// ---------------------------------------------------------------------------------------
// Scenes
// ---------------------------------------------------------------------------------------

/// The first entry is the default: rendered into the page by Askama so the scene is a
/// complete illustration before any script runs, and the one shown while the hero is in
/// view. The rest are keyed to `data-scene` on each section.
pub static SCENES: &[Scene] = &[
    Scene {
        id: "intro",
        pose: "typing",
        caption: "~/vish.bio",
        lines: &[
            "fn main() {",
            "  let me = Engineer",
            "        ::new()",
            "        .below(api);",
            "    me.build();",
            "}",
        ],
    },
    Scene {
        id: "work",
        pose: "typing",
        caption: "plan.scala",
        lines: &[
            "// rewrite the plan",
            "case Filter(p, scan)",
            "  if pushable(p) =>",
            "  scan.copy(",
            "    pushed = p +: ps",
            "  )",
        ],
    },
    Scene {
        id: "open-source",
        pose: "reviewing",
        caption: "hybridzip",
        lines: &[
            "$ git log --graph",
            "* 4f2a1c  entropy:",
            "|         adaptive",
            "|         ctx mixing",
            "* 91be07  bitio:",
            "|         2x reads",
        ],
    },
    Scene {
        id: "toolkit",
        pose: "thinking",
        caption: "Cargo.toml",
        lines: &[
            "[dependencies]",
            "rust    = \"stable\"",
            "cpp     = \"20\"",
            "scala   = \"2.13\"",
            "go      = \"1.22\"",
            "sleep   = \"0.0.1\"",
        ],
    },
    Scene {
        id: "elsewhere",
        pose: "waving",
        caption: "compose",
        lines: &[
            "$ mail vishaal",
            "Subject: hello",
            "",
            "Saw your site. Got",
            "a hard problem for",
            "you.",
        ],
    },
];

impl Scene {
    /// The scene rendered server-side, so the panel is never empty before JS boots.
    pub fn default_scene() -> &'static Scene {
        &SCENES[0]
    }
}
