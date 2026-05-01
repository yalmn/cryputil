use crate::core::error::{CalcError, CalcResult};
use crate::core::trace::{Table, Trace};

// Input: Geheimtext einer Spaltentransposition
// Calc:  alle nicht-trivialen Teiler der Textlänge bestimmen und je Spaltenanzahl
//        die Matrix (Spalten zuerst befüllt, zeilenweise gelesen) und den daraus
//        rekonstruierten Klartext-Kandidaten erzeugen
// Output: Trace mit Matrix-Tabellen und Kandidaten-Strings je Spaltenanzahl
pub fn decrypt(text: &str) -> CalcResult<Trace> {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    if len < 2 {
        return Err(CalcError::Bereich(
            "Geheimtext muss mindestens 2 Zeichen enthalten".into(),
        ));
    }

    let mut t = Trace::new("Spaltentransposition: Varianten aller Spaltenzahlen");
    t.input("Geheimtext", text);
    t.input("Länge", len);

    let divisors: Vec<usize> = (2..len).filter(|n| len % n == 0).collect();

    let s0 = t.step("Teiler der Textlänge bestimmen");
    if divisors.is_empty() {
        t.line(
            s0,
            format!(
                "Die Zeichenlänge {} ist eine Primzahl – keine Spaltentransformation möglich.",
                len
            ),
        );
        t.result("Hinweis", "Länge ist prim, keine Variante darstellbar");
        return Ok(t);
    }
    let teiler_str: Vec<String> = divisors.iter().map(|d| d.to_string()).collect();
    t.line(
        s0,
        format!(
            "Nicht-triviale Teiler (ohne 1 und {}): {}",
            len,
            teiler_str.join(", ")
        ),
    );

    for col in &divisors {
        let col = *col;
        let row = len / col;
        let s = t.step(&format!("Variante: {} Spalten, {} Zeilen", col, row));

        let headers: Vec<String> = (1..=col).map(|c| format!("S{}", c)).collect();
        let mut rows: Vec<Vec<String>> = Vec::with_capacity(row);
        let mut candidate = String::with_capacity(len);
        for r in 0..row {
            let mut row_cells: Vec<String> = Vec::with_capacity(col);
            for c in 0..col {
                let ch = chars[c * row + r];
                row_cells.push(ch.to_string());
                candidate.push(ch);
            }
            rows.push(row_cells);
        }
        t.table(s, Table { headers, rows });
        t.line(s, format!("Kandidat: {}", candidate));
        t.result(&format!("Kandidat ({} Spalten)", col), candidate);
    }

    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_length() {
        let t = decrypt("HELLO").unwrap();
        assert!(t.result.iter().any(|(k, _)| k == "Hinweis"));
    }

    #[test]
    fn test_roundtrip() {
        // Klartext "WIKIPEDIA" (9 Zeichen), bei col=3, row=3 spaltenweise eingetragen:
        // Klartext zeilenweise:
        //   W I K
        //   I P E
        //   D I A
        // Spaltenweise gelesen ergibt Geheimtext: W I D I P I K E A
        let cipher = "WIDIPIKEA";
        let t = decrypt(cipher).unwrap();
        let cand = t
            .result
            .iter()
            .find(|(k, _)| k == "Kandidat (3 Spalten)")
            .unwrap()
            .1
            .clone();
        assert_eq!(cand, "WIKIPEDIA");
    }

    #[test]
    fn test_multiple_divisors() {
        let t = decrypt("ABCDEFGH").unwrap();
        // Teiler von 8: 2, 4 → zwei Varianten
        let count = t
            .result
            .iter()
            .filter(|(k, _)| k.starts_with("Kandidat"))
            .count();
        assert_eq!(count, 2);
    }
}
