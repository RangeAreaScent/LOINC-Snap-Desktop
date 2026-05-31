# LOINC Snap Desktop — Handoff

<!-- snap-series:manager-block:start -->
- **App:** LOINC Snap
- **Platform:** desktop
- **Wave:** 2
- **Stage:** 2 features
- **Last updated:** 2026-05-30
- **Repo:** https://github.com/RangeAreaScent/LOINC-Snap-Desktop (public, created 2026-05-30)
- **Latest release:** [v1.0.0-beta.1](https://github.com/RangeAreaScent/LOINC-Snap-Desktop/releases/tag/v1.0.0-beta.1) (2026-05-30, prerelease) — Mac arm64 DMG + Win x64 NSIS+MSI, all unsigned
- **Latest CI:** ✅ success — [run #26697461908](https://github.com/RangeAreaScent/LOINC-Snap-Desktop/actions/runs/26697461908) on commit `c408673`
- **Bundle id:** com.ryan.loincsnap
- **Dataset:** LOINC v2.82 (License v5.8), Group 1 Artifacts only. 109,325 codes (97,314 ACTIVE). `loinc.sqlite` (50.0 MB) dropped at `src-tauri/resources/loinc.sqlite`. License: ok (Regenstrief perpetual + free + commercial, attribution required per `../LOINC-Snap/SPEC.md` §9).
- **Deviations from playbook:** Folder named `LOINC-Snap_Mac_Win_app` (hyphen) instead of playbook §4 convention `LOINC Snap_Mac_Win_app` (space). User decision 2026-05-29 to keep as-is. No functional impact.
- **Active blockers:**
  - **Beta is unsigned on both platforms** — macOS users see "unidentified developer" (right-click → Open bypass), Windows users see SmartScreen warning ("More info → Run anyway"). Apple Developer + Windows EV cert clearance unblocks v1.0.0 stable.
  - **Apple Silicon only on Mac** — Intel Mac build (`x86_64`) not produced yet. Add `--target x86_64-apple-darwin` to the local build flow when Intel coverage matters.
  - **`LOINC Snap_Win/` subfolder is git-ignored** (per `.gitignore`). Not a blocker for the current release; only matters if a Windows-specific build setup is reintroduced.
- **Next 3 steps:**
  1. **Sanity-test the v1.0.0-beta.1 build on a real Mac + Windows machine** — search ranking, 6-axis panel, status badges, PDF export header + LOINC §10 footer, CSV columns. Note any UI roughness for v1.0.0.
  2. **Apple Developer enrollment + Windows codesign cert procurement** (series-wide blocker). Once cleared: re-tag as `v1.0.0` for the signed stable release.
  3. **Optional: Intel Mac coverage** — `rustup target add x86_64-apple-darwin` + `cargo tauri build --target x86_64-apple-darwin`, then upload that DMG alongside the arm64 one.
- **Report-back trigger:** beta feedback, Apple Developer enrollment, codesign certs ready, v1.0.0 stable tag, any new blocker
<!-- snap-series:manager-block:end -->

---

## Body

> **For a new Claude Code session picking this up cold:** read the
> manager-block above first, then this body in order. The canonical
> domain spec is at [`../LOINC-Snap/SPEC.md`](../LOINC-Snap/SPEC.md)
> (shared with iOS — same dataset, same ranker design, same license
> obligations). The iOS counterpart HANDOFF is at
> [`../LOINC-Snap/HANDOFF.md`](../LOINC-Snap/HANDOFF.md) — read it if you
> need to know how a concept maps cross-platform.
>
> The desktop app is **shipped as v1.0.0-beta.1**. The next big work
> items are either (a) beta-feedback fixes or (b) Apple/Windows
> codesign cert procurement to enable a signed v1.0.0 stable.

---

## 1. Where we are right now

**Stage 2 features complete. v1.0.0-beta.1 published as prerelease on
GitHub.** Three assets attached: macOS arm64 DMG (19 MB), Windows x64
NSIS installer, Windows x64 MSI installer. All three unsigned — users
have to bypass the OS "unidentified developer" / "SmartScreen" warning
on first launch. App is functionally complete: search ranking,
6-axis breakdown panel, status badges, PDF/CSV export, About with
LOINC + UCUM license attribution.

**Outstanding for v1.0.0 stable:** Apple Developer cert + Windows EV
codesign cert (series-wide blockers). Both are about money + paperwork,
not engineering.

---

## 2. File map

```
LOINC-Snap_Mac_Win_app/                  ← repo root (iCloud-synced, IS a git repo)
├── .git/
├── .github/
│   └── workflows/
│       └── build.yml                    ← CI on `v*` tag push: Windows NSIS+MSI via tauri-action, auto-creates draft release
├── .gitignore                           ← ignores node_modules, dist, target, .DS_Store, ._*, LOINC Snap_Win/, .claude/
├── HANDOFF.md                           ← this file
├── README.md                            ← still has the ICD-template README content (Part 3 cleanup)
├── app-icon-rounded.png                 ← 1024×1024 LOINC primary icon (cream + outlined "LOINC" + black "SNAP" pill)
├── app-icon-source.png                  ← same source PNG
├── index.html                           ← Vite entry
├── package.json                         (name: loinc-snap-desktop; "tauri" script; deps: React 19, Tauri 2, fontsource fonts)
├── package-lock.json                    ← committed (CI uses `npm ci`)
├── public/
│   └── fonts/
│       ├── iAWriterQuattroS-Bold.woff2
│       └── iAWriterQuattroS-Regular.woff2
├── src/                                 ← React frontend (TypeScript)
│   ├── App.tsx                          ← layout shell, sidebar + list-pane + detail-pane
│   ├── main.tsx                         ← React 19 entry
│   ├── api.ts                           ← Tauri IPC wrappers (searchCodes, getCodeDetail, …)
│   ├── state.tsx                        ← React Context — favorites, collections, notes; persists via Tauri commands
│   ├── settings.tsx                     ← theme/font/license state hook + free vs premium theme lists
│   ├── export.ts                        ← CSV (in-process) + PDF (Tauri invoke → Rust)
│   ├── types.ts                         ← SearchResult / CodeDetail / Favorite / Collection / Note (generic fields + LOINC fields)
│   ├── styles.css                       ← all CSS (single file — Tailwind isn't used)
│   ├── vite-env.d.ts
│   └── components/
│       ├── SearchView.tsx               ← debounced search, "Search LOINC" empty state
│       ├── CodeRow.tsx                  ← LOINCRow equivalent — code, description, chapter, StatusBadge
│       ├── CodeDetailView.tsx           ← hero + 6-axis breakdown panel + classChip + UCUM units chip + note editor + AxisRow + AXIS_* unwrappers
│       ├── FavoritesView.tsx            ← list of FavoriteLOINCs
│       ├── CollectionsView.tsx          ← sidebar + per-collection code list (no status badge — see §6)
│       ├── SettingsView.tsx             ← appearance / font / data / about (LOINC + UCUM attribution)
│       ├── AddCodeModal.tsx             ← search + add to a collection
│       ├── AddToCollectionModal.tsx     ← from detail view, pick collection
│       ├── CollectionFormModal.tsx      ← new/rename collection
│       ├── PremiumPromptModal.tsx       ← upsell to premium themes (no premium icons since we ship one)
│       └── Modal.tsx                    ← generic overlay
├── src-tauri/                           ← Rust + Tauri backend
│   ├── .gitignore                       ← `target/`
│   ├── build.rs                         ← tauri-build hook (auto-generated)
│   ├── Cargo.toml                       (package: loincsnap, lib: loincsnap_lib, rusqlite-bundled, printpdf, ureq)
│   ├── Cargo.lock                       ← committed (CI parity; binary not lib)
│   ├── tauri.conf.json                  (productName: "LOINC Snap", identifier: com.ryan.loincsnap, bundle resources: fonts/ + loinc.sqlite)
│   ├── capabilities/
│   │   └── default.json                 ← Tauri capability allowlist (which commands the frontend can invoke)
│   ├── icons/                           ← 14 PNG sizes + icon.icns + icon.ico (all generated from loinc-snap.png)
│   │   ├── 32x32.png, 64x64.png, 128x128.png, 128x128@2x.png, icon.png
│   │   ├── Square{30,44,71,89,107,142,150,284,310}x*Logo.png, StoreLogo.png  ← Windows Store
│   │   ├── icon.icns                    ← built via Pillow (sips kept erroring)
│   │   ├── icon.ico                     ← built via Pillow
│   │   ├── android/                     ← unused on desktop (Tauri scaffolding)
│   │   └── ios/                         ← unused on desktop
│   ├── resources/
│   │   ├── loinc.sqlite                 ← 50 MB bundled LOINC v2.82 (same file as iOS)
│   │   └── fonts/
│   │       ├── NanumGothic-Regular.ttf  ← Korean glyph coverage for PDF export
│   │       ├── NanumGothic-Bold.ttf
│   │       └── OFL.txt                  ← SIL Open Font License
│   ├── gen/                             ← auto-generated, gitignored
│   ├── target/                          ← Cargo build output, gitignored (Cargo.lock IS committed)
│   └── src/
│       ├── main.rs                      ← tiny: just `loincsnap_lib::run()`
│       ├── lib.rs                       ← registers Tauri commands, sets up app state (db_path), opens main window
│       ├── loinc.rs                     ← LOINC repository — open() / search() / fetch_detail() / 4-signal ranker
│       ├── abbreviations.rs             ← ~120 lab abbreviations; `expand()` APPENDS expansions (mirrors iOS)
│       ├── store.rs                     ← favorites/collections/notes JSON persistence (app-config dir, atomic write)
│       ├── pdf.rs                       ← native PDF export (printpdf 0.8 + NanumGothic, US Letter, header + LOINC §10 footer)
│       └── license.rs                   ← Lemon Squeezy activate/deactivate via ureq, license key persisted in store
├── tsconfig.json
├── tsconfig.node.json
└── vite.config.ts                       (port 1420, strictPort, clearScreen: false)

(NOT in the repo, gitignored:)
├── node_modules/                        ← `npm install` puts deps here
├── dist/                                ← Vite build output
├── src-tauri/target/                    ← Rust build output (several GB on a clean build)
└── LOINC Snap_Win/                      ← Windows-specific copy of the project, carried through string subs during the fork but never tracked. If you need Windows-specific build customization later, either restructure to share `src-tauri/` or re-introduce this folder as tracked with its own port of loinc.rs.
```

---

## 3. Architecture (high level)

**Tauri 2 + React 19 + Rust.** The frontend is a Vite-bundled React app
served inside Tauri's webview (WKWebView on macOS, WebView2 on Windows).
The backend is a Rust binary that owns:

- The bundled `loinc.sqlite` (read-only, via `rusqlite` with the
  `bundled` feature so we don't depend on a system SQLite)
- User data persistence (favorites/collections/notes) as JSON in the
  app config dir
- PDF generation via `printpdf` (we can't rely on the webview's
  `window.print()` — macOS WKWebView doesn't implement it)
- Optional license validation via `ureq` against Lemon Squeezy

Frontend ↔ backend talk via **Tauri IPC** — the React side calls
`@tauri-apps/api/core.invoke("command_name", { args })` which dispatches
to a `#[tauri::command]` function registered in `lib.rs`.

**Data flow:**

```
user types in SearchView input
   → 200ms debounce
   → searchCodes(query)  in src/api.ts
       → invoke("search_codes", { query, limit: 50 })  Tauri IPC boundary
           → Rust loinc::search(db_path, query, 50)
               → expand abbreviations (abbreviations::expand)
               → make FTS query
               → Stage 1: LIKE LOINC_NUM
               → Stage 2: codes_fts MATCH + 4-signal ORDER BY
               → return Vec<SearchResult> as JSON
       → JSON deserialized into SearchResult[]
   → setResults(results) → React re-renders
```

---

## 4. The 4-signal SQL ranker (most important code)

Lives in `src-tauri/src/loinc.rs`. **Identical to the iOS version** in
`LOINC-Snap/LOINCSnap/Data/Repository/LOINCRepository.swift`.
**Keep the two in lockstep.** Search behavior on Mac/Win must match
iOS exactly.

### The query

```sql
ORDER BY
  CASE c.STATUS WHEN 'ACTIVE' THEN 0 ELSE 1 END,    -- 1. ACTIVE first
  <CLASS_PRIORITY>,                                  -- 2. CLASS bucket
  bm25(codes_fts, 10.0, 5.0, 3.0, 2.0, 2.0, 1.0),    -- 3. weighted BM25 (FTS stage only)
  length(c.LONG_COMMON_NAME)                         -- 4. shorter = more general (tiebreak)
LIMIT ?
```

`CLASS_PRIORITY` is the SQL fragment in
`src-tauri/src/loinc.rs::CLASS_PRIORITY`:

```sql
CASE
    WHEN c.CLASS IN ('CHEM','HEM/BC','MICRO','SERO','DRUG/TOX','COAG','UA','BLDBK') THEN 0
    WHEN c.CLASS IN ('ALLERGY','CYTO','HISTO','PATH','CARDIO') THEN 1
    WHEN c.CLASS LIKE 'PANEL.%' THEN 4
    WHEN c.CLASS LIKE '%DEVICE%' THEN 5
    WHEN c.CLASS LIKE '%ONTOLOGY%' THEN 5
    WHEN c.CLASS LIKE 'SURVEY.%' THEN 5
    WHEN c.CLASS = 'PHENX' OR c.CLASS = 'NHANES.SURVEY' THEN 5
    WHEN c.CLASS = 'CHAL' THEN 4
    ELSE 2
END
```

### Why each signal exists

1. **STATUS sort**. Deprecated/discouraged codes still appear in old
   EHR data so we keep them in the DB, but ACTIVE codes should always
   surface first when relevance ties.
2. **CLASS priority bucket**. Without it, BM25 surfaces meta-records
   ("Type of HbA1c measurement device" — class DEVICES) above the
   actual lab values (class CHEM/HEM/BC). The bucket damps those.
3. **Weighted BM25**. Column order is LOINC_NUM(10) /
   LONG_COMMON_NAME(5) / SHORTNAME(3) / COMPONENT(2) / SYSTEM(2) /
   RELATEDNAMES2(1). RELATEDNAMES2 is the noisy synonym dump.
4. **`length(LONG_COMMON_NAME)` tiebreak**. Shorter names tend to be
   the more general, more commonly ordered test.

### Stage 1 vs Stage 2

- **Stage 1**: `LOINC_NUM LIKE '<input>%'` — catches users typing the
  code directly. `"2160"` → `"2160-0"`, `"2160-1"`, …
- **Stage 2**: FTS pass over the abbreviation-expanded query.

Results from both stages are merged via a `HashSet<String>` so we don't
return the same code twice if it matches both.

### Validating the ranker

The Python equivalent of these queries is inline in
`../LOINC-Snap/loinc-data/build_loinc_db.py` — run it and the sample
queries at the end print the top-N results. Use this to tune CLASS
priorities or BM25 weights without rebuilding the desktop app.

---

## 5. Frontend — the type compat-shim pattern

`src/types.ts` defines the IPC payload shapes. The Rust side serializes
its structs via `serde(rename_all = "camelCase")` so Rust `is_billable`
arrives as TypeScript `isBillable`.

**Field naming is deliberately generic** (`code`, `description`,
`chapterDescription`, `blockDescription`) so the React code forked from
the ICD template keeps compiling without 30+ file rewrites. The Rust
side computes the compat-shim fields:

```rust
// SearchResult — populated by map_search_row() in loinc.rs:
//   code                = LOINC_NUM
//   description         = LONG_COMMON_NAME
//   chapterDescription  = CLASS (e.g. "CHEM")
//   blockDescription    = SYSTEM (e.g. "Ser/Plas")
//   status              = ACTIVE / TRIAL / DEPRECATED / DISCOURAGED
//   isBillable          = (status == "ACTIVE")   ← compat shim

// CodeDetail adds:
//   shortName           = SHORTNAME
//   component, property, timeAspect, scaleType, methodType
//                        = the 6 LOINC axes (system is reused as blockDescription)
//   exampleUcumUnits    = EXAMPLE_UCUM_UNITS
//   isBillable          = (status == "ACTIVE")
//   chapterNumber, blockCode, categoryCode, categoryDescription = ""  ← compat shims
```

**Why `status?: string` not `status: string`** (subtle, learned the hard
way during the v1.0.0-beta.1 build cycle):

- React's `FavoritesView` reconstructs a `SearchResult` from a local
  `Favorite` snapshot (which doesn't carry STATUS — we don't snapshot
  it). With `status: string` required, TypeScript fails compile.
- Made `status?: string` optional. Both `StatusBadge` variants accept
  `string | undefined` and no-op on undefined.
- If you add a new compat shim or rename a field, watch out for this
  same pattern.

### Where the 6-axis breakdown lives

`src/components/CodeDetailView.tsx`. The classification section (line
~158) renders 6 `AxisRow` calls. The `AXIS_PROPERTY`/`AXIS_TIME`/
`AXIS_SCALE`/`AXIS_SYSTEM` dictionaries at the bottom of that file
unwrap cryptic LOINC axis tokens (`MCnc` → `Mass concentration`,
`Pt` → `Point in time`, `Ser/Plas` → `Serum or plasma`).

**Keep these dictionaries in sync with iOS's
`LOINCAbbreviations.swift`'s `axis*` dicts.** They're manually
synced — change one, update the other.

### Status badge

Two variants for the same logic:

- `StatusBadge` in `src/components/CodeRow.tsx` — used on search rows
- `DetailStatusBadge` in `src/components/CodeDetailView.tsx` — used in
  the detail hero

Both reuse the existing CSS classes from the ICD template:
- `.badge--billable` (green) → ACTIVE / TRIAL
- `.badge--nonbillable` (orange) → DEPRECATED / DISCOURAGED

So no CSS changes were needed when status badges replaced billable
badges. If a future redesign wants distinct ACTIVE vs TRIAL colors,
add new badge classes in `styles.css`.

### Where status is NOT shown

`CollectionsView.tsx` — collection-row rendering doesn't show the
badge. The `CollectionItem` snapshot stored in Rust JSON doesn't carry
STATUS, and looking it up per-row on a long list would be wasteful.
Detail view shows it. If always-on per-row status display is wanted,
extend the snapshot schema in `src-tauri/src/store.rs`.

---

## 6. Build & release flow

### Local Mac build (the one that produces the DMG)

```bash
cd "LOINC-Snap_Mac_Win_app"
npm install                 # ~10s if cached, fresh node_modules ~30s
npm run tauri build         # ~4 min cold Cargo cache on Apple Silicon
# DMG at: src-tauri/target/release/bundle/dmg/LOINC Snap_1.0.0_aarch64.dmg
```

A successful build also produces `LOINC Snap.app` at
`src-tauri/target/release/bundle/macos/`. The DMG just wraps the .app
in a drag-to-Applications installer.

**Warnings you can ignore:**

- `warning: field 'category' is never read` on `ExportEntry` in
  `pdf.rs`. Intentional — we keep the field to match the React
  `ExportEntry` struct (so JSON deserialization works); it's empty for
  LOINC because LOINC has no category concept.

### CI Windows build (the one that produces the .exe + .msi)

Triggered by pushing a `v*` tag. `.github/workflows/build.yml` does:

1. Checkout
2. Setup node 20 + Rust stable
3. Cache Cargo workspace
4. `npm ci` (this is why `package-lock.json` MUST be committed)
5. `tauri-action@v0` — runs `tauri build`, creates a **draft release**
   named "LOINC Snap v1.0.0-beta.1" or similar based on the tag,
   attaches the Windows artifacts to the draft

`tauri-action` only creates a draft if `releaseDraft: true` (which we
have). The draft becomes a real release when you flip it via the GH
UI or `gh release edit <tag> --draft=false`.

### Assembling a multi-platform release (the dance)

Because Windows builds in CI and Mac builds locally, both have to
target the same draft release:

```bash
# 1. Local first — get a clean Mac build
npm run tauri build                    # produces the DMG

# 2. Now push the tag — CI creates the draft + attaches Win artifacts
git tag v1.0.0-beta.1 -m "..."
git push origin v1.0.0-beta.1

# 3. Wait for CI (~3-5 min) — watch with:
gh run watch
#  or:  until gh run view <run-id> --json status -q .status | grep -q completed; do sleep 30; done

# 4. Once draft exists (CI succeeded), upload the local Mac DMG:
gh release upload v1.0.0-beta.1 \
  "src-tauri/target/release/bundle/dmg/LOINC Snap_1.0.0_aarch64.dmg" \
  --repo RangeAreaScent/LOINC-Snap-Desktop

# 5. Flip the draft to a published prerelease:
gh release edit v1.0.0-beta.1 \
  --draft=false --prerelease \
  --notes "<markdown release notes>"
```

### The fix-and-retag dance (avoid by checking first)

We hit this twice on the v1.0.0-beta.1 cycle. To skip the pain on
future releases:

**Pre-flight checklist before pushing any `v*` tag:**

1. `npm run tauri build` locally → must succeed (catches TS errors,
   missing fields, etc. that CI would also hit)
2. `git status` → must be clean
3. `git ls-files | grep -E "package-lock|Cargo.lock"` → both must be
   tracked. If not, `git add` + commit them first.
4. THEN tag + push.

If CI fails AFTER a tag push:
```bash
git tag -d v1.0.0-beta.1                          # delete local tag
git push origin :refs/tags/v1.0.0-beta.1          # delete remote tag
# ... fix the bug, commit ...
git tag v1.0.0-beta.1 -m "..."                    # re-create on fixed commit
git push origin main v1.0.0-beta.1                # push commit + tag together
# CI auto-re-triggers. Old draft release (with Win artifacts attached to
# the broken tag) gets deleted by GitHub when its tag is deleted.
```

---

## 7. Codesigning (the cert blocker)

Currently both Mac and Windows builds ship **unsigned**:

- **macOS**: `LOINC Snap.app` has no `_CodeSignature` from a Developer
  ID. Users see "LOINC Snap can't be opened because it is from an
  unidentified developer." Workaround: right-click → Open → Open. This
  works once and macOS remembers.
- **Windows**: NSIS installer is unsigned. SmartScreen shows the
  "Windows protected your PC" blue dialog. Workaround: "More info →
  Run anyway".

To remove the warnings:

- **macOS**: enroll in [Apple Developer Program](https://developer.apple.com/programs/)
  ($99/year). Get a Developer ID Application certificate + Developer ID
  Installer certificate. Update local build to sign + notarize:
  - Tauri config can sign via `tauri.conf.json` `bundle.macOS.signingIdentity`
  - Notarization via `xcrun notarytool submit`
- **Windows**: get an EV codesign certificate (one-time ~$200-500 from
  Sectigo / DigiCert). Add `WINDOWS_CERTIFICATE` (base64 .pfx) +
  `WINDOWS_CERTIFICATE_PASSWORD` GitHub secrets. Update `build.yml`:
  ```yaml
  env:
    TAURI_SIGNING_PRIVATE_KEY: ${{ secrets.WINDOWS_CERTIFICATE }}
    TAURI_SIGNING_PRIVATE_KEY_PASSWORD: ${{ secrets.WINDOWS_CERTIFICATE_PASSWORD }}
  ```
  (Exact env names depend on tauri-action version — verify against docs.)

Both certs are series-wide blockers per SNAP_SERIES_STATUS — they're
shared across all Snap desktop apps once procured.

---

## 8. Known issues / decisions worth knowing

### Apple Silicon only
The DMG is `aarch64` only. Intel Macs (`x86_64`) need a separate build:

```bash
rustup target add x86_64-apple-darwin
npm run tauri build -- --target x86_64-apple-darwin
# DMG at: src-tauri/target/x86_64-apple-darwin/release/bundle/dmg/
gh release upload v1.0.0-beta.1 <dmg>
```

Or for a universal binary: `--target universal-apple-darwin`. Skipped
for v1.0.0-beta.1 because the dev Mac is Apple Silicon and most modern
Mac users are too. Reconsider if anyone reports.

### `LOINC Snap_Win/` subfolder
A leftover from the ICD template — a full duplicate project copy used
for Windows-specific build customization (probably codesign config). We
got it during the rsync fork, applied the string substitutions, then
**git-ignored it** (`.gitignore` line `LOINC Snap_Win/`). Not blocking
the current release. If a future Windows-specific workflow needs it
back, either restructure to share `src-tauri/` or re-introduce the
folder as tracked with its own port of `icd.rs` → `loinc.rs` and its
own sqlite swap.

### `dead_code` warning on `pdf.rs::ExportEntry::category`
Intentional. The field exists to match the React `ExportEntry` JSON
shape (the Rust struct deserializes the React payload). It's always
empty for LOINC. The warning is harmless.

### `sips` couldn't write `.icns` files
During the icon swap, `sips -s format icns input.png --out output.icns`
printed "Try 'sips --help'" on this macOS version. Pillow worked:

```python
from PIL import Image
Image.open(src).convert("RGBA").save("icon.icns", format="ICNS",
    sizes=[(16,16),(32,32),(64,64),(128,128),(256,256),(512,512),(1024,1024)])
```

Pillow also handled `.ico` (Windows). Both work without ImageMagick or
external tools.

### iOS / desktop axis dict drift
`AXIS_PROPERTY` / `AXIS_TIME` / `AXIS_SCALE` / `AXIS_SYSTEM` dictionaries
exist in **two places** — once in `src/components/CodeDetailView.tsx`
(TypeScript) and once in `LOINC-Snap/LOINCSnap/Data/LOINCAbbreviations.swift`
(Swift). They are **manually kept in sync**. There's no codegen. If you
add an entry, add it to both. Same applies to the lab abbreviations in
`src-tauri/src/abbreviations.rs` ↔ `LOINCAbbreviations.swift::lab`.

### React `CollectionItem` doesn't carry STATUS
By design — collection rows don't render a status badge. If you want
always-on per-row status:
1. Extend the snapshot in `src-tauri/src/store.rs::CollectionItem` to
   include `status: String`
2. Migrate existing on-disk JSON (read old shape, default status to
   `"ACTIVE"`, write new shape)
3. Update `src/types.ts::CollectionItem` to require `status: string`
4. Render `StatusBadge` in `CollectionsView.tsx`

---

## 9. Pitfalls when working on this app

### iCloud Drive path has spaces — quote shell paths
Repo root is at `…/Mobile Documents/com~apple~CloudDocs/App Projects/LOINC-Snap_Mac_Win_app/`.
Always wrap paths in double quotes. Periodically `find . -name "._*" -delete`
and `find . -name ".DS_Store" -delete` — macOS sprinkles AppleDouble
metadata into iCloud-synced folders, and these aren't always covered by
the `.gitignore` rules.

### `npm ci` (CI) vs `npm install` (local)
- Local devs use `npm install` — installs from package.json, updates
  `package-lock.json` if dependency versions resolve differently.
- CI uses `npm ci` — installs from `package-lock.json` exactly, fails
  if no lock or if lock is out of date.
- **`package-lock.json` MUST be committed.** If you forgot (as we did
  the first time), CI fails with `EUSAGE`.

### Rust `Cargo.lock` IS committed (binary, not lib)
Rust's convention: libraries (publish to crates.io) don't commit
`Cargo.lock`; binaries (applications) do. We're a binary. The lockfile
guarantees CI Windows and local Mac build the same dependency tree —
DMG and MSI ship identical Rust code.

### Don't bump versions in `package.json` / `Cargo.toml` / `tauri.conf.json` separately
All three must match. The version in `tauri.conf.json` is what shows
up in the .app's Info.plist and in the installer name. The version in
`Cargo.toml` is what `cargo` cares about. Mismatch leads to
confusing artifact filenames.

### Modal sheet "white screen" pattern
The iOS side hit this with `.sheet(isPresented:) + optional` rendering
empty when the optional hadn't been set yet. The web equivalent here
would be a `Modal` that conditionally renders its content from a `null`
state. Both `AddCodeModal`, `AddToCollectionModal`, `CollectionFormModal`
take their data via props (not via a state-soup race), so we don't have
this problem on desktop. If you add a new modal that reads from
component state, prefer to thread the data through props or guarantee
non-null before mounting.

### Cargo target/ is enormous
A clean release build produces 5-10 GB in `src-tauri/target/`. It's
gitignored but lives in iCloud Drive which syncs it. Consider adding
`target/` to your iCloud exclusion list (Apple menu → System Settings
→ Apple ID → iCloud → iCloud Drive Options → exclude folder) if your
storage is tight.

---

## 10. License obligations (LOINC §10 + UCUM)

Every shipping build MUST include both attribution strings somewhere
user-reachable. Currently they live in:

- **In-app**: `src/components/SettingsView.tsx` — About section
  (3 paragraphs: disclaimer + LOINC §10 + UCUM)
- **PDF export**: `src-tauri/src/pdf.rs::export` — footer of every
  exported PDF carries LOINC §10 attribution (~7.5pt text)
- **Privacy page**: <https://rangeareascent.github.io/Snap_Series/loincsnap/privacy/>
  — both blocks repeated

**Exact text** (copy verbatim if porting elsewhere):

```
This material contains content from LOINC (http://loinc.org).
LOINC is copyright © Regenstrief Institute, Inc. and the Logical
Observation Identifiers Names and Codes (LOINC) Committee and is
available at no cost under the license at http://loinc.org/license.
LOINC® is a registered United States trademark of Regenstrief Institute, Inc.
```

```
This product includes UCUM codes and definitions, copyright © 1995–2026
Regenstrief Institute, Inc. and the Unified Codes for Units of Measures
Organization. Available at http://unitsofmeasure.org. Provided "AS IS"
with no warranties.
```

---

## 11. Recent activity (concise log)

| Date | What | Commit |
|---|---|---|
| 2026-05-29 | Folder created (placeholder), HANDOFF with manager-block | (no git yet) |
| 2026-05-30 | Stage 1 scaffold — option B (full rsync from ICD desktop), strings + Rust query + sqlite swapped | (no git yet) |
| 2026-05-30 | Stage 2 Part 2 — React UI swap, 6-axis breakdown, status badge, app icon (29 files) | `13f5ff0` |
| 2026-05-30 | HANDOFF Stage 1 → 2 documented | `e51cad8` |
| 2026-05-30 | Repo created on GitHub, initial push (103 files) | `a18e6b8` (was — see retag note) |
| 2026-05-30 | Tag v1.0.0-beta.1 pushed → CI Win build started | (tag) |
| 2026-05-30 | TS error: SearchResult.status required but FavoritesView constructs without it. Fix: `status?: string` | `4a31640` |
| 2026-05-30 | CI failure: `npm ci` had no lockfile (was rsync-excluded). Fix: commit `package-lock.json` + `Cargo.lock` | `c408673` |
| 2026-05-30 | Tag moved to `c408673`. CI succeeded. Mac DMG built locally (~4 min). Uploaded to draft. Draft → published prerelease. | `d33c4c9` (HANDOFF only) |

The current `main` is at `d33c4c9` (or wherever subsequent commits
have landed since). `git log --oneline -10` for the latest.

---

## 12. What's next (concrete options)

**Can do now (no blocker):**
- **Sanity-test the v1.0.0-beta.1 build** on a real Mac + Windows
  machine. Search ranking, 6-axis panel, status badges, PDF export
  header + LOINC §10 footer, CSV columns. Note any UI roughness.
- **Re-port the `LOINC Snap_Win/` folder** if Windows-specific build
  customization becomes important (currently gitignored, ICD content
  inside)
- **Intel Mac DMG** for broader Mac coverage (see §8)
- **Universal binary** (`--target universal-apple-darwin`) instead of
  separate Intel + arm64 — slightly larger DMG but one download
- **Drop the React `categoryDescription` / `categoryCode` compat
  shims** by updating `CodeDetailView.tsx` to stop referencing them
  (currently always-empty strings from Rust). Cleanup pass.
- **Tune the search ranker** if beta feedback surfaces specific queries
  that return bad results. Levers: `CLASS_PRIORITY` buckets in
  `loinc.rs`, BM25 weights, additional ORDER BY signals.
- **Lemon Squeezy product creation** + license-flow end-to-end test.
  The Rust code in `license.rs` is wired but no LS product exists yet.

**Blocked on certs (Stage 3):**
- **Apple Developer enrollment** → sign + notarize Mac DMG
- **Windows EV codesign cert** → sign Windows artifacts via CI
- Once both signed: re-tag as `v1.0.0` (drop `-beta.1`) for the
  stable release

---

## 13. Folder naming note

The folder is `LOINC-Snap_Mac_Win_app` (hyphen between LOINC and Snap)
rather than the playbook §4 convention `LOINC Snap_Mac_Win_app` (space).
This deviation was acknowledged on 2026-05-29 and accepted as-is — no
functional impact, no rename planned. Recorded in the manager-block
"Deviations from playbook" line above so the series manager doesn't
keep flagging it as drift.

---

## 14. External references

- [`../LOINC-Snap/SPEC.md`](../LOINC-Snap/SPEC.md) — canonical
  cross-platform domain spec. License analysis is in §1; the 4-signal
  ranker rationale is in §7a.
- [`../LOINC-Snap/HANDOFF.md`](../LOINC-Snap/HANDOFF.md) — iOS
  counterpart. Same dataset, same ranker, same compat-shim pattern.
- [`../LOINC-Snap/PRIVACY.md`](../LOINC-Snap/PRIVACY.md) — markdown
  working draft for the privacy policy (HTML deployed copy is at
  Snap_Series/loincsnap/privacy/index.html)
- `../ICD Snap_mac_win_app/HANDOFF.md` — the original ICD desktop
  HANDOFF this app was forked from (still the most detailed reference
  for Tauri + React + Rust pattern + signing workflow once certs land)
- `../Snap Series Plan/SNAP_SERIES_GUIDE.md` §6 — desktop track in the
  series playbook
- `../Snap Series Plan/SNAP_SERIES_STATUS.md` — series dashboard
- Repo: https://github.com/RangeAreaScent/LOINC-Snap-Desktop
- Release: https://github.com/RangeAreaScent/LOINC-Snap-Desktop/releases/tag/v1.0.0-beta.1
- CI runs: https://github.com/RangeAreaScent/LOINC-Snap-Desktop/actions
- LOINC license: <http://loinc.org/license>
- App icon source: `../SNAP Series Plan/Icons/loinc-snap/loinc-snap.png`
- Tauri docs: <https://tauri.app/v2/>
- printpdf docs: <https://docs.rs/printpdf/>
- rusqlite docs: <https://docs.rs/rusqlite/>
- GRDB (iOS side, for cross-reference): <https://github.com/groue/GRDB.swift>
