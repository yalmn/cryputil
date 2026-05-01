use crate::core::error::{CalcError, CalcResult};
use crate::core::math::mod_pow;
use crate::core::trace::Trace;

// Input: n, d (privat), m (Nachricht < n)
// Calc:  RSA-Signatur s = m^d mod n
// Output: Trace mit Signatur s
pub fn sign(n: i128, d: i128, m: i128) -> CalcResult<Trace> {
    if m < 0 || m >= n {
        return Err(CalcError::Bereich("m muss in [0, n) liegen".into()));
    }
    let mut t = Trace::new("RSA-Signatur: Erzeugung");
    t.input("n", n);
    t.input("d (privat)", d);
    t.input("m (Nachricht)", m);

    let s1 = t.step("Signatur berechnen");
    let s = mod_pow(m, d, n)?;
    t.line(s1, format!("s = m^d mod n = {}^{} mod {} = {}", m, d, n, s));

    t.result("Signatur s", s);
    Ok(t)
}

// Input: n, e (öffentlich), m (Nachricht), s (Signatur)
// Calc:  Prüfen, ob s^e mod n = m
// Output: Trace mit Bewertung gültig/ungültig
pub fn verify(n: i128, e: i128, m: i128, s: i128) -> CalcResult<Trace> {
    if m < 0 || m >= n {
        return Err(CalcError::Bereich("m muss in [0, n) liegen".into()));
    }
    if s < 0 || s >= n {
        return Err(CalcError::Bereich("s muss in [0, n) liegen".into()));
    }
    let mut t = Trace::new("RSA-Signatur: Verifikation");
    t.input("n", n);
    t.input("e (öffentlich)", e);
    t.input("m (Nachricht)", m);
    t.input("s (Signatur)", s);

    let s1 = t.step("Wiederherstellung m' = s^e mod n");
    let m_check = mod_pow(s, e, n)?;
    t.line(
        s1,
        format!("m' = s^e mod n = {}^{} mod {} = {}", s, e, n, m_check),
    );

    let s2 = t.step("Vergleich m' = m");
    let valid = m_check == m;
    t.line(s2, format!("m' = {},  m = {}", m_check, m));
    if valid {
        t.line(s2, "m' = m ✓ – Signatur ist gültig.");
    } else {
        t.line(s2, "m' ≠ m – Signatur ist ungültig.");
    }

    t.result("m' = s^e mod n", m_check);
    t.result("Signatur gültig", valid);
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_verify_roundtrip() {
        let n = 143i128;
        let e = 7i128;
        let d = 103i128;
        let m = 42i128;
        let sig = sign(n, d, m).unwrap();
        let s: i128 = sig.result[0].1.parse().unwrap();
        let v = verify(n, e, m, s).unwrap();
        let valid = v
            .result
            .iter()
            .find(|(k, _)| k.starts_with("Signatur gültig"))
            .unwrap()
            .1
            .clone();
        assert_eq!(valid, "true");
    }

    #[test]
    fn test_invalid_signature() {
        let v = verify(143, 7, 42, 12).unwrap();
        let valid = v
            .result
            .iter()
            .find(|(k, _)| k.starts_with("Signatur gültig"))
            .unwrap()
            .1
            .clone();
        assert_eq!(valid, "false");
    }
}
