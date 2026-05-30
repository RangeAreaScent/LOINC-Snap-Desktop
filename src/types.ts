// Generic field names carry LOINC semantics — see the Rust `SearchResult`
// + `CodeDetail` structs in `src-tauri/src/loinc.rs` for the mapping
// (code = LOINC_NUM, description = LONG_COMMON_NAME, chapterDescription =
// CLASS, blockDescription = SYSTEM). The compat fields below
// (`isBillable`, `chapterNumber`, …) are supplied by Rust as derived or
// empty so this whole interface can keep its shape while the React side
// gradually migrates to LOINC-native rendering.
export interface SearchResult {
  code: string;
  description: string;
  isBillable: boolean;
  chapterDescription: string;
  blockDescription: string;
  status: string;
}

export interface CodeDetail {
  code: string;
  description: string;
  shortName: string;
  isBillable: boolean;
  chapterNumber: string;
  chapterDescription: string;
  blockCode: string;
  blockDescription: string;
  categoryCode: string;
  categoryDescription: string;
  // LOINC 6-axis breakdown.
  component: string;
  property: string;
  timeAspect: string;
  scaleType: string;
  methodType: string;
  status: string;
  exampleUcumUnits: string;
}

export interface Favorite {
  code: string;
  description: string;
  isBillable: boolean;
  chapterDescription: string;
  addedAt: number;
}

export interface CollectionItem {
  code: string;
  description: string;
  isBillable: boolean;
  chapterDescription: string;
  addedAt: number;
}

export interface Collection {
  id: string;
  name: string;
  emoji: string;
  createdAt: number;
  items: CollectionItem[];
}

export interface Note {
  text: string;
  editedAt: number;
}

/** Map of LOINC code -> note. */
export type NoteMap = Record<string, Note>;
