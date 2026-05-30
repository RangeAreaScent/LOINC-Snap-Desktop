import type { SearchResult } from "../types";

interface Props {
  item: SearchResult;
  selected: boolean;
  favorite: boolean;
  onSelect: () => void;
  onToggleFavorite: () => void;
}

export function CodeRow({
  item,
  selected,
  favorite,
  onSelect,
  onToggleFavorite,
}: Props) {
  return (
    <div
      className={`code-row${selected ? " code-row--selected" : ""}`}
      onClick={onSelect}
      role="button"
      tabIndex={0}
      onKeyDown={(e) => {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          onSelect();
        }
      }}
    >
      <div className="code-row__main">
        <div className="code-row__top">
          <span className="code-row__code">{item.code}</span>
          <StatusBadge status={item.status} />
        </div>
        <div className="code-row__desc">{item.description}</div>
        {item.chapterDescription && (
          <div className="code-row__chapter">{item.chapterDescription}</div>
        )}
      </div>
      <button
        className={`star-btn${favorite ? " star-btn--on" : ""}`}
        title={favorite ? "Remove from favorites" : "Add to favorites"}
        onClick={(e) => {
          e.stopPropagation();
          onToggleFavorite();
        }}
      >
        {favorite ? "★" : "☆"}
      </button>
    </div>
  );
}

/// Reuses the existing `badge` classes from the ICD template:
///   .badge--billable    → green   (we map to ACTIVE)
///   .badge--nonbillable → orange  (we map to DEPRECATED / DISCOURAGED)
/// TRIAL gets the green pill too for now — no CSS class change required.
/// Falls back to nothing when the status is missing (defensive).
function StatusBadge({ status }: { status: string | undefined }) {
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
