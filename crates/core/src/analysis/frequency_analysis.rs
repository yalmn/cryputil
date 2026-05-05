use crate::core::error::{CalcError, CalcResult};
use crate::core::trace::{Table, Trace};

// Referenz: typische Buchstabenhäufigkeiten Deutsch (Prozent), aufsteigend nach Häufigkeit absteigend.
const FREQ_DE: [(char, f64); 26] = [
    ('E', 17.40),
    ('N', 9.78),
    ('I', 7.55),
    ('S', 7.27),
    ('R', 7.00),
    ('A', 6.51),
    ('T', 6.15),
    ('D', 5.08),
    ('H', 4.76),
    ('U', 4.35),
    ('L', 3.44),
    ('C', 3.06),
    ('G', 3.01),
    ('M', 2.53),
    ('O', 2.51),
    ('B', 1.89),
    ('W', 1.89),
    ('F', 1.66),
    ('K', 1.21),
    ('Z', 1.13),
    ('P', 0.79),
    ('V', 0.67),
    ('J', 0.27),
    ('Y', 0.04),
    ('X', 0.03),
    ('Q', 0.02),
];

// Referenz: typische Buchstabenhäufigkeiten Englisch (Prozent), absteigend.
const FREQ_EN: [(char, f64); 26] = [
    ('E', 12.70),
    ('T', 9.06),
    ('A', 8.17),
    ('O', 7.51),
    ('I', 6.97),
    ('N', 6.75),
    ('S', 6.33),
    ('H', 6.09),
    ('R', 5.99),
    ('D', 4.25),
    ('L', 4.03),
    ('C', 2.78),
    ('U', 2.76),
    ('M', 2.41),
    ('W', 2.36),
    ('F', 2.23),
    ('G', 2.02),
    ('Y', 1.97),
    ('P', 1.93),
    ('B', 1.49),
    ('V', 0.98),
    ('K', 0.77),
    ('J', 0.15),
    ('X', 0.15),
    ('Q', 0.10),
    ('Z', 0.07),
];

// Input: Sprachschlüssel ("de" / "en" und gängige Synonyme)
// Calc:  passende Referenzhäufigkeitstabelle wählen
// Output: (Anzeigelabel, Tabellenreferenz)
fn pick_reference(language: &str) -> CalcResult<(&'static str, &'static [(char, f64); 26])> {
    match language.trim().to_lowercase().as_str() {
        "de" | "deu" | "deutsch" | "german" | "" => Ok(("Deutsch", &FREQ_DE)),
        "en" | "eng" | "englisch" | "english" => Ok(("Englisch", &FREQ_EN)),
        other => Err(CalcError::UngueltigeEingabe(format!(
            "Sprache '{}' nicht unterstützt (de oder en)",
            other
        ))),
    }
}

// Input: Geheimtext und Sprache ("de"/"en")
// Calc:  Buchstaben zählen, Referenzhäufigkeiten auflisten und eine Vorschlagszuordnung
//        nach Rang (häufigster Chiffrebuchstabe → häufigster Referenzbuchstabe) bilden,
//        anschließend Roh-Übersetzung anwenden
// Output: Trace mit drei Tabellen (Chiffretext, Referenz, Vorschlag) und Roh-Klartext
pub fn analyze(text: &str, language: &str) -> CalcResult<Trace> {
    let (lang_label, reference) = pick_reference(language)?;

    let mut counts = [0u32; 26];
    let mut total: u32 = 0;
    for ch in text.chars() {
        if ch.is_ascii_alphabetic() {
            counts[(ch.to_ascii_uppercase() as u8 - b'A') as usize] += 1;
            total += 1;
        }
    }
    if total == 0 {
        return Err(CalcError::UngueltigeEingabe(
            "Text enthält keine Buchstaben".into(),
        ));
    }

    let mut t = Trace::new("Häufigkeitsanalyse");
    t.input("Geheimtext", text);
    t.input("Sprache", lang_label);
    t.input("Buchstaben gesamt", total);

    let s1 = t.step("Buchstabenhäufigkeit im Geheimtext (absteigend)");
    let mut cipher_freq: Vec<(char, u32, f64)> = (0u8..26)
        .map(|i| {
            let c = (b'A' + i) as char;
            let n = counts[i as usize];
            let pct = n as f64 * 100.0 / total as f64;
            (c, n, pct)
        })
        .collect();
    cipher_freq.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let cipher_rows: Vec<Vec<String>> = cipher_freq
        .iter()
        .filter(|(_, n, _)| *n > 0)
        .map(|(c, n, p)| vec![c.to_string(), n.to_string(), format!("{:.2}", p)])
        .collect();
    t.table(
        s1,
        Table {
            headers: vec!["Buchstabe".into(), "Anzahl".into(), "Anteil %".into()],
            rows: cipher_rows,
        },
    );

    let s2 = t.step(&format!("Referenzhäufigkeiten ({})", lang_label));
    let ref_rows: Vec<Vec<String>> = reference
        .iter()
        .map(|(c, p)| vec![c.to_string(), format!("{:.2}", p)])
        .collect();
    t.table(
        s2,
        Table {
            headers: vec!["Buchstabe".into(), "Anteil %".into()],
            rows: ref_rows,
        },
    );

    let s3 = t.step("Vorschlag: häufigster Chiffre- → häufigster Referenzbuchstabe");
    let mut mapping: [Option<char>; 26] = [None; 26];
    let mut mapping_rows: Vec<Vec<String>> = Vec::new();
    let cipher_present: Vec<&(char, u32, f64)> =
        cipher_freq.iter().filter(|(_, n, _)| *n > 0).collect();
    for (rank, (c, _n, pct)) in cipher_present.iter().enumerate() {
        if rank >= reference.len() {
            break;
        }
        let (ref_c, ref_p) = reference[rank];
        mapping[(*c as u8 - b'A') as usize] = Some(ref_c);
        mapping_rows.push(vec![
            (rank + 1).to_string(),
            c.to_string(),
            format!("{:.2}", pct),
            ref_c.to_string(),
            format!("{:.2}", ref_p),
        ]);
    }
    t.table(
        s3,
        Table {
            headers: vec![
                "Rang".into(),
                "Chiffrat".into(),
                "% (Text)".into(),
                "Klartext".into(),
                "% (Sprache)".into(),
            ],
            rows: mapping_rows,
        },
    );

    let s4 = t.step("Roh-Übersetzung mit Vorschlag (unbekannte Buchstaben als '_')");
    let mut preview = String::with_capacity(text.len());
    for ch in text.chars() {
        if ch.is_ascii_alphabetic() {
            let was_lower = ch.is_ascii_lowercase();
            let idx = (ch.to_ascii_uppercase() as u8 - b'A') as usize;
            match mapping[idx] {
                Some(p) => preview.push(if was_lower { p.to_ascii_lowercase() } else { p }),
                None => preview.push('_'),
            }
        } else {
            preview.push(ch);
        }
    }
    t.line(s4, preview.clone());
    t.result("Vorschlag (Roh-Übersetzung)", preview);

    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skewed_german_suggests_e() {
        // dominanter Chiffrebuchstabe X → soll auf häufigsten DE-Buchstaben (E) abgebildet werden.
        let t = analyze("XXXXXXXXXY", "de").unwrap();
        let result = &t.result[0].1;
        assert!(result.starts_with("EEEEEEEEE"));
    }

    #[test]
    fn test_unknown_language() {
        assert!(analyze("ABC", "xx").is_err());
    }

    #[test]
    fn test_empty_letters() {
        assert!(analyze("123!@#", "de").is_err());
    }

    #[test]
    fn test_preserves_punctuation() {
        let t = analyze("AAA, BBB", "en").unwrap();
        assert!(t.result[0].1.contains(','));
    }
}
