use crate::core::error::CalcResult;
use crate::core::trace::{Table, Trace};

// Input: beliebige Verschiebung (auch negativ oder > 26)
// Calc:  in den Bereich [0, 26) abbilden
// Output: normalisierte Verschiebung k
fn normalize_shift(shift: i128) -> u8 {
    shift.rem_euclid(26) as u8
}

// Input: Verschiebung k ∈ [0, 26)
// Calc:  Mapping-Array bauen, Index 0 = Bild von A, ..., 25 = Bild von Z
// Output: Array [chiffrebuchstabe; 26]
fn build_mapping(k: u8) -> [char; 26] {
    std::array::from_fn(|i| (b'A' + ((i as u8 + k) % 26)) as char)
}

// Input: Mapping-Array
// Calc:  Alphabet-Zeile "A B C ... Z"
// Output: durch Leerzeichen getrennter String
fn alphabet_line(arr: &[char; 26]) -> String {
    let mut s = String::with_capacity(26 * 2);
    for (i, c) in arr.iter().enumerate() {
        if i > 0 {
            s.push(' ');
        }
        s.push(*c);
    }
    s
}

// Input: Text und Mapping-Array
// Calc:  zeichenweise Anwendung; Groß-/Kleinschreibung und Nicht-Buchstaben bleiben erhalten
// Output: transformierter String
fn apply(text: &str, mapping: &[char; 26]) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        if ch.is_ascii_alphabetic() {
            let was_lower = ch.is_ascii_lowercase();
            let idx = (ch.to_ascii_uppercase() as u8 - b'A') as usize;
            let mapped = mapping[idx];
            out.push(if was_lower {
                mapped.to_ascii_lowercase()
            } else {
                mapped
            });
        } else {
            out.push(ch);
        }
    }
    out
}

// Input: Verschiebung k
// Calc:  Tabelle mit Klartext, modularer Formel und Chiffrat-Buchstabe für jede Position
// Output: Table mit 26 Zeilen
fn shift_table(k: u8) -> Table {
    let mut rows: Vec<Vec<String>> = Vec::with_capacity(26);
    for i in 0..26u8 {
        let plain = (b'A' + i) as char;
        let cipher_idx = (i + k) % 26;
        let cipher = (b'A' + cipher_idx) as char;
        rows.push(vec![
            plain.to_string(),
            i.to_string(),
            format!("({} + {}) mod 26 = {}", i, k, cipher_idx),
            cipher.to_string(),
        ]);
    }
    Table {
        headers: vec![
            "Klartext".into(),
            "Index".into(),
            "Berechnung".into(),
            "Chiffrat".into(),
        ],
        rows,
    }
}

// Input: Klartext und Verschiebung
// Calc:  Caesar-Verschlüsselung c_i = (p_i + k) mod 26
// Output: Trace mit Mapping-Tabelle und Resultat
pub fn encrypt(text: &str, shift: i128) -> CalcResult<Trace> {
    let k = normalize_shift(shift);
    let mapping = build_mapping(k);
    let plain: [char; 26] = std::array::from_fn(|i| (b'A' + i as u8) as char);

    let mut t = Trace::new("Caesar-Verschlüsselung");
    t.input("Klartext", text);
    t.input("Verschiebung k", shift);

    let s1 = t.step("Verschiebung normalisieren");
    t.line(s1, format!("k = {} mod 26 = {}", shift, k));

    let s2 = t.step("Mapping (Klartext → Chiffrat)");
    t.line(s2, format!("Klartext: {}", alphabet_line(&plain)));
    t.line(s2, format!("Chiffrat: {}", alphabet_line(&mapping)));
    t.table(s2, shift_table(k));

    let s3 = t.step("Zeichenweise Substitution");
    let out = apply(text, &mapping);
    t.line(s3, format!("Resultat: {}", out));

    t.result("Chiffrat", out);
    Ok(t)
}

// Input: Chiffrat und Verschiebung
// Calc:  Caesar-Entschlüsselung p_i = (c_i − k) mod 26
// Output: Trace mit inversem Mapping und Resultat
pub fn decrypt(text: &str, shift: i128) -> CalcResult<Trace> {
    let k = normalize_shift(shift);
    let inv_k = normalize_shift(-(k as i128));
    let mapping = build_mapping(inv_k);
    let plain: [char; 26] = std::array::from_fn(|i| (b'A' + i as u8) as char);

    let mut t = Trace::new("Caesar-Entschlüsselung");
    t.input("Chiffrat", text);
    t.input("Verschiebung k", shift);

    let s1 = t.step("Verschiebung normalisieren");
    t.line(s1, format!("k = {} mod 26 = {}", shift, k));
    t.line(s1, format!("Rück-Verschiebung: −k mod 26 = {}", inv_k));

    let s2 = t.step("Inverses Mapping (Chiffrat → Klartext)");
    t.line(s2, format!("Chiffrat: {}", alphabet_line(&plain)));
    t.line(s2, format!("Klartext: {}", alphabet_line(&mapping)));
    t.table(s2, shift_table(inv_k));

    let s3 = t.step("Zeichenweise Rück-Substitution");
    let out = apply(text, &mapping);
    t.line(s3, format!("Resultat: {}", out));

    t.result("Klartext", out);
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_shift_3() {
        // klassisches Beispiel: HELLO mit k=3 → KHOOR
        let enc = encrypt("HELLO", 3).unwrap();
        assert_eq!(enc.result[0].1, "KHOOR");
    }

    #[test]
    fn test_roundtrip() {
        let enc = encrypt("Die Sonne scheint!", 7).unwrap();
        let c = enc.result[0].1.clone();
        let dec = decrypt(&c, 7).unwrap();
        assert_eq!(dec.result[0].1, "Die Sonne scheint!");
    }

    #[test]
    fn test_negative_shift() {
        // shift = -23 ist gleichbedeutend mit shift = 3
        let a = encrypt("HALLO", 3).unwrap();
        let b = encrypt("HALLO", -23).unwrap();
        assert_eq!(a.result[0].1, b.result[0].1);
    }

    #[test]
    fn test_large_shift() {
        // shift = 29 ist gleichbedeutend mit shift = 3
        let a = encrypt("HALLO", 3).unwrap();
        let b = encrypt("HALLO", 29).unwrap();
        assert_eq!(a.result[0].1, b.result[0].1);
    }

    #[test]
    fn test_preserves_case_and_punctuation() {
        let enc = encrypt("Hi, World!", 5).unwrap();
        let s = &enc.result[0].1;
        let chars: Vec<char> = s.chars().collect();
        assert!(chars[0].is_ascii_uppercase());
        assert!(chars[1].is_ascii_lowercase());
        assert!(s.contains(','));
        assert!(s.contains('!'));
    }

    #[test]
    fn test_zero_shift_identity() {
        let enc = encrypt("ABC xyz!", 0).unwrap();
        assert_eq!(enc.result[0].1, "ABC xyz!");
    }
}
