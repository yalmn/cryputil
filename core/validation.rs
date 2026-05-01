#![allow(dead_code)]

use crate::core::error::{CalcError, CalcResult};

// Input: Roh-String
// Calc:  Trim und Vorzeichen-tolerante Konvertierung in i128
// Output: i128 oder Fehler
pub fn parse_int(s: &str) -> CalcResult<i128> {
    let t = s.trim();
    t.parse::<i128>()
        .map_err(|_| CalcError::UngueltigeEingabe(format!("\"{}\" ist keine ganze Zahl", t)))
}

// Input: Roh-String
// Calc:  Konvertierung in u128 (>= 0)
// Output: u128 oder Fehler
pub fn parse_uint(s: &str) -> CalcResult<u128> {
    let v = parse_int(s)?;
    if v < 0 {
        return Err(CalcError::UngueltigeEingabe(format!(
            "\"{}\" muss nicht negativ sein",
            s.trim()
        )));
    }
    Ok(v as u128)
}

// Input: Wert, Untergrenze, Obergrenze (inkl.)
// Calc:  Bereichsprüfung
// Output: Wert oder Fehler
pub fn ensure_in_range(value: i128, lo: i128, hi: i128) -> CalcResult<i128> {
    if value < lo || value > hi {
        return Err(CalcError::Bereich(format!(
            "{} liegt nicht in [{}, {}]",
            value, lo, hi
        )));
    }
    Ok(value)
}

// Input: Wert
// Calc:  Prüfung > 0
// Output: Wert oder Fehler
pub fn ensure_positive(value: i128) -> CalcResult<i128> {
    if value <= 0 {
        return Err(CalcError::Bereich(format!("{} muss > 0 sein", value)));
    }
    Ok(value)
}
