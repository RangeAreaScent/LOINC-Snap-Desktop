//! Clinical lab abbreviation expansion. Mirrors iOS
//! `LOINCAbbreviations.swift` lab dictionary. Each expansion appears
//! verbatim in at least one LOINC LONG_COMMON_NAME or RELATEDNAMES2.
//!
//! NOTE: LOINC also has axis-value abbreviations (MCnc, Pt, Qn, …) but
//! those are display-time only and live in the React frontend's detail
//! view — not here. This file is search-time expansion only.

/// (abbreviation, expansion) pairs. Lookup keys are uppercased.
const DICTIONARY: &[(&str, &str)] = &[
    // Glycemic / metabolic panel
    ("HBA1C", "Hemoglobin A1c"),
    ("A1C", "Hemoglobin A1c"),
    ("CBC", "Complete blood count"),
    ("CMP", "Comprehensive metabolic panel"),
    ("BMP", "Basic metabolic panel"),
    ("BUN", "Urea nitrogen"),
    ("CR", "Creatinine"),
    ("EGFR", "Glomerular filtration rate"),
    ("GFR", "Glomerular filtration rate"),
    ("NA", "Sodium"),
    ("K", "Potassium"),
    ("CL", "Chloride"),
    ("CA", "Calcium"),
    ("MG", "Magnesium"),
    ("PHOS", "Phosphate"),
    ("PO4", "Phosphate"),
    ("HCO3", "Bicarbonate"),
    ("CO2", "Carbon dioxide"),
    // Thyroid
    ("T3", "Triiodothyronine"),
    ("T4", "Thyroxine"),
    ("FT3", "Triiodothyronine free"),
    ("FT4", "Thyroxine free"),
    ("TSH", "Thyrotropin"),
    ("RT3", "Reverse triiodothyronine"),
    ("TPO", "Thyroid peroxidase"),
    // Lipids
    ("LDL", "Low density lipoprotein cholesterol"),
    ("HDL", "High density lipoprotein cholesterol"),
    ("VLDL", "Very low density lipoprotein cholesterol"),
    ("TG", "Triglyceride"),
    ("CHOL", "Cholesterol"),
    ("APOA", "Apolipoprotein A"),
    ("APOB", "Apolipoprotein B"),
    // Liver / coag
    ("AST", "Aspartate aminotransferase"),
    ("ALT", "Alanine aminotransferase"),
    ("SGOT", "Aspartate aminotransferase"),
    ("SGPT", "Alanine aminotransferase"),
    ("ALP", "Alkaline phosphatase"),
    ("GGT", "Gamma glutamyl transferase"),
    ("LDH", "Lactate dehydrogenase"),
    ("TBILI", "Bilirubin total"),
    ("DBILI", "Bilirubin direct"),
    ("ALB", "Albumin"),
    ("TP", "Protein"),
    ("PT", "Prothrombin time"),
    ("INR", "INR"),
    ("PTT", "Partial thromboplastin time"),
    ("APTT", "Partial thromboplastin time activated"),
    ("FIB", "Fibrinogen"),
    ("DDIMER", "Fibrin D-dimer"),
    // Hematology
    ("WBC", "Leukocytes"),
    ("RBC", "Erythrocytes"),
    ("HGB", "Hemoglobin"),
    ("HB", "Hemoglobin"),
    ("HCT", "Hematocrit"),
    ("MCV", "Erythrocyte mean corpuscular volume"),
    ("MCH", "Erythrocyte mean corpuscular hemoglobin"),
    ("MCHC", "Erythrocyte mean corpuscular hemoglobin concentration"),
    ("RDW", "Erythrocyte distribution width"),
    ("PLT", "Platelets"),
    ("MPV", "Platelet mean volume"),
    ("RETIC", "Reticulocytes"),
    ("ESR", "Erythrocyte sedimentation rate"),
    // Inflammation / immune
    ("CRP", "C reactive protein"),
    ("HSCRP", "C reactive protein high sensitivity"),
    ("PCT", "Procalcitonin"),
    ("ANA", "Antinuclear antibody"),
    ("RF", "Rheumatoid factor"),
    ("CCP", "Cyclic citrullinated peptide"),
    ("IGG", "Immunoglobulin G"),
    ("IGA", "Immunoglobulin A"),
    ("IGM", "Immunoglobulin M"),
    ("IGE", "Immunoglobulin E"),
    // Cardiac
    ("TNI", "Troponin I"),
    ("TNT", "Troponin T"),
    ("CK", "Creatine kinase"),
    ("CKMB", "Creatine kinase MB"),
    ("BNP", "Natriuretic peptide B"),
    ("MYO", "Myoglobin"),
    // Tumor markers
    ("PSA", "Prostate specific antigen"),
    ("CEA", "Carcinoembryonic antigen"),
    ("AFP", "Alpha-1-fetoprotein"),
    ("CA125", "Cancer antigen 125"),
    ("HCG", "Choriogonadotropin"),
    // Glucose / metabolism
    ("GLU", "Glucose"),
    ("FBS", "Glucose fasting"),
    ("OGTT", "Glucose tolerance"),
    ("INS", "Insulin"),
    ("CPEP", "C peptide"),
    ("LAC", "Lactate"),
    ("UA", "Urate"),
    // Vitamins / iron
    ("B12", "Cobalamin"),
    ("FOLATE", "Folate"),
    ("FERR", "Ferritin"),
    ("FE", "Iron"),
    ("TIBC", "Iron binding capacity"),
    // Blood gas
    ("ABG", "Blood gas arterial"),
    ("VBG", "Blood gas venous"),
    ("PO2", "Oxygen partial pressure"),
    ("PCO2", "Carbon dioxide partial pressure"),
    ("O2SAT", "Oxygen saturation"),
    // Infectious disease
    ("HIV", "Human immunodeficiency virus"),
    ("HBV", "Hepatitis B virus"),
    ("HBSAG", "Hepatitis B virus surface Ag"),
    ("HCV", "Hepatitis C virus"),
    ("HAV", "Hepatitis A virus"),
    ("EBV", "Epstein Barr virus"),
    ("CMV", "Cytomegalovirus"),
    ("COVID", "SARS coronavirus 2"),
    ("RSV", "Respiratory syncytial virus"),
    ("FLU", "Influenza virus"),
    // Endocrine
    ("PTH", "Parathyrin"),
    ("CORT", "Cortisol"),
    ("ACTH", "Corticotropin"),
    ("FSH", "Follitropin"),
    ("LH", "Lutropin"),
    ("PRL", "Prolactin"),
    ("TEST", "Testosterone"),
    ("ESTR", "Estradiol"),
    ("PROG", "Progesterone"),
];

fn lookup(token: &str) -> Option<&'static str> {
    let key = token.to_uppercase();
    DICTIONARY
        .iter()
        .find(|(abbr, _)| *abbr == key)
        .map(|(_, phrase)| *phrase)
}

/// Expands tokens within a multi-word query. Tokens that match a known
/// abbreviation get their expansion appended to the query (rather than
/// replacing — so original tokens still drive matches too). Non-alphanumeric
/// characters are treated as token separators.
pub fn expand(query: &str) -> String {
    let tokens: Vec<&str> = query
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .collect();
    if tokens.is_empty() {
        return query.trim().to_string();
    }
    let mut pieces: Vec<String> = vec![query.to_string()];
    for token in &tokens {
        if let Some(expansion) = lookup(token) {
            pieces.push(expansion.to_string());
        }
    }
    pieces.join(" ")
}
