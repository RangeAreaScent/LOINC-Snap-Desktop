# LOINC Snap Desktop — Handoff

<!-- snap-series:manager-block:start -->
- **App:** LOINC Snap
- **Platform:** desktop
- **Wave:** 2
- **Stage:** 1 scaffold
- **Last updated:** 2026-05-30
- **Repo:** none yet (will be `https://github.com/RangeAreaScent/LOINC-Snap-Desktop` per playbook §4 naming)
- **Latest release:** none
- **Latest CI:** n/a (no repo yet)
- **Bundle id:** com.ryan.loincsnap
- **Dataset:** LOINC v2.82 (License v5.8), Group 1 Artifacts only. 109,325 codes (97,314 ACTIVE). `loinc.sqlite` (50.0 MB) dropped at `src-tauri/resources/loinc.sqlite`. License: ok (Regenstrief perpetual + free + commercial, attribution required per `../LOINC-Snap/SPEC.md` §9).
- **Deviations from playbook:** Folder named `LOINC-Snap_Mac_Win_app` (hyphen) instead of playbook §4 convention `LOINC Snap_Mac_Win_app` (space). User decision 2026-05-29 to keep as-is. No functional impact.
- **Active blockers:**
  - **Mac build not yet verified.** Project was scaffolded by copying ICD desktop and renaming (same option B as iOS). String substitution clean (grep verifies 0 `icdsnap`/`icd-snap`/`ICD Snap` left). Rust query layer (`loinc.rs`) rewritten for LOINC schema with the same 4-signal ranker as iOS (`SPEC.md` §7a). React frontend strings (App, settings, components) NOT yet domain-swapped — that's desktop Part 2.
  - **`LOINC Snap_Win/` subfolder is a separate full project copy** (Windows-specific build setup). Currently it carries through the same string substitutions but still ships ICD's `icd.rs` + ICD sqlite content unless treated separately. Either keep in sync manually, or restructure to share `src-tauri/` with the Mac side (deferred to a Windows session).
  - GitHub repo not yet created, Apple/Windows codesign certs still series-wide blockers.
- **Next 3 steps:**
  1. **`pnpm install && pnpm tauri dev`** (or `npm`/`cargo`) — first build verification. Rust will need a fresh `Cargo.lock`; expect a few `unused field` warnings on the compat shims in `loinc.rs` (is_billable, chapter_number, etc.) — those are intentional.
  2. **Desktop Part 2 — React UI swap.** Mirror iOS Part 2: Settings "Data" section labels, About copy with LOINC §10 + UCUM attribution, search placeholder + tips (HbA1c/CBC/BUN, 2160-0/4548-4/6690-2), 6-axis breakdown panel in detail view, status badge (ACTIVE/TRIAL/DEPRECATED), CollectionExporter headers + filename, app icon swap.
  3. **Create the `LOINC-Snap-Desktop` GitHub repo, push `main`, run CI.** Once Lemon Squeezy + Apple cert + Windows codesign cert clear, do the v1.0.0-beta.1 tag flow.
- **Report-back trigger:** first successful Mac/Windows build, first commit on `main`, repo creation, any `v*` tag, any new blocker, any SPEC change
<!-- snap-series:manager-block:end -->

---

## Body

### Stage 1 state (2026-05-30)

Desktop fork done via "option B" — full `ICD Snap_mac_win_app/` copy +
rename. Identical mechanical approach to iOS. What landed:

- All `icdsnap` / `icd-snap` / `ICD Snap` strings renamed to LOINC across
  19 source files. `Cargo.toml` package + lib renamed, `package.json`
  name renamed, `tauri.conf.json` `productName` + `identifier` renamed.
- `src-tauri/src/icd.rs` → `loinc.rs`. Rewritten end to end for the LOINC
  schema in `SPEC.md` §2: SearchResult + CodeDetail structs with the
  generic-field-name compat pattern (`code` = LOINC_NUM,
  `description` = LONG_COMMON_NAME, etc.), `is_billable` derived from
  `STATUS == 'ACTIVE'` for React consumers that still check it. The
  search query is the **same 4-signal ranker as iOS**: STATUS first,
  then CLASS-priority bucket (CHEM/HEM/MICRO at 0; DEVICES/ONTOLOGY/
  SURVEY at 5), then weighted BM25 (`10/5/3/2/2/1`), then
  `length(LONG_COMMON_NAME)` tiebreak.
- `src-tauri/src/abbreviations.rs` rewritten with the same ~120 clinical
  lab abbreviations as `LOINCAbbreviations.swift` (HbA1c, CBC, BUN, TSH,
  etc.). `expand()` was also tweaked to APPEND expansions rather than
  REPLACE — so original tokens still drive FTS matches alongside the
  expanded form, matching the iOS behavior.
- `src-tauri/resources/icd10cm_2026.sqlite` → `loinc.sqlite` (50 MB).
- All `icd10cm_2026` references in code (Rust + TS + JSON) → `loinc`.
- `LOINC Snap_Win/` subfolder carried through the same string subs but
  its own internal copies of icd.rs / sqlite are NOT yet ported (gap).

### What's NOT yet done (Desktop Part 2 — next session)

- React UI strings: `App.tsx`, `settings.tsx`, `components/*.tsx`,
  `export.ts` all still have ICD-domain copy and field assumptions
  (`isBillable` badge, "Search ICD-10-CM", CDC attribution, etc.)
- The 6-axis breakdown UI in the detail panel — Rust side returns the 6
  axes (`component`, `property`, `time_aspect`, `system`, `scale_type`,
  `method_type`), but React doesn't render them yet.
- Status badge (Rust returns `status` field; React doesn't show it yet).
- App icon (still ICD-branded `app-icon-rounded.png` / `app-icon-source.png`)
- About copy with LOINC §10 + UCUM attribution.
- `CollectionExporter` (TypeScript side) headers and filename.
- `LOINC Snap_Win/` subfolder Rust + sqlite parity with the Mac side.

### Top-level layout

```
LOINC-Snap_Mac_Win_app/
├── HANDOFF.md                 ← this file
├── README.md                  (still ICD-named, Part 2)
├── app-icon-rounded.png        (still ICD, Part 2)
├── app-icon-source.png         (still ICD, Part 2)
├── index.html
├── package.json                (name: loinc-snap-desktop)
├── public/
├── src/                        (React frontend — Part 2 swap pending)
│   ├── App.tsx
│   ├── api.ts
│   ├── components/
│   ├── export.ts
│   ├── main.tsx
│   ├── settings.tsx
│   ├── state.tsx
│   ├── styles.css
│   ├── types.ts                (generic field names; Rust supplies compat)
│   └── vite-env.d.ts
├── src-tauri/
│   ├── Cargo.toml              (package: loincsnap, lib: loincsnap_lib)
│   ├── tauri.conf.json         (com.ryan.loincsnap, "LOINC Snap")
│   ├── build.rs
│   ├── capabilities/
│   ├── icons/                  (still ICD, Part 2)
│   ├── resources/
│   │   ├── fonts/
│   │   └── loinc.sqlite        ← 50 MB LOINC v2.82 dataset
│   └── src/
│       ├── abbreviations.rs    ← LOINC clinical lab abbreviations
│       ├── lib.rs              (uses `loinc::` module)
│       ├── license.rs
│       ├── loinc.rs            ← LOINC repository + 4-signal ranker
│       ├── main.rs
│       ├── pdf.rs
│       └── store.rs
├── tsconfig.json
├── tsconfig.node.json
├── vite.config.ts
└── LOINC Snap_Win/             ← Windows-specific copy (partial parity)
```

For now, all domain detail lives in
[`../LOINC-Snap/SPEC.md`](../LOINC-Snap/SPEC.md) — the canonical
cross-platform spec, shared by the iOS and desktop sides.

For now, all domain detail lives in
[`../LOINC-Snap/SPEC.md`](../LOINC-Snap/SPEC.md) — the canonical
cross-platform spec, shared by the iOS and desktop sides.

Until the desktop fork starts, refer to:
- [`../LOINC-Snap/SPEC.md`](../LOINC-Snap/SPEC.md) — domain spec (dataset, item shape, attribution, etc.)
- `../LOINC Snap_mac_win_app/HANDOFF.md` — the desktop reference this app will mirror (Tauri 2 + React 19 + Rust)
- `../Snap Series Plan/SNAP_SERIES_GUIDE.md` §6 — desktop track in the playbook

## Note on folder naming

The folder is `LOINC-Snap_Mac_Win_app` (hyphen between LOINC and Snap)
rather than the playbook §4 convention `LOINC Snap_Mac_Win_app` (space).
This deviation was acknowledged on 2026-05-29 and accepted as-is — no
functional impact, no rename planned. Recorded in the manager-block
"Deviations from playbook" line above so the series manager doesn't
keep flagging it as drift.
