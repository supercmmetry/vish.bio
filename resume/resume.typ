// =========================================================================================
// Vishaal Selvaraj — curriculum vitae
//
// Content is the previous edition of this CV, verbatim: the same contact block, skills,
// education and experience bullets, in the same order. Only the setting is new — Computer
// Modern throughout (CMU Serif for display, CMU Typewriter Text for body and meta), the
// warm paper/ink palette out of styles/app.css, and the numbered-section masthead vish.bio
// uses. The body runs in two flowing columns, as the previous edition did.
//
// Build:
//   yarn resume
// which is:
//   typst compile --font-path node_modules/computer-modern/fonts resume/resume.typ \
//                 assets/resume.pdf
//
// The faces come from the `computer-modern` devDependency, so `yarn install` has to have
// run. Without --font-path Typst falls back to its bundled New Computer Modern: the same
// design, slightly different metrics, and no typewriter face — legible, but not the site.
// =========================================================================================

// ----------------------------------------------------------------------------- tokens ---
// Lifted verbatim from the `:root` block in styles/app.css, converted from the
// `R G B` triples Tailwind consumes into hex.

#let paper = rgb("#faf8f4")
#let ink = rgb("#16130f")
#let ink-muted = rgb("#55504a")
#let ink-faint = rgb("#8b857c")
#let rule-tint = rgb("#e4ded4")
// A 2px dot at screen scale is a 6pt glyph in print — --c-line is too faint to survive the
// reduction, so the separator between skills gets a darker cut of the same hue.
#let dot-tint = rgb("#c5bdb0")
#let accent = rgb("#1c3d5a")
#let ember = rgb("#a34518")

// Second entry in each list is the fallback for a bare `typst compile`.
#let display = ("CMU Serif", "New Computer Modern")
#let mono = ("CMU Typewriter Text", "New Computer Modern")

// ------------------------------------------------------------------------- experience ---
// The old PDF carried a hand-typed "3y+", which had gone stale by a year — there is a
// commit in this repo doing nothing but correcting it. A PDF has no clock, but it does
// have a compile time, so the figure is derived and every rebuild refreshes it.
// 2022-07-01 is the start of full-time work at Microsoft.

#let years-experience = {
  let start = datetime(year: 2022, month: 7, day: 1)
  str(int((datetime.today() - start).days() / 365.2425)) + "y+"
}

// ---------------------------------------------------------------------------- helpers ---

// The site's `.meta` treatment: typewriter, small, letterspaced, near enough always
// upper-case. Every label, period and rubric on the page goes through here.
#let meta(body, size: 6.4pt, tracking: 0.16em, fill: ink-faint) = text(
  font: mono,
  size: size,
  tracking: tracking,
  fill: fill,
  weight: 500,
  body,
)

#let heading-face(body, size: 13pt, fill: ink, style: "normal") = text(
  font: display,
  size: size,
  fill: fill,
  style: style,
  weight: 500,
  body,
)

#let hairline = line(length: 100%, stroke: 0.5pt + rule-tint)

// `.section__head` — index in ember, upper-case label, and a rule running out to the
// measure. The three-column grid is the flexbox equivalent: two `flex: none` items and one
// that takes the slack.
#let section-head(index, label) = block(width: 100%, above: 15pt, below: 8pt, grid(
  columns: (auto, auto, 1fr),
  column-gutter: 8pt,
  align: horizon,
  meta(index, size: 6.2pt, tracking: 0.12em, fill: ember),
  meta(upper(label), size: 6.2pt, tracking: 0.18em, fill: ink),
  hairline,
))

// `.entry` — one record, rule above. In two columns there is no room for the site's
// desktop rail, so this is its sub-48rem form: meta stacked over body.
//
// Records are unbreakable by default, which in a multi-column flow means a record never
// splits across the column break either. `breakable` is for the one record too tall to be
// worth the white space that would push it whole to the next column.
#let entry(company, period, title, body, breakable: false) = block(
  width: 100%,
  above: 8pt,
  below: 0pt,
  breakable: breakable,
  {
    // The rule belongs to the record, not to the gap before it — emitted separately it
    // gets left behind at the foot of a column when the record moves to the next one.
    hairline
    v(6pt)
    {
      set par(leading: 0.4em, spacing: 3pt)
      meta(upper(company), size: 6.8pt, tracking: 0.14em, fill: ink)
      parbreak()
      meta(period, size: 6.1pt, tracking: 0.06em)
    }
    v(4pt)
    block(below: 5pt, heading-face(title, size: 12.5pt))
    body
  },
)

// A `.link`: accent-coloured with the site's soft underline.
#let url(href, label) = link(href, text(fill: accent, underline(
  offset: 1.5pt,
  stroke: 0.4pt + accent.transparentize(60%),
  label,
)))

// One cell of the masthead strip under the name — `.hero__fact`, label over value. The
// handles carry the full URL as a link, which the printed line is too narrow to spell out.
#let fact(label, value, href: none) = block(inset: (top: 7pt, bottom: 7pt, right: 8pt), {
  meta(upper(label), size: 5.6pt, tracking: 0.16em)
  linebreak()
  v(2.5pt, weak: true)
  let shown = text(font: mono, size: 7pt, fill: ink, tracking: 0.01em, weight: 500, value)
  if href == none { shown } else { url(href, shown) }
})

// ------------------------------------------------------------------------------ page ----

#set document(
  title: "Vishaal Selvaraj — Curriculum vitae",
  author: "Vishaal Selvaraj",
  keywords: ("Spark", "Kubernetes", "Kafka", "Distributed systems", "Big data"),
)

#set page(
  paper: "a4",
  margin: (x: 50pt, top: 48pt, bottom: 40pt),
  fill: paper,
  header: context {
    if counter(page).get().first() > 1 {
      grid(
        columns: (1fr, auto),
        meta(upper("Vishaal Selvaraj"), size: 6.2pt, tracking: 0.16em),
        meta(upper("Curriculum vitae"), size: 6.2pt, tracking: 0.16em),
      )
      v(-4pt)
      hairline
    }
  },
  header-ascent: 18pt,
  // `.colophon`, near enough verbatim.
  footer: context {
    hairline
    v(1pt)
    grid(
      columns: (1fr, auto, 1fr),
      align: (left, center, right),
      meta("Vishaal Selvaraj — Bangalore, India", size: 6pt, tracking: 0.12em),
      meta(str(counter(page).get().first()), size: 6pt, tracking: 0.12em),
      meta("vish.bio", size: 6pt, tracking: 0.12em),
    )
  },
  footer-descent: 14pt,
)

#set text(font: mono, size: 8.2pt, fill: ink-muted, weight: 500, lang: "en")
#set par(leading: 0.56em, spacing: 7pt, justify: false)
#show link: it => it

// Bullets, as the previous edition had them.
#set list(marker: text(fill: ember, size: 0.9em)[•], indent: 0pt, body-indent: 6pt, spacing: 6pt)

// =========================================================================================
// Masthead — spans the measure, above the columns.
// =========================================================================================

#meta(upper("Bangalore, India · Experience " + years-experience), size: 6.6pt, tracking: 0.16em)

#v(11pt)

#heading-face(size: 30pt)[Vishaal Selvaraj]

#v(12pt)

#hairline
#grid(
  columns: (1fr,) * 5,
  column-gutter: 0pt,
  stroke: (x, y) => (left: if x > 0 { 0.5pt + rule-tint } else { none }),
  inset: (x, y) => (left: if x > 0 { 9pt } else { 0pt }),
  fact("Email", "vishaals2000@gmail.com", href: "mailto:vishaals2000@gmail.com"),
  fact("Phone", "+91 6290972045"),
  fact("GitHub", "supercmmetry", href: "https://github.com/supercmmetry"),
  fact("LinkedIn", "vishaal-selvaraj", href: "https://www.linkedin.com/in/vishaal-selvaraj"),
  fact("Website", "vish.bio", href: "https://vish.bio"),
)
#hairline

// =========================================================================================
// Body — two flowing columns, as the previous edition of this CV had.
// =========================================================================================

#show: rest => columns(2, gutter: 24pt, rest)

// --------------------------------------------------------------------------- 01 Skills ---

#section-head("01", "Skills")

// Three unlabelled lines, exactly as they ran before; the comma lists are set in ink
// rather than prose grey so they read as data.
#let skills(items) = block(width: 100%, above: 6pt, below: 0pt, breakable: false, {
  hairline
  v(5pt)
  text(font: mono, size: 7.4pt, fill: ink, tracking: 0.02em, weight: 500,
    items.join(text(fill: dot-tint)[ #sym.dot.c ]))
})

#skills((
  "Rust", "C++", "Java", "Python", "Golang", "Scala", "Dart", "JavaScript", "OpenCL",
  "Bash", "Tensorflow", "React", "Flutter",
))
#skills((
  "Cloud", "Azure", "Kubernetes", "Docker", "Linux", "Networking", "GPGPU", "Backend",
  "System Design", "Distributed Systems", "Frontend", "Big Data", "Algorithms",
))
#skills(("Communication", "Debugging", "Team player"))
#v(6pt)
#hairline

// ------------------------------------------------------------------------ 02 Education ---

#section-head("02", "Education")

// Split across the rail exactly as the old line read — "B. Tech Computer Science and
// Engineering / Graduated @ July 2022" — rather than hoisting the institution into the
// rail, which would only repeat the sentence below it.
#entry("B. Tech", "Graduated @ July 2022", "Computer Science and Engineering")[
  Graduated with a 9.03 GPA from Vellore Institute of Technology.
]

// ----------------------------------------------------------------------- 03 Experience ---

#section-head("03", "Experience (" + years-experience + ")")

#entry("Microsoft", "September 2026 – Current", "Senior Software Engineer")[
  - I drove the research, design and development of materialized views and incremental view
    maintenance within the Spark query engine for Microsoft Fabric, significantly enhancing
    performance in medallion architecture-based pipelines. This key feature is critical for
    Microsoft Fabric to compete effectively with its competitors, providing a more robust
    solution for high-performance data processing.
]

#entry("Microsoft", "September 2024 – September 2026", "Software Engineer - II")[
  - I engineered the support for inplace upgrades in Azure HDInsight on AKS, a unique and
    competitive feature for any big data PaaS in the market. This capability enabled
    customers to seamlessly integrate the latest open-source features and fixes into their
    big data clusters with minimal workload disruption. As a result, many high-revenue
    enterprise customers (\~2 million USD potential revenue) were unblocked during
    onboarding, allowing them to receive critical updates in a managed and efficient manner.
]

#entry(
  "Microsoft",
  "July 2022 – September 2024",
  "Software Engineer - I",
  breakable: true,
)[
  - I designed and implemented zone-aware scheduling for Kafka brokers by developing a
    custom Kubernetes scheduler. This feature was a critical requirement for many key
    enterprise customers seeking to use Kafka in HDInsight on AKS. This enabled the
    onboarding of these key customers by allowing them to create Kafka clusters with
    zone-resiliency, enhancing the platform's reliability and appeal.

  - I wrote several key platform components, including cluster provisioning and gateways,
    for HDInsight on AKS clusters. These components are crucial to the platform's
    infrastructure and are expected to meet high reliability standards. I ensured these core
    components achieved a 99.9% reliability metric, reinforcing the platform's stability and
    performance.

  - I designed and implemented end-to-end provisioning for Ranger clusters in HDInsight on
    AKS, which play a crucial role in establishing governance for other cluster types such
    as Trino and Kafka. This feature enabled customers to define row-level policies, topic
    restrictions, and more, meeting their governance needs. This helped to onboard numerous
    enterprise customers to the platform by addressing their critical governance
    requirements.
]

#entry("Microsoft", "Jan 2022 - July 2022", "Software Engineer Intern")[
  - I developed a low-cost alternative serverless scaling infrastructure during my
    internship and successfully proposed its production deployment when the legacy scaling
    system crashed. Recognizing this as a critical solution, I presented compelling data
    from test regions to the leadership team, demonstrating its effectiveness. This
    initiative prevented significant customer refund losses during the outage and replaced
    the outdated infrastructure, saving approximately 30,000 USD per region per month in
    maintenance costs.
]

#entry("SmokeTrees", "May 2020 - July 2020", "Full-stack development intern")[
  - Worked on building a digital marketplace using Flutter, FastAPI and PostgreSQL for
    BearLyfe, a Hong Kong based organization that helps users improve their wellbeing.

  - Built a salon booking app using Flutter, FastAPI, Firestore and Razorpay for fabsalons,
    an India based startup that allows users to book salon facilities from their home.
]

#v(6pt)
#hairline
