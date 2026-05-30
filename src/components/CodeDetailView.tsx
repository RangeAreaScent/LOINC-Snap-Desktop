import { useEffect, useState } from "react";
import { getCodeDetail } from "../api";
import { useAppData } from "../state";
import type { CodeDetail, SearchResult } from "../types";
import { AddToCollectionModal } from "./AddToCollectionModal";

interface Props {
  code: string | null;
}

export function CodeDetailView({ code }: Props) {
  const [detail, setDetail] = useState<CodeDetail | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [copied, setCopied] = useState<string | null>(null);
  const [addingToCollection, setAddingToCollection] = useState(false);
  const { isFavorite, toggleFavorite, notes } = useAppData();

  useEffect(() => {
    if (!code) {
      setDetail(null);
      setError(null);
      return;
    }
    let active = true;
    setLoading(true);
    setError(null);
    getCodeDetail(code)
      .then((d) => {
        if (!active) return;
        setDetail(d);
        if (!d) setError(`Code "${code}" was not found.`);
      })
      .catch((e) => {
        if (active) setError(String(e));
      })
      .finally(() => {
        if (active) setLoading(false);
      });
    return () => {
      active = false;
    };
  }, [code]);

  async function copy(label: string, text: string) {
    try {
      await navigator.clipboard.writeText(text);
      setCopied(label);
      setTimeout(() => setCopied((c) => (c === label ? null : c)), 1600);
    } catch (e) {
      console.error("copy failed:", e);
    }
  }

  if (!code) {
    return (
      <div className="detail-pane detail-pane--empty">
        <p>Select a code to see its details.</p>
      </div>
    );
  }
  if (loading) {
    return (
      <div className="detail-pane detail-pane--empty">
        <p>Loading…</p>
      </div>
    );
  }
  if (error || !detail) {
    return (
      <div className="detail-pane detail-pane--empty">
        <p>{error ?? "Not found."}</p>
      </div>
    );
  }

  const asItem: SearchResult = {
    code: detail.code,
    description: detail.description,
    isBillable: detail.isBillable,
    chapterDescription: detail.chapterDescription,
    blockDescription: detail.blockDescription,
    status: detail.status,
  };

  const note = notes[detail.code];
  const fullDetail = [
    detail.code,
    detail.description,
    detail.shortName && `Short name: ${detail.shortName}`,
    note?.text && `Note: ${note.text}`,
    `Status: ${detail.status || "—"}`,
    detail.exampleUcumUnits && `Units: ${detail.exampleUcumUnits}`,
    detail.chapterDescription && `Class: ${detail.chapterDescription}`,
    detail.component && `Component: ${detail.component}`,
    detail.property && `Property: ${detail.property}`,
    detail.timeAspect && `Time: ${detail.timeAspect}`,
    detail.blockDescription && `System: ${detail.blockDescription}`,
    detail.scaleType && `Scale: ${detail.scaleType}`,
    detail.methodType && `Method: ${detail.methodType}`,
  ]
    .filter(Boolean)
    .join("\n");

  const fav = isFavorite(detail.code);

  return (
    <div className="detail-pane">
      <div className="detail-scroll">
        <div className="detail-hero">
          <div className="detail-hero__actions">
            <button
              className={`star-btn star-btn--lg${fav ? " star-btn--on" : ""}`}
              title={fav ? "Remove from favorites" : "Add to favorites"}
              onClick={() => toggleFavorite(asItem)}
            >
              {fav ? "★" : "☆"}
            </button>
            <button
              className="icon-btn"
              title="Add to collection"
              onClick={() => setAddingToCollection(true)}
            >
              ＋
            </button>
          </div>
          <div className="detail-hero__code">{detail.code}</div>
          <div className="detail-hero__desc">{detail.description}</div>
          <div className="detail-hero__meta">
            <DetailStatusBadge status={detail.status} />
            {detail.exampleUcumUnits && (
              <span className="badge badge--info">
                Units: {detail.exampleUcumUnits}
              </span>
            )}
          </div>
        </div>

        <div className="copy-group">
          <button className="copy-btn" onClick={() => copy("code", detail.code)}>
            Copy code · {detail.code}
          </button>
          <button
            className="copy-btn"
            onClick={() =>
              copy("codeDesc", `${detail.code} ${detail.description}`)
            }
          >
            Copy code + description
          </button>
          {note?.text && (
            <button
              className="copy-btn"
              onClick={() =>
                copy("codeNote", `${detail.code}\n${note.text}`)
              }
            >
              Copy code + Note
            </button>
          )}
          <button className="copy-btn" onClick={() => copy("full", fullDetail)}>
            Copy full detail
          </button>
        </div>

        <div className="classification">
          <h3 className="classification__heading">6-axis breakdown</h3>
          <AxisRow
            label="Component"
            value={detail.component}
            hint="what is measured"
          />
          <AxisRow
            label="Property"
            value={detail.property}
            expanded={AXIS_PROPERTY[detail.property]}
            hint="how expressed"
          />
          <AxisRow
            label="Time"
            value={detail.timeAspect}
            expanded={AXIS_TIME[detail.timeAspect]}
            hint="when measured"
          />
          <AxisRow
            label="System"
            value={detail.blockDescription}
            expanded={AXIS_SYSTEM[detail.blockDescription]}
            hint="specimen / source"
          />
          <AxisRow
            label="Scale"
            value={detail.scaleType}
            expanded={AXIS_SCALE[detail.scaleType]}
            hint="quantitative / qualitative"
          />
          <AxisRow
            label="Method"
            value={detail.methodType || "—"}
            hint="measurement method"
          />
          {detail.chapterDescription && (
            <ClassRow label="Class" value={detail.chapterDescription} />
          )}
        </div>

        <NoteSection code={detail.code} />
      </div>

      <div className={`toast${copied ? " toast--show" : ""}`}>Copied</div>

      {addingToCollection && (
        <AddToCollectionModal
          item={asItem}
          onClose={() => setAddingToCollection(false)}
        />
      )}
    </div>
  );
}

function ClassRow({ label, value }: { label: string; value: string }) {
  return (
    <div className="class-row">
      <span className="class-row__label">{label}</span>
      <span className="class-row__value">{value}</span>
    </div>
  );
}

/// One row in the 6-axis breakdown. Shows the raw LOINC axis token (e.g.
/// "MCnc") plus an unwrapped form ("Mass concentration") when known. The
/// `.class-row` styling is reused — no new CSS required.
function AxisRow({
  label,
  value,
  expanded,
  hint,
}: {
  label: string;
  value: string;
  expanded?: string;
  hint: string;
}) {
  if (!value) return null;
  const showExpansion = expanded && expanded !== value;
  return (
    <div className="class-row">
      <span className="class-row__label">
        {label}
        <span className="class-row__hint"> · {hint}</span>
      </span>
      <span className="class-row__value">
        {showExpansion ? `${value} · ${expanded}` : value}
      </span>
    </div>
  );
}

function DetailStatusBadge({ status }: { status: string | undefined }) {
  switch (status) {
    case "ACTIVE":
      return <span className="badge badge--billable">Active</span>;
    case "TRIAL":
      return <span className="badge badge--billable">Trial</span>;
    case "DEPRECATED":
      return <span className="badge badge--nonbillable">Deprecated</span>;
    case "DISCOURAGED":
      return <span className="badge badge--nonbillable">Discouraged</span>;
    default:
      return null;
  }
}

// LOINC axis-value unwrappers. Same dictionary as iOS `LOINCAbbreviations`,
// kept in lockstep manually. (Not exhaustive — covers the common cases the
// detail view will hit; unrecognized tokens display verbatim.)
const AXIS_PROPERTY: Record<string, string> = {
  MCnc: "Mass concentration",
  SCnc: "Substance concentration",
  CCnc: "Catalytic concentration",
  NCnc: "Number concentration",
  ACnc: "Arbitrary concentration",
  MFr: "Mass fraction",
  SFr: "Substance fraction",
  NFr: "Number fraction",
  VFr: "Volume fraction",
  Prid: "Presence or identity",
  Type: "Type",
  Titr: "Titer",
  Ratio: "Ratio",
  Vol: "Volume",
  Time: "Time",
  Temp: "Temperature",
  Pres: "Pressure",
  Len: "Length",
  Find: "Finding",
  Imp: "Impression",
  Anat: "Anatomic site",
  Cnt: "Count",
};

const AXIS_TIME: Record<string, string> = {
  Pt: "Point in time",
  "1H": "1-hour collection",
  "2H": "2-hour collection",
  "4H": "4-hour collection",
  "6H": "6-hour collection",
  "8H": "8-hour collection",
  "12H": "12-hour collection",
  "24H": "24-hour collection",
  "1Wk": "1-week collection",
  "1M": "1-month collection",
};

const AXIS_SCALE: Record<string, string> = {
  Qn: "Quantitative",
  Ord: "Ordinal",
  Nom: "Nominal",
  Nar: "Narrative",
  Doc: "Document",
  Set: "Set",
  OrdQn: "Ordinal or quantitative",
  Multi: "Multiple",
};

const AXIS_SYSTEM: Record<string, string> = {
  Ser: "Serum",
  Plas: "Plasma",
  "Ser/Plas": "Serum or plasma",
  Bld: "Blood",
  BldA: "Arterial blood",
  BldV: "Venous blood",
  BldC: "Capillary blood",
  Ur: "Urine",
  CSF: "Cerebrospinal fluid",
  Stl: "Stool",
  Saliva: "Saliva",
  Sweat: "Sweat",
  Smn: "Semen",
  "Synv fld": "Synovial fluid",
  "Periton fld": "Peritoneal fluid",
  "Plr fld": "Pleural fluid",
  "Amnio fld": "Amniotic fluid",
  Tiss: "Tissue",
  Bone: "Bone",
  Hair: "Hair",
  Nail: "Nail",
  "^Patient": "Patient",
};

function NoteSection({ code }: { code: string }) {
  const { notes, setNote, deleteNote } = useAppData();
  const note = notes[code];
  const [editing, setEditing] = useState(false);
  const [draft, setDraft] = useState("");

  // Leave edit mode when switching to a different code.
  useEffect(() => {
    setEditing(false);
    setDraft("");
  }, [code]);

  function startEdit() {
    setDraft(note?.text ?? "");
    setEditing(true);
  }

  function save() {
    const trimmed = draft.trim();
    if (trimmed) {
      setNote(code, trimmed);
    } else if (note) {
      deleteNote(code);
    }
    setEditing(false);
  }

  return (
    <div className="note-section">
      <h3 className="classification__heading">Note</h3>
      {editing ? (
        <>
          <textarea
            className="note-input"
            value={draft}
            autoFocus
            placeholder="Add a note for this code…"
            onChange={(e) => setDraft(e.target.value)}
          />
          <div className="note-actions">
            <button className="btn" onClick={() => setEditing(false)}>
              Cancel
            </button>
            <button className="btn btn--primary" onClick={save}>
              Save
            </button>
          </div>
        </>
      ) : note ? (
        <>
          <div className="note-text">{note.text}</div>
          <div className="note-actions">
            <button className="btn" onClick={startEdit}>
              Edit
            </button>
            <button
              className="btn btn--danger"
              onClick={() => deleteNote(code)}
            >
              Delete
            </button>
          </div>
        </>
      ) : (
        <button className="note-add" onClick={startEdit}>
          ＋ Add a note
        </button>
      )}
    </div>
  );
}
