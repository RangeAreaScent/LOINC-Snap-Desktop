//! Read-only access to the bundled LOINC SQLite database.
//!
//! Mirrors the iOS `LOINCRepository.swift` — same schema, same two-stage
//! search, same weighted BM25 + CLASS-priority ranker. Keep the two in
//! lockstep so search behavior is identical across platforms.

use crate::abbreviations;
use rusqlite::{Connection, OpenFlags};
use serde::Serialize;
use std::collections::HashSet;
use std::path::Path;

/// Field naming is deliberately generic (`code`, `description`, …) to
/// minimize churn on the React side. Values carry LOINC semantics:
///
/// | field                 | semantic                              |
/// |-----------------------|---------------------------------------|
/// | code                  | LOINC_NUM (e.g. "2160-0")             |
/// | description           | LONG_COMMON_NAME                      |
/// | chapter_description   | CLASS (e.g. "CHEM")                   |
/// | block_description     | SYSTEM (e.g. "Ser/Plas")              |
/// | status                | ACTIVE / TRIAL / DEPRECATED / DISCOURAGED |
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub code: String,
    pub description: String,
    pub chapter_description: String,
    pub block_description: String,
    pub status: String,
    /// Compatibility shim from the ICD template — `true` for ACTIVE codes.
    /// React consumers still check `result.isBillable`; revisit in desktop
    /// Part 2 when the UI swaps to status-aware rendering.
    pub is_billable: bool,
}

/// Full LOINC record for the detail screen. Includes the six axes that
/// define every LOINC code (Component / Property / Time / System / Scale /
/// Method) plus UCUM units when present.
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CodeDetail {
    pub code: String,
    pub description: String,
    pub short_name: String,
    pub chapter_description: String,
    pub block_description: String,
    pub component: String,
    pub property: String,
    pub time_aspect: String,
    pub scale_type: String,
    pub method_type: String,
    pub status: String,
    pub example_ucum_units: String,
    // Compatibility shims for React consumers forked from the ICD template.
    // Always derived/empty; revisit in desktop Part 2.
    pub is_billable: bool,
    pub chapter_number: String,
    pub block_code: String,
    pub category_code: String,
    pub category_description: String,
}

/// BM25 weights — same column order as `codes_fts` in build_loinc_db.py:
/// LOINC_NUM, LONG_COMMON_NAME, SHORTNAME, COMPONENT, SYSTEM, RELATEDNAMES2.
/// LOINC_NUM is highest so embedded code-number matches surface; the noisy
/// RELATEDNAMES2 synonym dump is intentionally damped to 1.0.
const BM25_WEIGHTS: &str = "10.0, 5.0, 3.0, 2.0, 2.0, 1.0";

/// CLASS-priority bucket fragment. Lower = ranked higher. Without this,
/// BM25 alone surfaces niche specimens and device-metadata codes above
/// the orderable lab values users actually want. See SPEC.md §7a.
const CLASS_PRIORITY: &str = "\
CASE \
    WHEN c.CLASS IN ('CHEM','HEM/BC','MICRO','SERO','DRUG/TOX','COAG','UA','BLDBK') THEN 0 \
    WHEN c.CLASS IN ('ALLERGY','CYTO','HISTO','PATH','CARDIO') THEN 1 \
    WHEN c.CLASS LIKE 'PANEL.%' THEN 4 \
    WHEN c.CLASS LIKE '%DEVICE%' THEN 5 \
    WHEN c.CLASS LIKE '%ONTOLOGY%' THEN 5 \
    WHEN c.CLASS LIKE 'SURVEY.%' THEN 5 \
    WHEN c.CLASS = 'PHENX' OR c.CLASS = 'NHANES.SURVEY' THEN 5 \
    WHEN c.CLASS = 'CHAL' THEN 4 \
    ELSE 2 \
END";

fn open(db_path: &Path) -> Result<Connection, String> {
    Connection::open_with_flags(
        db_path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .map_err(|e| format!("failed to open LOINC database: {e}"))
}

/// Injection-safe FTS5 MATCH expression: alphanumeric tokens of length >= 2,
/// each wrapped as a quoted prefix term, AND-joined.
fn make_fts_query(expanded: &str) -> String {
    expanded
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| t.chars().count() >= 2)
        .map(|t| format!("\"{t}\"*"))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn search(db_path: &Path, query: &str, limit: usize) -> Result<Vec<SearchResult>, String> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    let conn = open(db_path)?;
    let expanded = abbreviations::expand(trimmed);
    let fts_query = make_fts_query(&expanded);
    // LOINC codes are formatted "2160-0" — case preserved as-is.
    let num_prefix = format!("{trimmed}%");
    let num_budget = limit.min(20) as i64;
    let fts_budget = limit.max(50) as i64;

    let mut seen: HashSet<String> = HashSet::new();
    let mut results: Vec<SearchResult> = Vec::with_capacity(limit);

    // Stage 1 — LOINC_NUM prefix match. "2160" → "2160-0", "2160-1", …
    {
        let sql = format!(
            "SELECT c.LOINC_NUM, c.LONG_COMMON_NAME, c.CLASS, c.SYSTEM, c.STATUS \
             FROM codes c \
             WHERE c.LOINC_NUM LIKE ?1 \
             ORDER BY \
                 CASE c.STATUS WHEN 'ACTIVE' THEN 0 ELSE 1 END, \
                 {CLASS_PRIORITY}, \
                 length(c.LOINC_NUM), \
                 c.LOINC_NUM \
             LIMIT ?2"
        );
        let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map(rusqlite::params![num_prefix, num_budget], map_search_row)
            .map_err(|e| e.to_string())?;
        for row in rows {
            let r = row.map_err(|e| e.to_string())?;
            if seen.insert(r.code.clone()) {
                results.push(r);
            }
        }
    }

    // Stage 2 — FTS5 with weighted BM25 + multi-signal sort.
    // Order of signals (most decisive first):
    //   1. STATUS = ACTIVE (legacy/trial codes pushed below)
    //   2. CLASS priority bucket (lab essentials over devices/meta)
    //   3. BM25 (semantic relevance within the bucket)
    //   4. shorter LONG_COMMON_NAME (proxy for "more common test")
    if !fts_query.is_empty() && results.len() < limit {
        let sql = format!(
            "SELECT c.LOINC_NUM, c.LONG_COMMON_NAME, c.CLASS, c.SYSTEM, c.STATUS \
             FROM codes_fts f \
             JOIN codes c ON c.rowid = f.rowid \
             WHERE codes_fts MATCH ?1 \
             ORDER BY \
                 CASE c.STATUS WHEN 'ACTIVE' THEN 0 ELSE 1 END, \
                 {CLASS_PRIORITY}, \
                 bm25(codes_fts, {BM25_WEIGHTS}), \
                 length(c.LONG_COMMON_NAME) \
             LIMIT ?2"
        );
        let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map(rusqlite::params![fts_query, fts_budget], map_search_row)
            .map_err(|e| e.to_string())?;
        for row in rows {
            let r = row.map_err(|e| e.to_string())?;
            if seen.insert(r.code.clone()) {
                results.push(r);
                if results.len() >= limit {
                    break;
                }
            }
        }
    }

    Ok(results)
}

pub fn fetch_detail(db_path: &Path, code: &str) -> Result<Option<CodeDetail>, String> {
    let conn = open(db_path)?;
    let mut stmt = conn
        .prepare(
            "SELECT LOINC_NUM, LONG_COMMON_NAME, SHORTNAME, CLASS, SYSTEM, \
             COMPONENT, PROPERTY, TIME_ASPCT, SCALE_TYP, METHOD_TYP, \
             STATUS, EXAMPLE_UCUM_UNITS \
             FROM codes WHERE LOINC_NUM = ?1",
        )
        .map_err(|e| e.to_string())?;
    let mut rows = stmt
        .query_map(rusqlite::params![code], |row| {
            let status: String = row.get::<_, Option<String>>(10)?.unwrap_or_default();
            let is_billable = status == "ACTIVE";
            Ok(CodeDetail {
                code:                row.get(0)?,
                description:         row.get(1)?,
                short_name:          row.get::<_, Option<String>>(2)?.unwrap_or_default(),
                chapter_description: row.get::<_, Option<String>>(3)?.unwrap_or_default(),
                block_description:   row.get::<_, Option<String>>(4)?.unwrap_or_default(),
                component:           row.get::<_, Option<String>>(5)?.unwrap_or_default(),
                property:            row.get::<_, Option<String>>(6)?.unwrap_or_default(),
                time_aspect:         row.get::<_, Option<String>>(7)?.unwrap_or_default(),
                scale_type:          row.get::<_, Option<String>>(8)?.unwrap_or_default(),
                method_type:         row.get::<_, Option<String>>(9)?.unwrap_or_default(),
                status,
                example_ucum_units:  row.get::<_, Option<String>>(11)?.unwrap_or_default(),
                is_billable,
                chapter_number:       String::new(),
                block_code:           String::new(),
                category_code:        String::new(),
                category_description: String::new(),
            })
        })
        .map_err(|e| e.to_string())?;
    match rows.next() {
        Some(r) => Ok(Some(r.map_err(|e| e.to_string())?)),
        None => Ok(None),
    }
}

fn map_search_row(row: &rusqlite::Row) -> rusqlite::Result<SearchResult> {
    let status = row.get::<_, Option<String>>(4)?.unwrap_or_default();
    let is_billable = status == "ACTIVE";
    Ok(SearchResult {
        code:                row.get(0)?,
        description:         row.get(1)?,
        chapter_description: row.get::<_, Option<String>>(2)?.unwrap_or_default(),
        block_description:   row.get::<_, Option<String>>(3)?.unwrap_or_default(),
        status,
        is_billable,
    })
}
