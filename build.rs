use std::{fs, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=assets");
    println!("cargo:rerun-if-changed=templates");

    if !Path::new("assets/app.generated.css").exists() {
        panic!(
            "\n\n  assets/app.generated.css is missing.\n\n  \
             Run `yarn build` before `cargo build` — the stylesheet is produced by PostCSS\n  \
             and baked into the binary by rust-embed at compile time, so building without\n  \
             it yields a binary that 404s its own styles.\n"
        );
    }

    // rust-embed's release codegen emits one include_bytes! per file. That tracks files
    // which already exist, but is *not* invalidated when a new file appears — so a newly
    // added font or script can be silently missing from a release binary. Feeding a
    // fingerprint of the asset tree into the build forces re-expansion when the set changes.
    let mut entries = Vec::new();
    collect(Path::new("assets"), &mut entries);
    entries.sort();
    println!(
        "cargo:rustc-env=ASSETS_FINGERPRINT={:016x}",
        fnv1a(&entries.join("\n"))
    );
}

fn collect(dir: &Path, out: &mut Vec<String>) {
    let Ok(read) = fs::read_dir(dir) else { return };
    for entry in read.flatten() {
        let path = entry.path();
        println!("cargo:rerun-if-changed={}", path.display());
        if path.is_dir() {
            collect(&path, out);
        } else {
            let len = entry.metadata().map(|m| m.len()).unwrap_or(0);
            out.push(format!("{}:{len}", path.display()));
        }
    }
}

/// Enough to notice an asset being added, removed, or resized. Content edits that preserve
/// size are caught by the per-file `rerun-if-changed` above. No dependency required.
fn fnv1a(s: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}
