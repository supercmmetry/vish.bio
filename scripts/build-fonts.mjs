/*
 * Subsets the Computer Modern faces into assets/fonts/.
 *
 * The upstream faces carry Greek, Cyrillic and full TeX math coverage — 1868 glyphs for
 * CMU Typewriter Text, 2278 for CMU Serif — which lands at ~873 KB across the five faces
 * this site uses. All of it would be embedded into the binary by rust-embed and shipped
 * to every visitor. Subsetting to the characters the site can actually render cuts that
 * by roughly an order of magnitude.
 *
 * Output is deterministic for a given input, so the results are committed: the Docker
 * image is FROM scratch and has no filesystem to fall back on, and assets/fonts would
 * otherwise be missing from a fresh clone.
 */
import { readFile, writeFile, mkdir, readdir, unlink } from "node:fs/promises";
import { existsSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import subsetFont from "subset-font";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const src = path.join(root, "node_modules", "computer-modern", "fonts");
const out = path.join(root, "assets", "fonts");

/*
 * The site's own text is ASCII apart from `· — ↓ ↗`, so Latin-1's full 96-glyph block is
 * dead weight — it costs ~14 KB per face for characters nothing renders. What is kept
 * instead is a curated set of Western European accented letters, ~6 KB per face cheaper,
 * so a name or place with a diacritic never falls out of the family mid-word.
 *
 * The rest is enumerated rather than pulled in by block, so adding a glyph to the site is
 * a deliberate edit here: typographic punctuation, the arrows used by the hero actions and
 * project links, the middot separating eyebrow fields, and the box-drawing characters and
 * caret the coder scene types.
 */
const ASCII = Array.from({ length: 0x7f - 0x20 }, (_, i) => String.fromCodePoint(0x20 + i));
const ACCENTED = "àáâãäåçèéêëìíîïñòóôõöùúûüýÿÀÁÂÃÄÅÇÈÉÊËÌÍÎÏÑÒÓÔÕÖÙÚÛÜÝßæœøÆŒØ";
const PUNCTUATION = "‘’“”–—…·•†‡§¶";
const SYMBOLS = "→←↑↓↗↘✓✗▍│─┌┐└┘├┤";

const charset = [...ASCII, ...ACCENTED, ...PUNCTUATION, ...SYMBOLS].join("");

/*
 * The package names regular weight "500"; the site declares it as 400.
 *
 * Three faces, not five. CMU Typewriter's italic is omitted because nothing uses body
 * emphasis, and a synthesised oblique is a fair rendering of a typewriter face anyway.
 * Its bold is kept despite being unused today: faux-bold smears Computer Modern's
 * hairlines badly, so bold is the one weight with no acceptable fallback. To add a face
 * back, add the pair here and re-run.
 */
const FACES = [
  ["cmu-typewriter-text-500-roman", "cmu-typewriter-400-normal"],
  ["cmu-typewriter-text-700-roman", "cmu-typewriter-700-normal"],
  ["cmu-serif-500-roman", "cmu-serif-400-normal"],
  ["cmu-serif-500-italic", "cmu-serif-400-italic"],
];

const kb = (n) => `${(n / 1024).toFixed(1)} KB`;

if (!existsSync(src)) {
  console.error("computer-modern is not installed — run `yarn install` first.");
  process.exit(1);
}

await mkdir(out, { recursive: true });

/* Anything not produced below is a leftover from a previous font system. */
const keep = new Set(FACES.map(([, name]) => `${name}.woff2`));
for (const stale of await readdir(out)) {
  if (stale.endsWith(".woff2") && !keep.has(stale)) {
    await unlink(path.join(out, stale));
    console.log(`  removed  ${stale}`);
  }
}

let before = 0;
let after = 0;

for (const [from, to] of FACES) {
  /*
   * Subset from the .ttf, not the .woff2: harfbuzz has to inflate a woff2 before it can
   * touch the glyf table, and the package ships both.
   */
  const input = await readFile(path.join(src, `${from}.ttf`));
  const source = await readFile(path.join(src, `${from}.woff2`));
  const result = await subsetFont(input, charset, { targetFormat: "woff2" });

  await writeFile(path.join(out, `${to}.woff2`), result);

  before += source.length;
  after += result.length;
  console.log(
    `  ${to.padEnd(28)} ${kb(source.length).padStart(9)} → ${kb(result.length).padStart(9)}`,
  );
}

console.log(`\n  total${" ".repeat(25)} ${kb(before).padStart(9)} → ${kb(after).padStart(9)}`);
