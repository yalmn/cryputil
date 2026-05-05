use crate::core::error::{CalcError, CalcResult};
use crate::core::trace::Trace;

// Input: Schlüssel als String mit (mind.) 26 Buchstaben, Reihenfolge entspricht A..Z
// Calc:  validieren, normalisieren auf Großbuchstaben, Permutation prüfen
// Output: Array [chiffrebuchstabe; 26], Index 0 = Bild von A, ..., 25 = Bild von Z
fn validate_key(key: &str) -> CalcResult<[char; 26]> {
    let upper: String = key
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_uppercase())
        .collect();
    if upper.chars().count() != 26 {
        return Err(CalcError::UngueltigeEingabe(format!(
            "Schlüssel muss 26 Buchstaben enthalten (geliefert: {})",
            upper.chars().count()
        )));
    }
    let mut seen = [false; 26];
    let mut arr = ['A'; 26];
    for (i, c) in upper.chars().enumerate() {
        let idx = (c as u8 - b'A') as usize;
        if seen[idx] {
            return Err(CalcError::UngueltigeEingabe(format!(
                "Buchstabe '{}' kommt mehrfach im Schlüssel vor",
                c
            )));
        }
        seen[idx] = true;
        arr[i] = c;
    }
    Ok(arr)
}

// Input: Mapping-Array
// Calc:  Klartext-Alphabet "A B C ... Z"
// Output: String mit einzelnen Buchstaben, durch Leerzeichen getrennt
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

// Input: Klartext und Schlüssel
// Calc:  monoalphabetische Substitution gemäß Schlüssel
// Output: Trace mit Mapping und Resultat
pub fn encrypt(text: &str, key: &str) -> CalcResult<Trace> {
    let cipher = validate_key(key)?;
    let plain: [char; 26] = std::array::from_fn(|i| (b'A' + i as u8) as char);

    let mut t = Trace::new("Substitutions-Verschlüsselung");
    t.input("Klartext", text);
    t.input("Schlüssel", key);

    let s1 = t.step("Mapping (Klartext → Chiffrat)");
    t.line(s1, format!("Klartext: {}", alphabet_line(&plain)));
    t.line(s1, format!("Chiffrat: {}", alphabet_line(&cipher)));

    let s2 = t.step("Zeichenweise Substitution");
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        if ch.is_ascii_alphabetic() {
            let was_lower = ch.is_ascii_lowercase();
            let idx = (ch.to_ascii_uppercase() as u8 - b'A') as usize;
            let mapped = cipher[idx];
            out.push(if was_lower {
                mapped.to_ascii_lowercase()
            } else {
                mapped
            });
        } else {
            out.push(ch);
        }
    }
    t.line(s2, format!("Resultat: {}", out));
    t.result("Chiffrat", out);
    Ok(t)
}

// Input: Chiffrat und Schlüssel
// Calc:  inverse Substitution
// Output: Trace mit inversem Mapping und Resultat
pub fn decrypt(text: &str, key: &str) -> CalcResult<Trace> {
    let cipher = validate_key(key)?;
    let plain: [char; 26] = std::array::from_fn(|i| (b'A' + i as u8) as char);
    let mut inv = ['A'; 26];
    for i in 0..26 {
        let idx = (cipher[i] as u8 - b'A') as usize;
        inv[idx] = plain[i];
    }

    let mut t = Trace::new("Substitutions-Entschlüsselung");
    t.input("Chiffrat", text);
    t.input("Schlüssel", key);

    let s1 = t.step("Inverses Mapping (Chiffrat → Klartext)");
    t.line(s1, format!("Chiffrat: {}", alphabet_line(&plain)));
    t.line(s1, format!("Klartext: {}", alphabet_line(&inv)));

    let s2 = t.step("Zeichenweise Rück-Substitution");
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        if ch.is_ascii_alphabetic() {
            let was_lower = ch.is_ascii_lowercase();
            let idx = (ch.to_ascii_uppercase() as u8 - b'A') as usize;
            let mapped = inv[idx];
            out.push(if was_lower {
                mapped.to_ascii_lowercase()
            } else {
                mapped
            });
        } else {
            out.push(ch);
        }
    }
    t.line(s2, format!("Resultat: {}", out));
    t.result("Klartext", out);
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: &str = "QWERTZUIOPASDFGHJKLYXCVBNM";

    #[test]
    fn test_roundtrip() {
        let enc = encrypt("HALLO WELT", KEY).unwrap();
        let c = enc.result[0].1.clone();
        let dec = decrypt(&c, KEY).unwrap();
        assert_eq!(dec.result[0].1, "HALLO WELT");
    }

    #[test]
    fn test_preserves_non_letters() {
        let enc = encrypt("AB, CD!", KEY).unwrap();
        assert!(enc.result[0].1.contains(','));
        assert!(enc.result[0].1.contains('!'));
    }

    #[test]
    fn test_rejects_short_key() {
        assert!(encrypt("ABC", "ABC").is_err());
    }

    #[test]
    fn test_rejects_duplicate_letters() {
        assert!(encrypt("ABC", "AAERTZUIOPSDFGHJKLYXCVBNMW").is_err());
    }

    #[test]
    fn test_case_preserved() {
        let enc = encrypt("Ab", KEY).unwrap();
        let result = &enc.result[0].1;
        let chars: Vec<char> = result.chars().collect();
        assert!(chars[0].is_ascii_uppercase());
        assert!(chars[1].is_ascii_lowercase());
    }
}
